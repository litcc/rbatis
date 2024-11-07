use std::future::Future;

#[cfg(feature = "tls-native-tls")]
pub use native_tls;
pub use tokio::fs;
pub use tokio::io::AsyncRead;
pub use tokio::io::AsyncReadExt;
pub use tokio::io::AsyncWrite;
pub use tokio::io::AsyncWriteExt;
pub use tokio::io::ReadBuf;
pub use tokio::net::TcpStream;
pub use tokio::runtime::Handle;
pub use tokio::task::spawn;
pub use tokio::task::yield_now;
pub use tokio::time::sleep;
pub use tokio::time::timeout;
pub use tokio::{self};

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
pub use tokio_native_tls::TlsConnector;
#[cfg(feature = "tls-native-tls")]
pub use tokio_native_tls::TlsStream;
#[cfg(feature = "tls-rustls")]
pub use tokio_rustls::client::TlsStream;
#[cfg(feature = "tls-rustls")]
pub use tokio_rustls::TlsConnector;
