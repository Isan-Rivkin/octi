use crate::parser::manifest::{Manifest, PackageType};
use std::io::Error;
use std::path::{Path, PathBuf};
use std::fs::{create_dir, create_dir_all, read_dir, remove_dir_all};

pub struct WorkspaceArtifact {
    artifacts: Vec<PathBuf>,
    manifest: Manifest,
    build_info: PathBuf, // TODO: switch to actual Build Info json struct
}

pub fn create_folder_in_workspace<P: AsRef<Path>>(base_path: P, folder_path: P) {
    let ws_path = base_path.as_ref().join("target").as_path();
    let ws_folder_path = ws_path.join(folder_path).as_path();

    create_dir_all(ws_folder_path);
}

// Helper functions to control the workspace
pub fn create_workspace<P: AsRef<Path>>(base_path: P) {
    let ws_path = base_path.as_ref().join("target").as_path();

    // Creates the workspace along with build and target folders
    if !workspace_exists(ws_path).unwrap() {
        create_dir(ws_path);
    }

    create_folder_in_workspace(ws_path, "artifacts");
    create_folder_in_workspace(ws_path, "sources");
    create_folder_in_workspace(ws_path, "build");
}

pub fn workspace_exists<P: AsRef<Path>>(base_path: P) -> Result<bool, Error> {
    let ws_path = base_path.as_ref().join("target").as_path();

    Ok(ws_path.exists())
}

pub fn workspace_artifacts<P: AsRef<Path>>(base_path: P) -> Result<Vec<WorkspaceArtifact>, Error> {
    let ws_path = base_path.as_ref().join("target").as_path();
    let artifacts_dir = ws_path.join("artifacts").as_path();

    // Check first for all the existing artifacts and list them
    let artifacts_paths = read_dir(artifacts_dir).unwrap();
    let artifacts = Vec<WorkspaceArtifact>::new();

    for path in artifacts_paths {
        // Each artifact path contains the artifacts, manifest and build.json (contains build info)
        let artifact: WorkspaceArtifact;
        
        // List all files in the directory
        let artifact_entries = read_dir(path.path()).unwrap();
        for artifact_entry in artifact_entries {
            match artifact_entry.file_name() {
                "build.json" => {
                    dep.build_info = artifact_entry.path();
                },
                "manifest.json" => {
                    dep.manifest = Manifest::new(artifact_entry.path());
                }
                _ => {
                    dep.artifacts.push(artifact_entry.path());
                },
            }
        }

        artifacts.push(artifact);
    }

    Ok(artifacts)
}

pub fn clear_workspace_artifacts<P: AsRef<Path>>(base_path: P) {
    // Removes all artifacts and recreates the artifacts folder
    let ws_path = base_path.as_ref().join("target").as_path();
    let artifacts_dir = ws_path.join("artifacts").as_path();

    remove_dir_all(artifacts_dir);
    create_dir(artifacts_dir);
}