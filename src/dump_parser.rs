use std::path::Path;
use crate::sigcheck_data::SigcheckData;

pub fn parse_dump(raw: &str, sigchecks: &mut Vec<SigcheckData>, to_trim: &str) {
    let mut sc: SigcheckData;
    let mut raw_iter = raw.lines().into_iter();
    let mut is_line = raw_iter.next();
    while is_line.is_some() {
        let line = is_line.unwrap().trim();
        let path = Path::new(line);
        if path.exists() {
            if path.extension().unwrap().eq_ignore_ascii_case("cat") {
                sc = SigcheckData::new();
                let f_name = path.to_string_lossy().to_string();
                let trimed: String;
                if let Some(trim_i) = f_name.find(to_trim) {
                    trimed = (&f_name[(trim_i+to_trim.len())..]).to_string();
                }
                else {
                    trimed = f_name.clone();
                }
                sc.file_name = trimed.clone();
                sigchecks.push(sc);
            }
        }
        else if line.starts_with("OS:") {
            if let Some(sig) = sigchecks.last_mut() {
                sig.os_support = (&line[3..]).to_string().trim().to_string();
            }
        }
        else if line.starts_with("SigningType:") {
            if let Some(sig) = sigchecks.last_mut() {
                sig.signing_type = (&line[12..]).to_string().trim().to_string();
            }
        }
        is_line = raw_iter.next();
    }
}