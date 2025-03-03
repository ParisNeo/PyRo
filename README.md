# PyRo

Pyro is a blazing-fast Python version and environment manager written in Rust. It supports both system-wide and portable modes, making it easy to manage Python environments and distribute Python applications.

## Features
- **Python Version Management**: Install and switch between Python versions.
- **Virtual Environment Management**: Create, activate, and manage virtual environments.
- **System-Wide Mode**: Manage Python installations globally.
- **Portable Mode**: Create self-contained Python environments for projects.
- **Compression**: Compress and decompress Python projects for easy distribution.

## Installation
```bash
cargo install --git https://github.com/ParisNeo/PyRo
```
## Usage
```bash
# Install a Python package
pyro install-package numpy

# Install packages from a requirements file
pyro install-requirements requirements.txt
```