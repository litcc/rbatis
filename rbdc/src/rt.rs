use std::future::Future;

#[cfg(feature = "tls-native-tls")]
pub use native_tls;
pub use tokio::{
    self, fs,
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
    net::TcpStream,
    runtime::Handle,
    task::{spawn, yield_now},
    time::{sleep, timeout},
};

pub fn block_on<T, R>(task: T) -> R
where
    T: Future<Output = R> + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::task::block_in_place(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio block_on fail")
            .block_on(task)
    })
}

//unix
#[cfg(unix)]
pub use tokio::net::UnixStream;
#[cfg(feature = "tls-native-tls")]
pub use tokio_native_tls::{TlsConnector, TlsStream};
#[cfg(feature = "tls-rustls")]
pub use tokio_rustls::{client::TlsStream, TlsConnector};
