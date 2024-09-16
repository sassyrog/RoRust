use curl::easy::Easy;
use dirs::home_dir;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;
use std::path::PathBuf;
use tar::Archive;

pub fn get_default_installation_folder() -> PathBuf {
    let home = home_dir().unwrap();
    home.join(".venvit")
}

fn extract_tgz(tgz_path: &Path, extract_path: &Path) -> io::Result<()> {
    let file = File::open(tgz_path)?;
    let buf_reader = BufReader::new(file);
    let gz_decoder = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(gz_decoder);

    archive.unpack(extract_path)?;
    Ok(())
}

pub fn download_and_extract_python(version: &str, install_folder: &PathBuf) {
    let python_url = format!(
        "https://www.python.org/ftp/python/{}/Python-{}.tgz",
        version, version
    );

    let python_folder = format!("python-{}", version);
    let python_tar_file = format!("Python-{}.tgz", version);

    let mut easy = Easy::new();
    easy.url(&python_url).unwrap();

    let mut file = File::create(&python_tar_file).unwrap();
    let mut transfer = easy.transfer();

    transfer
        .write_function(|data| {
            file.write_all(data).unwrap();
            Ok(data.len())
        })
        .unwrap();

    transfer.perform().unwrap();

    // let temp_folder = ".";

    println!("{:?}", Path::new(&python_folder));
    if let Err(e) = extract_tgz(
        &Path::new(&python_tar_file),
        &install_folder.join(&install_folder),
    ) {
        eprintln!("Error extracting tarball: {:?}", e);
    }
}

#[cfg(target_os = "linux")]
fn build_and_install_python(version: &str, install_folder: &PathBuf) {
    let python_folder = format!("python-{}", version);
    let python_tar_file = format!("Python-{}.tgz", version);

    let python_folder_path = install_folder.join(&python_folder);

    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd {} && ./configure --prefix={} && make && make install",
            python_folder_path.display(),
            install_folder.display()
        ))
        .output()
        .expect("Failed to build and install python");

    println!("{:?}", output);
}
