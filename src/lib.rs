#[cfg(all(feature = "use-ort", feature = "use-ncnn"))]
compile_error!("'use-ort' and 'use-ncnn' cannot be enabled at the same time.");

pub mod error;
pub mod rec;

pub use rec::Rec;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "use-ort")]
    fn it_works() {
        rec::Rec::from_embed().unwrap();
    }
    #[test]
    #[cfg(feature = "use-ncnn")]
    fn it_works() {
        rec::Rec::from_model_and_params().unwrap();
    }

}
