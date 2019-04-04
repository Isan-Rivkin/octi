use crate::manifest::{Manifest, PackageType};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::error::Error;
use super::templates::*;

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
        let mut template = match self.manifest.package_type {
            PackageType::bin => CMAKE_TARGET_BINARY_TEMPLATE,
            PackageType::staticlib | PackageType::sharedlib => CMAKE_TARGET_LIBARY_TEMPLATE
        }.to_string();
        
        template = template.replace("#OCTI_PROJECT_NAME#", &self.manifest.name.as_str());
        template = template.replace("#OCTI_PROJECT_SOURCE_FILES#", &self.manifest.src_files.join("\n"));
        template = template.replace("#OCTI_PROJECT_INCLUDE_DIRS#", &self.manifest.include_directories.join("\n"));
        
        match self.manifest.package_type {
            PackageType::staticlib => { template = template.replace("#OCTI_LIB_TYPE#", "STATIC") },
            PackageType::sharedlib => { template = template.replace("#OCTI_LIB_TYPE#", "SHARED") },
            PackageType::bin => {}
        }

        file.write_all(template.as_bytes());

        Ok(())
    }

    fn add_cmake_config(&self, file: &mut File) -> Result<(), Box<Error>> {
        let mut template: String = CMAKE_CONFIG_TEMPLATE.to_string().clone();
            
        let mut in_config: String = self.output_path.clone();
        let mut out_config: String = self.output_path.clone();

        in_config.push_str("config.h.temp");
        out_config.push_str("config.h");

        template = template.replace("#OCTI_CONFIG_INPUT#", &in_config);
        template = template.replace("#OCTI_CONFIG_OUTPUT#", &out_config);

        file.write_all(template.as_bytes());

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
        self.add_cmake_config(&mut file);

        Ok(())
    }
}