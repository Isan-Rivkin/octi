pub mod ws_controller;

#[cfg(test)]
mod test {
    extern crate fs_extra;
    use crate::workspace::ws_controller;
    use std::path::Path;
    use fs_extra::dir::copy;
    use fs_extra::dir::CopyOptions;

    const WORKSPACE_NAME: &str = "octi-tests-ws";

    #[test]
    fn test_create_workspace() {
        ws_controller::create_workspace(WORKSPACE_NAME);
        assert!(Path::new([WORKSPACE_NAME, "target"].join("/").as_str()).exists());
        assert!(Path::new([WORKSPACE_NAME, "target", "build"].join("/").as_str()).exists());
        assert!(Path::new([WORKSPACE_NAME, "target", "artifacts"].join("/").as_str()).exists());
        assert!(Path::new([WORKSPACE_NAME, "target", "sources"].join("/").as_str()).exists());
    }

    #[test]
    fn test_collect_workspace_artifacts() {
        if ws_controller::workspace_exists(WORKSPACE_NAME).unwrap() {
            // Create the dummy artifact folder and copy the dummy artifact
            let mut options = CopyOptions::new();
            options.copy_inside = true;
            options.overwrite = true;
            let r = copy("dummy-artifacts/a", 
                [WORKSPACE_NAME, "target", "artifacts"].join("/"), &options);
            match(r) {
                Ok(v) => {
                    let artifacts = ws_controller::collect_workspace_artifacts(WORKSPACE_NAME).unwrap();
                    assert!(artifacts.len() == 1);
                }
                Err(e) => {
                    println!("Error = {:?}", e);
                }
            }
        }
    }
}