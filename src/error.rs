use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    FileNotFound(PathBuf, String),
    DirectoryNotFound(PathBuf),
    CreakErr(creak::DecoderError),
    HoundErr(hound::Error),
    NoSamples,
    ChannelCoversionError(u32, u32),
}

impl std::convert::From<hound::Error> for Error {
    fn from(error: hound::Error) -> Self {
        Error::HoundErr(error)
    }
}

impl std::convert::From<creak::DecoderError> for Error {
    fn from(error: creak::DecoderError) -> Self {
        Error::CreakErr(error)
    }
}

pub type SBResult<T = ()> = Result<T, Error>;
