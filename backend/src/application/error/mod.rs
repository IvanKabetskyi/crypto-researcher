use actix_web::error;
use std::fmt::{Debug, Display, Formatter};

pub struct DataError {
    pub message: &'static str,
}

impl Debug for DataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataError").finish()
    }
}

impl Display for DataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "internal server error: {}", self.message)
    }
}

impl error::ResponseError for DataError {}

impl DataError {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }
}
