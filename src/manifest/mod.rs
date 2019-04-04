use serde::{Deserialize, Serialize};
use uname; 
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
    pub name : String, 
    pub version : String,
    pub uri : String,
    pub compiler_flags : Vec<String>, 
}
#[derive(Debug,Clone)]
pub struct OsInfo{
    pub arch : String, 
    pub os : String,
}
impl OsInfo{
    pub fn new()->Self{
        let info = uname::uname().unwrap();
        OsInfo {
         os : info.sysname, 
         arch : info.machine,
        }
    }
}
impl Default for OsInfo {
    fn default() -> Self { 
        OsInfo::new()
     }
}

#[derive( Deserialize,Serialize, Clone,Debug)]
pub struct Manifest{
    pub name : String, 
    pub version : String,
    pub build_type : BuildType,
    pub package_type : PackageType,
    pub compiler_flags : Vec<String>,
    pub include_directories : Vec<String>,
    pub src_directories : Vec<String>,
    pub src_files : Vec<String>, 
    pub dependencies : Vec<Dependency>, 
    #[serde(skip)]
    pub os_info : OsInfo,
}
impl Manifest{
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let u : Manifest = serde_json::from_reader(reader)?;
        Ok(u)
    }
    fn set_os_info(&mut self){
        self.os_info = OsInfo::new();
    }
}


#[cfg(test)]
mod test {
    use crate::manifest::{Manifest,OsInfo};
    #[test]
    fn test_load_manifest(){
        let m = Manifest::new("./src/manifest/MANIFEST_TEMPL.json").unwrap();
        println!("{:?}",m);
    }
    #[test]
    fn basic_os_detection(){
        // if cfg!(windows) {
        //     println!("this is windows");
        // } else if cfg!(unix) {
        //     println!("this is unix alike");
        // }
        // use uname;
        // let info = uname::uname().unwrap();
        // println!("{:?}",info);
        // let sys_name = info.sysname; 
        // let machine = info.machine; 
        let i = OsInfo::new();
        assert_eq!(String::from("Linux"), i.os);
    }
}