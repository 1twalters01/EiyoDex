use std::{
    env,
    path::PathBuf
};

pub fn get_workspace_root() -> Result<String, std::env::VarError>{
    let workspace_root = std::env::var("WORKSPACE_ROOT")?;
    return Ok(workspace_root)
}

pub fn init_env() {
    let workspace_root = get_workspace_root().expect("Invalid workspace root");
    let env_path = std::path::Path::new(&workspace_root).join(".env");
    dotenvy::from_path(env_path).ok();
}

pub fn get_workspace_pathbuf() -> Result<PathBuf, env::VarError> {
    get_workspace_root().and_then(|workspace_root| Ok(PathBuf::from(workspace_root)))
}
