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


pub fn get_src_files(path : &str)->Result<Vec<DirEntry>,walkdir::Error>{
    let mut valid_entries = Vec::new();
    let walker = WalkDir::new(path).contents_first(true).into_iter();
    for entry in walker {
        let entry = entry?;
        if is_valid_extension(&entry){
            valid_entries.push(entry.clone()); 
        }
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
    use std::{
        fs,
        collections::HashSet,
    };
    use crate::eval::traverse;
    #[test]
    fn test_dir_traverse(){
        let path = fs::canonicalize("./src/eval/test").unwrap();
        let src_files = traverse::get_src_files(path.to_str().unwrap()).unwrap();
        let mut m : HashSet<String> = HashSet::new();
        for entry in src_files{
            let str_e = entry.path().to_str().unwrap();
            m.insert(str_e.to_string());
        }
        let expected_a = fs::canonicalize("./src/eval/test/a/src/folder/another.cpp").unwrap().to_str().unwrap().to_string();
        let expected_b = fs::canonicalize("./src/eval/test/a/src/main.cpp").unwrap().to_str().unwrap().to_string();
        let expected_c = fs::canonicalize("./src/eval/test/a/test/test.cpp").unwrap().to_str().unwrap().to_string();
        m.contains(&expected_a);
        m.contains(&expected_b);
        m.contains(&expected_c);
    }
}