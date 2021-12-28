use crate::error::Error;

pub fn run() -> Error {
    internal!("error from module")
    // Error::internal(file!(), line!(), "internal error")
    // Error::internal("foo", line!(), "internal error")
}
