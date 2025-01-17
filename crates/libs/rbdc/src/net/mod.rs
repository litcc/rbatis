mod socket;

pub use socket::Socket;

#[cfg(any(feature = "tls-rustls", feature = "tls-native-tls"))]
mod tls;

#[cfg(any(feature = "tls-rustls", feature = "tls-native-tls"))]
pub use tls::CertificateInput;
#[cfg(any(feature = "tls-rustls", feature = "tls-native-tls"))]
pub use tls::MaybeTlsStream;

type PollReadBuf<'a> = crate::rt::ReadBuf<'a>;

type PollReadOut = ();
