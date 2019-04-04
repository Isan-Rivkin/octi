// use manifest::Manifest;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::error::Error;
use super::templates::*;

#[derive(Debug)]
pub struct Manifest {
    name: String,
    version: String,
    compiler_flags: Vec<String>,
    package_type: String,
}

#[derive(Debug)]
pub struct CMakeGenerator<'a> {
    output_path: String,
    manifest: &'a Manifest,
}

impl<'a> CMakeGenerator<'a> {
    pub fn new(output_path: String, manifest: &'a Manifest) -> CMakeGenerator<'a> {
        CMakeGenerator { output_path: output_path, manifest: manifest }
    }

    fn add_cmake_meta(&self, file: &mut File) -> Result<(), Box<Error>> {
        let mut template: String = CMAKE_PACKAGE_METADATA_TEMPLATE.to_string().clone();

        template = template.replace("#OCTI_PROJECT_NAME#", &self.manifest.name.to_string());
        template = template.replace("#OCTI_PROJECT_VERSION#", &self.manifest.version.to_string());

        file.write_all(template.as_bytes()) ?;

        Ok(())
    }

    fn add_cmake_compiler_flags(&self, file: &mut File) -> Result<(), Box<Error>> {
        let mut template: String = CMAKE_COMPILER_FLAGS_TEMPLATE.to_string().clone();
        let joined_flags = self.manifest.compiler_flags.join(" ");

        template = template.replace("#OCTI_COMPILER_FLAGS#", &joined_flags.to_string());
        
        file.write_all(template.as_bytes()) ?;

        Ok(())
    }

    fn add_cmake_target(&self, file: &mut File) -> Result<(), Box<Error>> {
        let mut template = match self.manifest.package_type.as_str() {
            "bin" => CMAKE_TARGET_BINARY_TEMPLATE,
            _ => CMAKE_TARGET_LIBARY_TEMPLATE
        };
        
        template.replace("#OCTI_PROJECT_NAME#", &self.manifest.name.as_str());
        template.replace("#OCTI_PROJECT_INCLUDE_DIRS#", &self.manifest.)

        Ok(())
    }

    fn create_project_config(&self) -> Result<(), Box<Error>> {
        let path = Path::new(self.output_path.as_str()).join("config.h.temp");

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = File::create(&path) ?;
        file.write_all(CPP_CONFIG_TEMPLATE.as_bytes());

        Ok(())
    }

    pub fn generate(&self) -> Result<(),Box<Error>>{
        let mut path = Path::new(self.output_path.as_str()).join("CMakeLists.txt");

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = File::create(&path) ?;

        // Create the template config file
        self.create_project_config();

        // Start adding the cmake parts     
        self.add_cmake_meta(&mut file);
        self.add_cmake_compiler_flags(&mut file);
        self.add_cmake_target(&mut file);

        Ok(())
    }
}