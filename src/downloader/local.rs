use crate::parser::manifest::{Manifest,Dependency};
use std::{
    path::Path
};

#[derive(Default)]
pub struct WorkspaceSrc{
    src_path : String, 
    manifest : Manifest, 
}


// impl WorkspaceSrc{
//     pub fn new()->{

//     }
// }

pub trait Downloader{
    // fetch single dependency 
    fn download_dep<P: AsRef<Path>>(d : &Dependency, target_path : P)->Result<WorkspaceSrc,()>;
    // download all dependencies 
    fn download_manifest<P: AsRef<Path>>(m : &Manifest, target_path : P)->Result<Vec<WorkspaceSrc>,()>;
    fn is_exist<P: AsRef<Path>>(d : & Dependency, target_path : P) -> bool;
}

pub struct LocalDL; 

impl Downloader for LocalDL{
    /// get the dependency with all its dependencies 
    /// if the deps of the deps exist -> dont download 
    fn download_dep<P: AsRef<Path>>(d : &Dependency, target_path : P)->Result<WorkspaceSrc,()>{
        // check if dependencies 
        Ok(WorkspaceSrc::default())
    }

    fn download_manifest<P: AsRef<Path>>(m : &Manifest, target_path : P)->Result<Vec<WorkspaceSrc>,()>{
        // get all deps
        let mut deps : Vec<Dependency> = Vec::new();
        for d in m.get_dependencies(){
            deps.push(d.clone());
        }
        // for each deps get all deps 
        // move dep to target 
        // finally, move main manifest project to target 
        Ok(Vec::new())
    }
    fn is_exist<P: AsRef<Path>>(d : & Dependency, target_path : P) -> bool{
        false
    }
}
