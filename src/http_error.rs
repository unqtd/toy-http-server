#[derive(Debug)]
pub enum HttpError {
    Io(std::io::Error),
    BadStartingLineSyntax,
    BadHeaderSyntax,
    UnknowMethod(String),
}
