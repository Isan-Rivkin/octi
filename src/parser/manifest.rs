use crate::parser::os_info::{OsInfo};

use serde::{Deserialize, Serialize};
use uname; 
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::Path
};

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
    /// append path to src files derived from src dirs 
    pub fn add_src_files(&mut self,mut other_src_files : Vec<String>){
        self.src_files.append(&mut other_src_files);
    }
    pub fn get_src_dirs(&self)-> &Vec<String>{
        return &self.src_directories;
    }
    pub fn get_src_files(&self)-> &Vec<String>{
        return &self.src_files;
    }
}
