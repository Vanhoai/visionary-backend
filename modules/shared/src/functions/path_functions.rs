use std::path::Path;

pub struct PathFunctions;

impl PathFunctions {
    pub fn root_path() -> String {
        let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .parent().unwrap();

        workspace_dir.to_str().unwrap().to_string()
    }
}
