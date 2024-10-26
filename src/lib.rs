mod error;
mod rec;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        rec::Rec::from_embed().unwrap();
    }
}
