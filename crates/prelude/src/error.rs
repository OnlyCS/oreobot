#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
}

#[macro_export]
macro_rules! bail_assert {
    ($cond:expr, $err:expr) => {
        if !$cond {
            bail!($err)
        }
    };
}

#[macro_export]
macro_rules! bail_assert_eq {
    ($left:expr, $right:expr, $err:expr) => {
        bail_assert!($left == $right, $err)
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
