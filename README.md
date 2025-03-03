# PyRo Documentation

## Introduction
PyRo is a blazing-fast Python version and environment manager written in Rust. It supports both system-wide and portable modes, making it easy to manage Python environments and distribute Python applications efficiently.

## Features
- **Python Version Management**: Install and switch between multiple Python versions.
- **Virtual Environment Management**: Create, activate, and manage virtual environments.
- **System-Wide Mode**: Manage Python installations globally for all users.
- **Portable Mode**: Create self-contained Python environments for projects.
- **Compression**: Compress and decompress Python projects for easy distribution.

## Installation
### Installing PyRo
To install PyRo using Cargo, run the following command:
```bash
cargo install --git https://github.com/ParisNeo/PyRo
```

### Installing Dependencies
Ensure you have Rust and Cargo installed on your system before installing PyRo. You may also need `zip`, `tar`, and `reqwest` dependencies for full functionality.

## Usage
### Listing Available Python Versions
To list available Python versions:
```bash
pyro list
```

### Installing a Specific Python Version
To install a specific Python version:
```bash
pyro install 3.9.7
```

### Creating a Virtual Environment
To create a virtual environment:
```bash
pyro create-venv my_env 3.9.7
```

### Activating a Virtual Environment
To activate a virtual environment:
```bash
pyro activate-venv my_env
```
Run the displayed command to activate the environment.

### Installing Packages
To install a package:
```bash
pyro install-package numpy
```

To install packages from a requirements file:
```bash
pyro install-requirements requirements.txt
```

### Compressing a Project
To compress a Python project into a portable archive:
```bash
pyro compress my_project output.zip
```

### Decompressing a Project
To extract a compressed Python project:
```bash
pyro decompress output.zip extracted_project
```

## Modes of Operation
### System-Wide Mode
System-wide mode installs Python globally on your machine. The configuration file (`config.json`) typically contains:
```json
{
    "default_python_version": "3.9",
    "venv_path": "/usr/local/pyro/venvs",
    "mode": "System"
}
```
Python installations and virtual environments are available system-wide.

### Portable Mode
Portable mode allows users to keep Python environments contained within project directories. Example configuration:
```json
{
    "default_python_version": "3.9",
    "venv_path": "./venvs",
    "mode": "Portable"
}
```
Each project will have its own independent Python environment, useful for distribution or working across different systems.

## Error Handling
### Common Errors
- **Failed to Download Python Version**: Ensure your internet connection is stable and check the Python version URL.
- **Virtual Environment Creation Fails**: Check if the specified Python version is installed.
- **Package Installation Issues**: Ensure that the `pip` executable exists within the virtual environment.

## Conclusion
PyRo simplifies Python version and environment management. Whether used as a system-wide tool or a portable solution, it enhances flexibility and efficiency in Python project development and distribution.

