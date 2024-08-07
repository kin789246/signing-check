pub mod options;

#[cfg(test)]
mod tests {
    use crate::options;

    #[test]
    fn test_path() {
        let opts = options::Options::parse();    
        println!("{:?}", opts);
    }
}