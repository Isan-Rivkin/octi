use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive( Deserialize,Serialize, Clone,Debug)]
#[serde(rename = "build_type")]
pub enum BuildType {
    Debug, 
    Release,
}

#[derive( Deserialize,Serialize, Clone,Debug)]
#[serde(rename = "package_type")]
pub enum PackageType {
    staticlib,
    sharedlib,
    bin,
}

#[derive( Deserialize,Serialize, Clone,Debug)]
pub struct Dependency{
    name : String, 
    version : String,
    uri : String,
    compiler_flags : Vec<String>, 
}
#[derive( Deserialize,Serialize, Clone,Debug)]
pub struct Manifest{
    name : String, 
    version : String,
    build_type : BuildType,
    package_type : PackageType,
    compiler_flags : Vec<String>,
    include_directories : Vec<String>,
    src_directories : Vec<String>,
    src_files : Vec<String>, 
    dependencies : Vec<Dependency>, 
}
impl Manifest{
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let u : Manifest = serde_json::from_reader(reader)?;
        Ok(u)
    }
}


#[cfg(test)]
mod test {
    use crate::manifest::{Manifest};
    #[test]
    fn test_load_manifest(){
        let m = Manifest::new("./src/manifest/MANIFEST_TEMPL.json").unwrap();
    }
}