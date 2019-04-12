use walkdir::{DirEntry, WalkDir};
use crate::eval::{traverse}; 
use crate::parser::manifest::{Manifest}; 
use std::{fs};
pub struct Evaluator<'a> {
    manifest : &'a mut Manifest, 
}


impl<'a> Evaluator<'a> {
    pub fn new(manifest: &'a mut Manifest) -> Evaluator<'a> {
        Evaluator {
            manifest : manifest
        }
    }
    /// add all the src files with valid extensions to the manifest
    /// default is src folder, no need to mention it.   
    pub fn expand_src_files(&mut self)->Result<bool,walkdir::Error>{
        let mut src_files : Vec<String> = Vec::new();
        let default_src = fs::canonicalize("./src").unwrap().to_str().unwrap().to_string();
        let mut dirs = vec![default_src]; 
        let src_dirs = self.manifest.get_src_dirs(); 
        // if not default path 
        if(src_dirs.len() > 0){
            dirs = src_dirs.to_vec(); 
        }
        // extract all src files recursily 
        for path in src_dirs{
            let src_entries = traverse::get_src_files(path)?;
            for entry in src_entries{
                let str_e = entry.path().to_str().unwrap();
                src_files.push(str_e.to_string());
            }
        }
        self.manifest.add_src_files(src_files);
        Ok(true)
    }   
}

#[cfg(test)]
mod test {
    use crate::eval::evaluator::{Evaluator};
    use crate::parser::manifest::Manifest; 
    #[test]
    fn test_evaluateor(){
        let mut m = Manifest::new("./src/parser/MANIFEST_TEMPL.json").unwrap();
        let mut evaluator = Evaluator::new(&mut m);
        assert!(evaluator.expand_src_files().unwrap());
        assert_eq!(3,m.get_src_files().len());
    }
}