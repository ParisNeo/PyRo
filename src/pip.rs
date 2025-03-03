use std::process::Command;
use std::path::Path;

/// Install a Python package using pip
pub fn install_package(python_path: &Path, package: &str) -> std::io::Result<()> {
    let pip_command = if cfg!(windows) {
        python_path.join("Scripts").join("pip.exe")
    } else {
        python_path.join("bin").join("pip")
    };

    Command::new(pip_command)
        .arg("install")
        .arg(package)
        .status()?;
    Ok(())
}

/// Install packages from a requirements file
pub fn install_requirements(python_path: &Path, requirements_file: &Path) -> std::io::Result<()> {
    let pip_command = if cfg!(windows) {
        python_path.join("Scripts").join("pip.exe")
    } else {
        python_path.join("bin").join("pip")
    };

    Command::new(pip_command)
        .arg("install")
        .arg("-r")
        .arg(requirements_file)
        .status()?;
    Ok(())
}