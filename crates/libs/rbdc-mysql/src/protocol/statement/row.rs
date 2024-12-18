use bytes::Buf;
use bytes::Bytes;
use rbdc::err_protocol;
use rbdc::io::BufExt;
use rbdc::io::Decode;
use rbdc::Error;

use crate::io::MySqlBufExt;
use crate::protocol::text::ColumnType;
use crate::protocol::Row;
use crate::result_set::MySqlColumn;

// https://dev.mysql.com/doc/internals/en/binary-protocol-resultset-row.html#packet-ProtocolBinary::ResultsetRow
// https://dev.mysql.com/doc/internals/en/binary-protocol-value.html

#[derive(Debug)]
pub struct BinaryRow(pub Row);

impl<'de> Decode<'de, &'de [MySqlColumn]> for BinaryRow {
    fn decode_with(
        mut buf: Bytes,
        columns: &'de [MySqlColumn],
    ) -> Result<Self, Error> {
        let header = buf.get_u8();
        if header != 0 {
            return Err(err_protocol!(
                "exepcted 0x00 (ROW) but found 0x{:02x}",
                header
            ));
        }

        let storage = buf.clone();
        let offset = buf.len();

        let null_bitmap_len = (columns.len() + 9) / 8;
        let null_bitmap = buf.get_bytes(null_bitmap_len);

        let mut values = Vec::with_capacity(columns.len());

        for (column_idx, column) in columns.iter().enumerate() {
            // NOTE: the column index starts at the 3rd bit
            let column_null_idx = column_idx + 2;
            let is_null = null_bitmap[column_null_idx / 8] &
                (1 << (column_null_idx % 8) as u8) !=
                0;

            if is_null {
                values.push(None);
                continue;
            }

            // NOTE: MySQL will never generate NULL types for non-NULL values
            let type_info = &column.type_info;

            let size: usize = match type_info.r#type {
                ColumnType::String |
                ColumnType::VarChar |
                ColumnType::VarString |
                ColumnType::Enum |
                ColumnType::Set |
                ColumnType::LongBlob |
                ColumnType::MediumBlob |
                ColumnType::Blob |
                ColumnType::TinyBlob |
                ColumnType::Geometry |
                ColumnType::Bit |
                ColumnType::Decimal |
                ColumnType::Json |
                ColumnType::NewDecimal => buf.get_uint_lenenc() as usize,

                ColumnType::LongLong => 8,
                ColumnType::Long | ColumnType::Int24 => 4,
                ColumnType::Short | ColumnType::Year => 2,
                ColumnType::Tiny => 1,
                ColumnType::Float => 4,
                ColumnType::Double => 8,

                ColumnType::Time |
                ColumnType::Timestamp |
                ColumnType::Date |
                ColumnType::Datetime => {
                    // The size of this type is important for decoding
                    buf[0] as usize + 1
                }

                // NOTE: MySQL will never generate NULL types for non-NULL values
                ColumnType::Null => unreachable!(),
            };

            let offset = offset - buf.len();

            values.push(Some(offset..(offset + size)));

            buf.advance(size);
        }
        Ok(BinaryRow(Row::from((values, storage.to_vec()))))
    }
}
