pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    XShell(xshell::Error),
}

macro_rules! impl_from_error {
    ($source:ty,$variant:ident) => {
        impl From<$source> for Error {
            fn from(value: $source) -> Self {
                Self::$variant(value)
            }
        }
    };
}

impl_from_error!(std::io::Error, Io);
impl_from_error!(xshell::Error, XShell);
