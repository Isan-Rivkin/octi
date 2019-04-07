pub mod os_info;
pub mod manifest;

#[cfg(test)]
mod test {
    use crate::parser::os_info::{OsInfo};
    use crate::parser::manifest::{Manifest};
    #[test]
    fn test_load_manifest(){
        let m = Manifest::new("./src/parser/MANIFEST_TEMPL.json").unwrap();
        println!("{:?}",m);
    }
}