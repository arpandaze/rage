use crate::core::Errors;

pub type Response<T> = Result<T, Errors>;
