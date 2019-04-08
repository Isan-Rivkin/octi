use walkdir::{DirEntry, WalkDir};
use lazy_static;
use std::{
    collections::HashSet,
    path::{Path,PathBuf},
    ffi::OsStr,
};

lazy_static! {
    static ref EXTENSIONS: HashSet<String> = {
        let mut m = HashSet::new();
        m.insert("cpp".to_string());
        m.insert("cxx".to_string());
        m.insert("tpp".to_string());
        m.insert("c".to_string());
        m
    };
}


pub fn get_valid_files(path : &str)->Result<Vec<DirEntry>,walkdir::Error>{
    let mut valid_entries = Vec::new();
    let walker = WalkDir::new(path).contents_first(true).into_iter();
    for entry in walker.filter_entry(|e| is_valid_extension(e)) {
        let entry = entry?;
        println!("{}", entry.path().display());
    }
    Ok(valid_entries )
}

pub fn is_valid_extension(entry: &DirEntry)->bool{
    match entry.path().extension(){
        Some(extension) => {
            let e_str = extension.to_str().unwrap(); 
            EXTENSIONS.contains(&e_str.to_string())
        }, 
        None => {
            false 
        }
    }
}

#[cfg(test)]
mod test {
    use walkdir::{DirEntry, WalkDir};
    use std::fs;
    use crate::eval::traverse;
    #[test]
    fn test_dir_traverse(){
        let path = fs::canonicalize("./dummy-libraries/a/src").unwrap();
        traverse::get_valid_files(path.to_str().unwrap()).unwrap();
    }
}