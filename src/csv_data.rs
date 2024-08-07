use crate::date_helper::parse_date;
use std::{collections::HashMap, path::Path, fmt};
use crate::sigcheck_data::SigcheckData;
#[derive(Clone)]
pub struct CsvOutData {
    pub summary: String,
    pub title: String, 
    pub data: Vec<CsvData>
}

impl CsvOutData {
    pub fn new() -> Self {
        Self {
            summary: String::new(),
            title: String::new(),
            data: Vec::new(),
        }
    }

    pub fn get_summary(&mut self) {
        if self.data.is_empty() {
            self.summary = "No file is analyzed".to_string();
        }
        else if self.data
            .iter()
            .all(|csv| 
                csv.summary
                    .eq_ignore_ascii_case(&self.data.first().unwrap().summary)) 
        {
            self.summary = self.data.first().unwrap().summary.clone();
        }
        else {
            self.summary = "Multiple Signed".to_string();
        }
    }

    pub fn get_title(&mut self, os_version: &Vec<(String, bool)>) {
        let mut sb = String::new();
        sb.push_str("Name,Summary,Path,PRE,ATT,TS,WHQL,");
        os_version.iter()
            .for_each(|item| {
                sb.push_str(&item.0);
                sb.push(',');
            });
        sb.push_str("Other,Expiry,Signers{Signer||ValidUsages||SigningDate||ValidFrom||ValidTo}");
        self.title = sb;
    }
}

impl fmt::Display for CsvOutData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sb = String::new();
        sb.push_str(&self.summary);
        sb.push_str("\n");
        sb.push_str(&self.title);
        sb.push_str("\n");
        self.data
            .iter()
            .for_each(|item| {
                sb.push_str(&item.to_string());
                sb.push_str("\n");
            });
        write!(f, "{}", &sb)
    }
}

#[derive(Clone)]
pub struct CsvData {
    pub name: String,
    pub summary: String,
    pub file_path: String,
    pub pre_prod: bool,
    pub test_signing: bool,
    pub attestation: bool,
    pub whql: bool,
    pub os_version: Vec<(String, bool)>,
    pub other_os: String,
    pub signer_info: String,
    pub ts_expiry_date: String,
    pub sigcheck_data: SigcheckData,
}

impl CsvData {
    const VU_WHQL: &'static str = "whql crypto";
    const VUATTESTATION: &'static str = "1.3.6.1.4.1.311.10.3.5.1";
    const VULIFETIME: &'static str = "lifetime signing";
    const WHCP: &'static str = "Microsoft Windows Hardware Compatibility Publisher";

    pub fn new(sig_data: &SigcheckData, osv: &Vec<(String, bool)>) -> Self {
        Self {
            name: String::new(),
            summary: String::new(),
            file_path: String::new(),
            pre_prod: false,
            test_signing: false,
            attestation: false,
            whql: false,
            os_version: osv.clone(),
            other_os: String::new(),
            signer_info: String::new(),
            ts_expiry_date: String::new(),
            sigcheck_data: sig_data.clone(),
        }
    }

    pub fn generate_output(&mut self, dl_sy_: &HashMap<String, String>) {
        self.name = self.check_expand_name(dl_sy_);
        self.file_path = Path::new(&self.sigcheck_data.file_name)
            .parent()
            .unwrap()
            .to_string_lossy()
            .to_string();
        if self.sigcheck_data.signing_type.eq_ignore_ascii_case("PreProd") {
            self.pre_prod = true;
        }
        let sp = self.sigcheck_data.os_support.split(',');
        for item in sp.into_iter() {
            let mut in_other = true;
            for i in 0..self.os_version.len() {
                if item.to_uppercase().contains(&self.os_version[i].0) {
                    if let Some(osv) = self.os_version.iter_mut().nth(i) {
                        (*osv).1 = true;
                        in_other = false;
                    }
                }
            }
            if in_other && !self.other_os.contains(item) {
                self.other_os = self.other_os.clone() + item + " ";
            }
        }
        self.other_os = self.other_os.trim_end_matches(' ').to_string();
        let mut sb = String::new();
        self.sigcheck_data
            .signers
            .iter()
            .for_each(|signer| {
                sb.push_str(&signer.to_string());
                sb.push('-');
                if signer.name.contains(CsvData::WHCP) {
                    if signer.valid_usages
                        .iter()
                        .any(|x| x.eq_ignore_ascii_case(CsvData::VU_WHQL)) &&
                        signer.valid_usages
                        .iter()
                        .any(|x| x.eq_ignore_ascii_case(CsvData::VUATTESTATION))
                    {
                        self.attestation = true;
                    }
                    else if signer.valid_usages
                        .iter()
                        .any(|x| x.eq_ignore_ascii_case(CsvData::VU_WHQL)) &&
                        signer.valid_usages
                        .iter()
                        .any(|x| x.eq_ignore_ascii_case(CsvData::VULIFETIME))
                    {
                        self.test_signing = true;
                        if self.ts_expiry_date.is_empty() {
                            self.ts_expiry_date = signer.valid_to.clone();
                        }
                        else {
                            let ts_expiry = parse_date(&self.ts_expiry_date).unwrap();
                            let valid_to = parse_date(&signer.valid_to).unwrap();
                            if ts_expiry > valid_to {
                                self.ts_expiry_date = signer.valid_to.clone();
                            }
                        } 
                    }
                    else if signer.valid_usages
                        .iter()
                        .any(|x| x.eq_ignore_ascii_case(CsvData::VU_WHQL)) 
                    {
                        self.whql = true;
                    }
                } 
            });
            self.signer_info = sb.trim_end_matches('-').to_string();
            self.summary = self.get_summary();
    }

    fn get_summary(&mut self) -> String {
        let mut summ_builder = String::new();
        if self.pre_prod {
            summ_builder.push_str("Pre-production signed + ");
        }
        if self.test_signing {
            if !self.whql {
                summ_builder.push_str("Test-signed + ");
            }
            else {
                summ_builder.push_str("Duo-signed (TS + WHQL) + ");
            }
        }
        if self.attestation {
            if !self.whql {
                summ_builder.push_str("Attestation-signed + ");
            }
            else {
                summ_builder.push_str("WHQL signed + ");
            }
        }
        
        if self.whql && !self.test_signing && !self.attestation {
            summ_builder.push_str("WHQL signed + ");
        }

        if !self.whql && !self.test_signing && !self.attestation {
            summ_builder.push_str("Not signed + ");
        }
        
        self.summary = summ_builder.trim_end_matches(&[' ', '+']).to_string();
        summ_builder.trim_end_matches(&[' ', '+']).to_string()
    }

    fn check_expand_name(&self, dl_sy_: &HashMap<String, String>) -> String {
        let full_name = Path::new(&self.sigcheck_data.file_name);
        let name = full_name.file_name().unwrap().to_str().unwrap();
        for item in dl_sy_.iter() {
            if item.0.contains(name) {
                return Path::new(item.1)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            }
        }
        name.to_string()
    }
}

impl fmt::Display for CsvData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sb = String::new();
        sb.push_str(&self.name);
        sb.push(',');
        sb.push_str(&self.summary);
        sb.push(',');
        sb.push_str(&self.file_path);
        sb.push(',');
        if self.pre_prod { sb.push('O'); }
        sb.push(',');
        if self.attestation { sb.push('O'); }
        sb.push(',');
        if self.test_signing { sb.push('O'); }
        sb.push(',');
        if self.whql { sb.push('O'); }
        sb.push(',');
        self.os_version
            .iter()
            .for_each(|item| {
                if item.1 { sb.push('O'); }
                sb.push(',');
            });
        sb.push_str(&self.other_os);
        sb.push(',');
        sb.push_str(&self.ts_expiry_date);
        sb.push(',');
        sb.push_str(&self.signer_info);

        write!(f, "{}", &sb)
    }
}