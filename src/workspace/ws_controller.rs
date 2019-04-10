use crate::parser::manifest::{Manifest, PackageType};
use std::io::Error;
use std::path::{Path, PathBuf};
use std::fs::{create_dir, create_dir_all, read_dir, remove_dir_all};

pub struct WorkspaceArtifact {
    pub full_path: Option<PathBuf>,
    pub artifacts: Vec<PathBuf>,
    pub manifest: Option<Manifest>,
    pub build_info: Option<PathBuf>, // TODO: switch to actual Build Info json struct
}

impl WorkspaceArtifact {
    pub fn new() -> WorkspaceArtifact {
        WorkspaceArtifact {
            full_path: None,
            artifacts: Vec::<PathBuf>::new(),
            manifest: None,
            build_info: None
        }
    }
}

pub fn create_folder_in_workspace<P: AsRef<Path>>(base_path: P, folder_path: P) {
    let ws_path = base_path.as_ref().join("target");
    let ws_folder_path = ws_path.join(folder_path);

    create_dir_all(ws_folder_path);
}

// Helper functions to control the workspace
pub fn create_workspace<P: AsRef<Path>>(base_path: P) {
    let ws_path = base_path.as_ref().join("target");

    // Creates the workspace along with build and target folders
    if !workspace_exists(&ws_path).unwrap() {
        create_dir(&ws_path);
    }

    create_folder_in_workspace(base_path.as_ref(), &Path::new("artifacts").to_path_buf());
    create_folder_in_workspace(base_path.as_ref(), &Path::new("sources").to_path_buf());
    create_folder_in_workspace(base_path.as_ref(), &Path::new("build").to_path_buf());
}

pub fn workspace_exists<P: AsRef<Path>>(base_path: P) -> Result<bool, Error> {
    let ws_path = base_path.as_ref().join("target");

    Ok(ws_path.exists())
}

pub fn collect_workspace_artifacts<P: AsRef<Path>>(base_path: P) -> Result<Vec<WorkspaceArtifact>, Error> {
    let ws_path = base_path.as_ref().join("target");
    let artifacts_dir = ws_path.join("artifacts");

    // Check first for all the existing artifacts and list them
    let artifacts_paths = read_dir(artifacts_dir).unwrap();
    let mut artifacts = Vec::<WorkspaceArtifact>::new();

    for path in artifacts_paths {
        // Each artifact path contains the artifacts, manifest and build.json (contains build info)
        let mut artifact = WorkspaceArtifact::new();
        
        // List all files in the directory
        let artifact_entries = read_dir(path.unwrap().path()).unwrap();
        for artifact_entry in artifact_entries {
            let dir_entry = artifact_entry.unwrap();
            artifact.full_path = Some(dir_entry.path());
            match dir_entry.file_name().to_str() {
                Some("build.json") => {
                    artifact.build_info = Some(dir_entry.path());
                },
                Some("manifest.json") => {
                    artifact.manifest = Some(Manifest::new(dir_entry.path()).unwrap());
                }
                _ => {
                    artifact.artifacts.push(dir_entry.path());
                },
            }
        }

        artifacts.push(artifact);
    }

    Ok(artifacts)
}

pub fn clear_workspace_artifacts<P: AsRef<Path>>(base_path: P) {
    // Removes all artifacts and recreates the artifacts folder
    let ws_path = base_path.as_ref().join("target");
    let artifacts_dir = ws_path.join("artifacts");

    remove_dir_all(&artifacts_dir);
    create_dir(artifacts_dir);
}

pub fn finalize_source_artifacts<P: AsRef<Path>>(base_path: P, source_name: String) {
    // TODO
}