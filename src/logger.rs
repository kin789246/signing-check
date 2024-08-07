use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;
pub struct Logger {}

impl Logger {
    const DIR: &'static str = "logs";
    pub fn log(content: &str, path: &str) -> std::io::Result<()> {
        let combined = Path::new(Self::DIR).join(path);
        let full_dir = combined.parent().unwrap();
        if !full_dir.exists() {
            create_dir_all(full_dir).expect("create log directory failed");
        }
        let file_path = Self::DIR.to_owned() + "\\" + path;
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)?;
        writeln!(file, "{}", content)?;
        Ok(())
    }
}
