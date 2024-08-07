use std::path::Path;
use chrono::{Local, Months};
use crate::date_helper::parse_date;

use crate::sigcheck_data::{SigcheckData, SignerData};

pub fn parse_signchain(raw: &str, sigchecks: &mut Vec<SigcheckData>, to_trim: &str) {
    let mut raw_iter = raw.lines().into_iter();
    let mut is_line = raw_iter.next();
    while is_line.is_some() {
        let line = is_line.unwrap().trim();
        let path = Path::new(line.trim_end_matches(':'));
        if path.exists() {
            if path.extension().unwrap().eq_ignore_ascii_case("cat") ||
                path.extension().unwrap().eq_ignore_ascii_case("dll") ||
                path.extension().unwrap().eq_ignore_ascii_case("sys")
            {
                let f_name = path.to_string_lossy().to_string();
                let trimed: String;
                if let Some(trim_i) = f_name.find(to_trim) {
                    trimed = (&f_name[(trim_i+to_trim.len())..]).to_string();
                }
                else {
                    trimed = f_name.clone();
                }
                if !sigchecks
                    .iter()
                    .any(|x| x.file_name.eq_ignore_ascii_case(&trimed)) 
                {
                    let mut s = SigcheckData::new();
                    s.file_name = trimed.clone();
                    sigchecks.push(s);
                }
                let sc: &mut SigcheckData = sigchecks
                    .iter_mut()
                    .find(|x| x.file_name.eq_ignore_ascii_case(&trimed))
                    .unwrap();
                is_line = raw_iter.next();
                while is_line.is_some() {
                    let mut line = is_line.unwrap().trim();
                    if line.starts_with("Signing date:") {
                        let mut signer = SignerData::new();
                        signer.signing_date = (&line[13..]).trim().to_string();
                        is_line = raw_iter.next();
                        while is_line.is_some() {
                            line = is_line.unwrap().trim();
                            if line.starts_with("Signers:") ||
                                line.starts_with("Signer:") 
                            {
                                if let Some(ll) = raw_iter.next() {
                                    signer.name = ll.replace(",", " ").trim().to_string();
                                }
                                for _i in 0..8 {
                                    is_line = raw_iter.next();
                                    line = is_line.unwrap().trim();
                                    if line.starts_with("Valid Usage:") {
                                        for v in (&line[12..]).trim().split(",") {
                                            signer.valid_usages.push(v.trim().to_string());
                                        }
                                    }
                                    else if line.starts_with("Valid from:") {
                                        signer.valid_from = (&line[11..]).trim().to_string();
                                    }
                                    else if line.starts_with("Valid to:") {
                                        signer.valid_to = (&line[9..]).trim().to_string();
                                        let now = Local::now()
                                            .format("%Y/%m/%d")
                                            .to_string();
                                        if let Ok(vt) = parse_date(&signer.valid_to) {
                                            let nt = parse_date(&now).unwrap();
                                            if nt
                                                .checked_add_months(Months::new(1))
                                                .unwrap() <
                                                vt
                                            {
                                                signer.validated = true;
                                            }
                                        }
                                    }
                                }
                                break;
                            }
                            is_line = raw_iter.next();
                        }
                        sc.signers.push(signer);
                    }
                    is_line = raw_iter.next();
                    if let Some(mt) = is_line {
                        if mt.trim().starts_with("MachineType:") {
                            break;
                        }
                    }
                }
            }
        }
        is_line = raw_iter.next();
    }
}