use std::{fs::{create_dir_all, File}, io::copy, path::Path};
use zip::result::ZipError;

pub fn extract_exts(path: &str, exts: &Vec<String>) -> Result<String, ZipError> {
    let t = "tmp";
    let mut r = String::from("\n");
    let file = File::open(path)?;
    let mut zip = zip::ZipArchive::new(file)?;
    if !Path::new(t).exists() {
        let _= create_dir_all(t);
    }
    for ext in exts.iter() {
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let tp = t.to_string() + "\\" + file.name();
            if file.is_dir() {
                create_dir_all(tp).expect("create log directory failed");
                continue;
            }
            if file.is_file() {
                if file.name().ends_with(ext) {
                    let mut out_file = File::create(tp)?;
                    copy(&mut file, &mut out_file)?;   
                    r = r + "extract " + file.name() + "\n";
                    continue;
                }
            }
        }
    }
    Ok(r)
}

pub fn list_zip_contents(path: &str) -> zip::result::ZipResult<()> {
    let file = File::open(path)?;
    let mut zip = zip::ZipArchive::new(file)?;

    let mut n = 0;
    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        if file.is_dir() {
            continue;
        }
        n += 1;
        println!("{} Filename: {}", n, file.name());
    }

    Ok(())
}