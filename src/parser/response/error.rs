use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("ERR_NEEDMOREPARAMS")]
    ErrNeedMoreParams,
    #[error("ERR_ALREADYREGISTRED")]
    ErrAlreadyRegistered,
}
