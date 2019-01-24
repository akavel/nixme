use failure;

pub type Result<T> = std::result::Result<T, failure::Error>;

#[macro_export]
macro_rules! raise {
    ($e:expr) => {
        return Err(failure::Error::from($e));
    }
}
