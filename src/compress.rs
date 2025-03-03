use std::fs::{self, File};
use std::io::{self};
use std::path::Path;
use zip::write::FileOptions;
use walkdir::WalkDir;

pub fn compress_project(project_path: &str, output_path: &str) -> io::Result<()> {
    let file = File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);

    for entry in WalkDir::new(project_path) {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(Path::new(project_path)).unwrap();

        if path.is_file() {
            zip.start_file(name.to_string_lossy(), FileOptions::default())?;
            let mut f = File::open(path)?;
            io::copy(&mut f, &mut zip)?;
        }
    }

    zip.finish()?;
    Ok(())
}

pub fn decompress_project(archive_path: &str, output_path: &str) -> io::Result<()> {
    let file = File::open(archive_path)?;
    let mut zip = zip::ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = Path::new(output_path).join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}