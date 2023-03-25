use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("bad starting line syntax")]
    BadStartingLineSyntax,
    #[error("bad header syntax")]
    BadHeaderSyntax,
    #[error("unknow/unsupported method '{0}'")]
    UnknowMethod(String),
}
