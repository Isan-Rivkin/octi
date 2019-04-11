use crate::eval::{traverse}; 
use crate::parser::manifest::{Manifest}; 

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
    /// default is src folder 
    pub fn expand_src_files(&mut self)->Result<bool,()>{
        Ok(true)
    }   
}