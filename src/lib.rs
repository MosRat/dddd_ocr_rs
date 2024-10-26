pub mod error;
pub mod rec;

pub use rec::Rec;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        rec::Rec::from_embed().unwrap();
    }
}
