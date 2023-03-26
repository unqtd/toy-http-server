#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("some IO error")]
    Io(anyhow::Error),
    #[error("bad starting line syntax")]
    BadStartingLineSyntax,
    #[error("bad header syntax")]
    BadHeaderSyntax,
    #[error("unknow/unsupported method '{0}'")]
    UnknowMethod(String),
    #[error("bad 'Content-Length' header syntax")]
    BadContentLengthSyntax,
}
