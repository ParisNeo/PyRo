use clap::{Parser, Subcommand};
use pyro::{install_package, install_requirements, PythonManager, VirtualEnvManager, compress_project, decompress_project, PyroError};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pyro")]
#[command(about = "Pyro: A blazing-fast Python version and environment manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available Python versions
    List,
    /// Install a specific Python version
    Install { version: String },
    /// Create a virtual environment
    CreateVenv { path: String, python_version: String },
    /// Activate a virtual environment
    ActivateVenv { path: String },
    /// Compress a project into a portable archive
    Compress { project_path: String, output_path: String },
    /// Decompress a project archive
    Decompress { archive_path: String, output_path: String },
    /// Install a Python package using pip
    InstallPackage { package: String },
    /// Install packages from a requirements file
    InstallRequirements { requirements_file: String },
}

#[tokio::main]
async fn main() -> Result<(), PyroError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => PythonManager::list_versions().await?,
        Commands::Install { version } => PythonManager::install_version(&version).await?,
        Commands::CreateVenv { path, python_version } => {
            VirtualEnvManager::create_venv(&path, &python_version)?;
        }
        Commands::ActivateVenv { path } => {
            VirtualEnvManager::activate_venv(&path)?;
        }
        Commands::Compress { project_path, output_path } => {
            compress_project(&project_path, &output_path)?;
        }
        Commands::Decompress { archive_path, output_path } => {
            decompress_project(&archive_path, &output_path)?;
        }
        Commands::InstallPackage { package } => {
            let python_path = PathBuf::from("."); // Replace with actual Python path
            install_package(&python_path, &package)?;
        }
        Commands::InstallRequirements { requirements_file } => {
            let python_path = PathBuf::from("."); // Replace with actual Python path
            install_requirements(&python_path, &PathBuf::from(requirements_file))?;
        }
    }

    Ok(())
}