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
    pub fn expand_src_files(&mut self)->Result<bool,()>{
        
        Ok(true)
    }   
}