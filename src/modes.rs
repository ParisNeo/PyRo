use std::path::PathBuf;

pub enum Mode {
    System(SystemMode),
    Portable(PortableMode),
}

pub struct SystemMode {
    base_path: PathBuf,
}

pub struct PortableMode {
    project_path: PathBuf,
}

impl SystemMode {
    pub fn new() -> Self {
        let base_path = if cfg!(windows) {
            PathBuf::from("C:\\PyRo")
        } else {
            PathBuf::from("/usr/local/pyro")
        };
        SystemMode { base_path }
    }
}

impl PortableMode {
    pub fn new(project_path: PathBuf) -> Self {
        PortableMode { project_path }
    }
}