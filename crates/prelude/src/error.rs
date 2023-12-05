#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err)
    };
}

pub trait MakeError<T, E> {
    fn make_error(self, error: E) -> Result<T, E>;
}

impl<T, U, E> MakeError<T, E> for Result<T, U> {
    fn make_error(self, error: E) -> Result<T, E> {
        match self {
            Ok(n) => Ok(n),
            Err(_) => Err(error),
        }
    }
}

impl<T, E> MakeError<T, E> for Option<T> {
    fn make_error(self, error: E) -> Result<T, E> {
        match self {
            Some(n) => Ok(n),
            None => Err(error),
        }
    }
}
