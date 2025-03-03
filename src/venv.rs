use std::process::Command;
use std::path::Path;

pub struct VirtualEnvManager;

impl VirtualEnvManager {
    pub fn create_venv(path: &str, python_version: &str) -> std::io::Result<()> {
        let python_bin = if cfg!(windows) {
            format!("python{}.exe", python_version)
        } else {
            format!("python{}", python_version)
        };

        Command::new(&python_bin)
            .arg("-m")
            .arg("venv")
            .arg(path)
            .status()?;
        Ok(())
    }

    pub fn activate_venv(path: &str) -> std::io::Result<()> {
        let activate_script = if cfg!(windows) {
            Path::new(path).join("Scripts").join("activate.bat")
        } else {
            Path::new(path).join("bin").join("activate")
        };

        if !activate_script.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Activation script not found",
            ));
        }

        println!("Run the following command to activate the virtual environment:");
        if cfg!(windows) {
            println!("{}", activate_script.display());
        } else {
            println!("source {}", activate_script.display());
        }

        Ok(())
    }
}