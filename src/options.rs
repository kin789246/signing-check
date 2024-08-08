use std::path::Path;
use chrono::Local;

#[derive(Debug)]
pub struct Options {
    pub save_log: bool,  // -v
    pub log_by_dir: bool,  // -f
    pub print_help: bool,
    pub is_zip: bool,
    pub not_exist: bool,
    pub source: String,
    pub output: String
}

impl Options {
    pub fn parse() -> Self {
        let mut save_log = false;
        let mut log_by_dir = false;
        let mut is_zip = false;
        let mut print_help = match std::env::args().len() {
            1 => true,
            _ => false
        };
        let mut not_exist = false;
        let mut source = String::new();
        let mut output = String::new();
        let mut args_iter = std::env::args().into_iter();
        let mut para = args_iter.next();
        while para.is_some() {
            let pp = para.clone().unwrap();
            match pp {
                s if s.eq_ignore_ascii_case("-v") => save_log = true,
                s if s.eq_ignore_ascii_case("-f") => log_by_dir = true,
                s if s.eq_ignore_ascii_case("-p") => {
                    if let Some(pp) = args_iter.next() {
                        Self::get_path(
                            &pp,
                            &mut is_zip, 
                            &mut not_exist,
                            &mut source,
                            &mut output
                        );
                    }
                },
                _ => {
                    para = args_iter.next();
                    continue;
                }
            }
            para = args_iter.next();
        } 
        if source.is_empty() {
            print_help = true;
        }
        if log_by_dir {
            output = output.clone() + "\\" + &output;
        }
        Self { save_log, log_by_dir, print_help, is_zip, not_exist, source, output }
    }

    fn get_path(
        line: &str,
        is_zip: &mut bool,
        not_exist: &mut bool,
        source: &mut String,
        output: &mut String
    ) {
        let time = Local::now();
        let time_stamp = time.format("_%Y%m%d_%H%M%S").to_string();
        let path = Path::new(line);
        if path.is_dir() {
            *is_zip = false;
            *source = line.trim_end_matches('\\').to_string();
            let o = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            *output = o + &time_stamp;
        }
        else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("zip") {
                    *is_zip = true;
                    *source = line.to_string();
                    let o = path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    *output = o + &time_stamp;
                }
            }
        }
        else {
            *not_exist = true;
        }
    }
}