#[derive(Debug)]
pub struct Error {
    code: String,
    error: String,
}

impl Error {
    pub fn internal(file: &'static str, line: u32, error: &'static str) -> Self {
        let file = file
            .split("/")
            .collect::<Vec<&str>>()
            .iter()
            .last()
            .unwrap_or(&"wtf")
            .replace(".rs", "");

        let code = format!("{}-{:04}", file, line);

        Self {
            code,
            error: error.into(),
        }
    }
}

#[macro_export]
macro_rules! internal {
    () => {
        crate::error::Error::internal(file!(), line!(), "unexpected server error")
    };
    ($error:expr) => {
        crate::error::Error::internal(file!(), line!(), $error)
    };
}
