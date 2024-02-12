use crate::error::Error;

pub type Result<R, E = Error> = core::result::Result<R, E>;
#[derive(Clone)]
pub struct W<T>(pub T);
