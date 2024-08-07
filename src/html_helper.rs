use std::{fs::File, io::{BufRead, Error, Read}};
use crate::csv_data::CsvOutData;
pub struct HtmlHelper {}

impl HtmlHelper {
    const BODY: &'static str = "<body>";
    const _ENDBODY: &'static str = "</body>";
    const H1: &'static str = "<h1>";
    const ENDH1: &'static str = "</h1>";
    const TABLE: &'static str = "<table>";
    const ENDTABLE: &'static str = "</table>";
    const _TH: &'static str = "<th>";
    const ENDTH: &'static str = "</th>";
    const TR: &'static str = "<tr>";
    const ENDTR: &'static str = "</tr>";
    const TD: &'static str = "<td>";
    const ENDTD: &'static str = "</td>";
    const STYLE: &'static str = "<style>";
    const SCRIPT: &'static str = "<script>";
    const STYLENAME: &'static str = "config\\style.css";
    const SCRIPTNAME: &'static str = "config\\main.js";

    pub fn to_html_table(data: &CsvOutData) -> String {
        let mut sb = String::new();
        let report_t = "config\\report";
        let mut file = File::open(report_t).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        for is_line in buffer.lines() {
            if let Ok(line) = is_line {
                sb.push_str(&line);
                sb.push_str("\n");
                if line.trim().eq_ignore_ascii_case("<title>") {
                    let version = env!("CARGO_PKG_VERSION");
                    let name = env!("CARGO_PKG_NAME");
                    sb.push_str(&format!("{} {}\n", name, version));
                }
                if line.trim().eq_ignore_ascii_case(Self::STYLE) {
                    sb.push_str(&Self::load_txt(Self::STYLENAME).unwrap());
                }
                if line.trim().eq_ignore_ascii_case(Self::BODY) {
                    sb.push_str(&Self::make_table(&data));
                }
                if line.trim().eq_ignore_ascii_case(Self::SCRIPT) {
                    sb.push_str(&Self::load_txt(Self::SCRIPTNAME).unwrap());
                }
            }
        }
        sb
    }

    fn add_string_to_td(sb: &mut String, text: &str, class_name: &str) {
        if !class_name.is_empty() {
            (*sb).push_str("<td class='");
            (*sb).push_str(class_name);
            (*sb).push_str("'>");
        }
        else {
            (*sb).push_str(Self::TD);
        }
        (*sb).push_str(text);
        (*sb).push_str(Self::ENDTD);
    }

    fn add_o_to_td(sb: &mut String, support: bool) {
        if support {
            Self::add_string_to_td(sb, "O", "");
        }
        else {
            Self::add_string_to_td(sb, "", "");
        }
    }

    fn make_table(data: &CsvOutData) -> String {
        let mut sb = String::new();
        sb.push_str("<div id='summary-block'>\n");
        sb.push_str(Self::H1);
        sb.push_str(&data.summary);
        sb.push_str(Self::ENDH1);
        sb.push_str("\n</div>\n");
        //div for display options
        sb.push_str("<div id='displayOptions'></div>\n");
        //table part
        sb.push_str("<div id='results'>\n");
        sb.push_str(Self::TABLE);
        sb.push_str("\n");
        sb.push_str(Self::TR);
        //NO.,Name,Summary,Path,PreProd,Attestation,TestSigning,WHQL,24H2,22H2,21H2,OtherOS,Expiry,Signers{Signer||ValidUsages||SigningDate||ValidFrom||ValidTo}
        sb.push_str("<th class='sticky'>No.");
        sb.push_str(Self::ENDTH);
        let adjustable = vec!["Path", "Other", "Signers"];
        let stickys = vec!["Name", "Summary"];
        for item in data.title.split(',').into_iter() {
            let mut add_class = String::new();
            let mut add_id = String::new();
            for adjust in adjustable.iter() {
                if item.contains(adjust) {
                    add_class = " class='".to_string() + adjust + "-column'";
                    break;
                }
            }
            for sticky in stickys.iter() {
                if item.contains(sticky) {
                    add_class = " class='sticky'".to_string();
                    add_id = " id='".to_string() + sticky + "-column'";
                    break;
                }
            }
                
            sb.push_str(&("<th".to_string() + &add_id + &add_class + ">"));
            sb.push_str(item);
            sb.push_str(Self::ENDTH);
        }
        sb.push_str(Self::ENDTR);
        sb.push_str("\n");
        let mut i = 1i32;
        for csv_data in data.data.iter() {
            sb.push_str(Self::TR);
            Self::add_string_to_td(&mut sb, &i.to_string(), "sticky"); i+=1;
            Self::add_string_to_td(&mut sb, &csv_data.name, "sticky");
            Self::add_string_to_td(&mut sb, &csv_data.summary, "sticky");
            Self::add_string_to_td(&mut sb, &csv_data.file_path, "Path-column");
            Self::add_o_to_td(&mut sb, csv_data.pre_prod);
            Self::add_o_to_td(&mut sb, csv_data.attestation);
            Self::add_o_to_td(&mut sb, csv_data.test_signing);
            Self::add_o_to_td(&mut sb, csv_data.whql);
            for os in csv_data.os_version.iter() {
                Self::add_o_to_td(&mut sb, os.1);
            }
            Self::add_string_to_td(&mut sb, &csv_data.other_os, "Other-column");
            Self::add_string_to_td(&mut sb, &csv_data.ts_expiry_date, "");
            let mut pre_str = String::new();
            for signer in csv_data.sigcheck_data.signers.iter() {
                pre_str = pre_str.clone() + &signer.name + " {";
                for vu in signer.valid_usages.iter() {
                    pre_str = pre_str.clone() + vu + ", ";
                }
                pre_str = 
                    pre_str.clone() +
                    "date:" + 
                    &signer.signing_date +
                    "||from:" +
                    &signer.valid_from +
                    "||to:" + 
                    &signer.valid_to +
                    "} ";
                }
                Self::add_string_to_td(&mut sb, &pre_str, "Signers-column");
                sb.push_str(Self::ENDTR);
                sb.push_str("\n");
            }
            sb.push_str(Self::ENDTABLE);
            sb.push_str("\n</div>\n");

        sb
    }

    fn load_txt(path: &str) -> Result<String, Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}