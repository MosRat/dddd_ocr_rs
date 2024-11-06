#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum DDDDOcrError {
    #[cfg(feature = "use-ort")]
    #[error("ort error:`{0}`")]
    Ort(#[from] ort::Error),
    #[cfg(feature = "use-ncnn")]
    #[error("ncnn error:`{0}`")]
    Ncnn(#[from] anyhow::Error),
    #[error("io error:`{0}`")]
    Io(#[from] std::io::Error),
    #[error("gif decode error:`{0}`")]
    GifDecoder(#[from] gif::DecodingError),
    #[error("custom error:`{0}`")]
    Custom(String)
}

impl DDDDOcrError {
    #[allow(dead_code)]
    pub fn custom(s: &str) -> Self {
        Self::Custom(s.to_string())
    }
}

#[allow(dead_code)]
pub type DDDDOcrResult<T> = Result<T, DDDDOcrError>;