pub mod templates;
pub mod generator;


#[cfg(test)]
mod test {
    use crate::cmake_generator::generator::{CMakeGenerator};
    use crate::parser::manifest::{Manifest};
    use std::str::FromStr;
    #[test]
    fn test_generate_cmake() {
        // Load manifest
        let m = Manifest::new("./src/parser/MANIFEST_TEMPL.json").unwrap();
        let gen = CMakeGenerator::new(String::from_str("./src/").unwrap(), &m);
        gen.generate();
    }
}