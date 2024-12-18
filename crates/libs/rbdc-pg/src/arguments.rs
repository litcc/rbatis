use std::fmt::Write;
use std::fmt::{self};
use std::ops::Deref;
use std::ops::DerefMut;

use rbdc::error::Error;
use rbdc::ext::ustr::UStr;
use rbs::Value;

use crate::connection::PgConnection;
use crate::type_info::PgTypeInfo;
use crate::types::encode::Encode;
use crate::types::encode::IsNull;
use crate::types::TypeInfo;

// TODO: buf.patch(|| ...) is a poor name, can we think of a better name? Maybe
// `buf.lazy(||)` ? TODO: Extend the patch system to support dynamic lengths
//       Considerations:
//          - The prefixed-len offset needs to be back-tracked and updated
//          - message::Bind needs to take a &PgArguments and use a `write` method
//            instead of referencing a buffer directly
//          - The basic idea is that we write bytes for the buffer until we get
//            somewhere that has a patch, we then apply the patch which should write
//            to &mut Vec<u8>, backtrack and update the prefixed-len, then write
//            until the next patch offset

pub type Patch = (
    usize, // offset
    usize, // argument index
    Box<dyn Fn(&mut [u8], &PgTypeInfo) + 'static + Send + Sync>,
);

#[derive(Default)]
pub struct PgArgumentBuffer {
    buffer: Vec<u8>,

    // Number of arguments
    count: usize,

    // Whenever an `Encode` impl needs to defer some work until after we resolve
    // parameter types it can use `patch`.
    //
    // This currently is only setup to be useful if there is a *fixed-size* slot
    // that needs to be tweaked from the input type. However, that's the only
    // use case we currently have.
    patches: Vec<Patch>,

    // Whenever an `Encode` impl encounters a `PgTypeInfo` object that does not have
    // an OID It pushes a "hole" that must be patched later.
    //
    // The hole is a `usize` offset into the buffer with the type name that should
    // be resolved This is done for Records and Arrays as the OID is needed well
    // before we are in an async function and can just ask postgres.
    type_holes: Vec<(usize, UStr)>, // Vec<{ offset, type_name }>
}

/// Implementation of [`Arguments`] for PostgreSQL.
#[derive(Default)]
pub struct PgArguments {
    // Types of each bind parameter
    pub(crate) types: Vec<PgTypeInfo>,

    // Buffer of encoded bind parameters
    pub(crate) buffer: PgArgumentBuffer,
}

impl PgArguments {
    pub fn add(&mut self, value: Value) -> Result<(), Error> {
        // encode the value into our buffer
        let type_info = self.buffer.encode(value)?;
        self.types.push(type_info);
        // increment the number of arguments we are tracking
        self.buffer.count += 1;
        Ok(())
    }

    //Apply patches
    //This should only go out and ask postgres if we have not seen the type name yet
    pub(crate) async fn apply_patches(
        &mut self,
        conn: &mut PgConnection,
        parameters: &[PgTypeInfo],
    ) -> Result<(), Error> {
        let PgArgumentBuffer { ref patches, ref type_holes, ref mut buffer, .. } =
            self.buffer;

        for (offset, ty, callback) in patches {
            let buf = &mut buffer[*offset..];
            let ty = &parameters[*ty];
            callback(buf, ty);
        }

        for (offset, name) in type_holes {
            let oid = conn.fetch_type_id_by_name(name).await?;
            buffer[*offset..(*offset + 4)].copy_from_slice(&oid.0.to_be_bytes());
        }

        Ok(())
    }
}

impl PgArguments {
    pub fn reserve(&mut self, additional: usize, size: usize) {
        self.types.reserve(additional);
        self.buffer.reserve(size);
    }

    pub fn format_placeholder<W: Write>(&self, writer: &mut W) -> fmt::Result {
        write!(writer, "${}", self.buffer.count)
    }
}

impl PgArgumentBuffer {
    pub fn encode(&mut self, value: Value) -> Result<PgTypeInfo, Error> {
        // reserve space to write the prefixed length of the value
        let offset = self.len();
        self.extend(&[0; 4]);

        let info = value.type_info();
        // encode the value into our buffer
        let is_null = value.encode(self)?;
        let len = if let IsNull::No = is_null {
            (self.len() - offset - 4) as i32
        } else {
            // Write a -1 to indicate NULL
            // NOTE: It is illegal for [encode] to write any data
            debug_assert_eq!(self.len(), offset + 4);
            -1_i32
        };

        // write the len to the beginning of the value
        self[offset..(offset + 4)].copy_from_slice(&len.to_be_bytes());
        Ok(info)
    }

    // Adds a callback to be invoked later when we know the parameter type
    #[allow(dead_code)]
    pub(crate) fn patch<F>(&mut self, callback: F)
    where
        F: Fn(&mut [u8], &PgTypeInfo) + 'static + Send + Sync,
    {
        let offset = self.len();
        let index = self.count;

        self.patches.push((offset, index, Box::new(callback)));
    }

    // Extends the inner buffer by enough space to have an OID
    // Remembers where the OID goes and type name for the OID
    pub(crate) fn patch_type_by_name(&mut self, type_name: &UStr) {
        let offset = self.len();

        self.extend_from_slice(&0_u32.to_be_bytes());
        self.type_holes.push((offset, type_name.clone()));
    }
}

impl Deref for PgArgumentBuffer {
    type Target = Vec<u8>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for PgArgumentBuffer {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
