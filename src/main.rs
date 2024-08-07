pub mod options;
pub mod exec_cmd;
pub mod logger;
pub mod zip_helper;
pub mod sigcheck_data;
pub mod csv_data;
pub mod dump_parser;
pub mod signing_chain_parser;
pub mod html_helper;
pub mod date_helper;

use std::{
    collections::HashMap, 
    fs::{self, remove_dir_all, remove_file, File}, 
    io::{self, Error, Read},
    path::Path, 
    process::ExitCode
};
use chrono::Local;
use csv_data::{CsvData, CsvOutData};
use dump_parser::parse_dump;
use exec_cmd::cmd;
use html_helper::HtmlHelper;
use logger::Logger;
use options::Options;
use sigcheck_data::SigcheckData;
use signing_chain_parser::parse_signchain;
fn main() -> ExitCode {
    const HELP_STR: &'static str = 
        "command examples:\n\
        command1: signingcheck -p path\\drv_name.zip\n\
        command2: signingcheck -p path\\of\\driver\\directory\n\n\
        add -f for logs in folder\n\
        command3: signingcheck -f -p path\\drv_name.zip\n\
        command4: signingcheck -f -p path\\of\\driver\\directory\n\n\
        add -v for saving all detail logs";
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    println!("{} {} by Kin|Jiaching", name, version);

    let opts = Options::parse();
    if opts.not_exist {
        println!("file or directory doesn't exist");
        return ExitCode::from(1);
    }
    if opts.print_help {
        println!("{}", HELP_STR);
        return ExitCode::from(1);
    }
    let log_name = opts.output.clone() + "_signingcheck.log";
    log(
        &format!("### {} {} ###", name, version),
        &log_name,
        opts.save_log,
        false,
        false
    );

    let p: &Path;
    let to_trim: &str;
    if opts.is_zip {
        let exts: Vec<String> = vec![ 
            "cat".to_string(), 
            "dll".to_string(), 
            "sys".to_string(), 
            "dl_".to_string(), 
            "sy_".to_string()
        ];
        if let Ok(r) = zip_helper::extract_exts(&opts.source, &exts) {
            log(&r, &log_name, opts.save_log, true, false);
        }
        p = Path::new("tmp");
        to_trim = "tmp";
    }
    else {
        p = Path::new(&opts.source);
        to_trim = p.parent().unwrap().to_str().unwrap();
    }
    let mut to_del: HashMap<String, String> = HashMap::new();
    if let Ok(exp_log) = visit_dirs(p, &mut to_del) {
        log(
            &format!("try to expand dl_ sy_\n{}", &exp_log),
            &log_name, 
            opts.save_log, 
            true, 
            false
        );
    }
    let pp = p.to_string_lossy().to_string();
    let dump_cmd = r#"sigcheck.exe -d -s ""#.to_string() + &pp + r#"\\*.cat""#;
    let dumplog = cmd(&dump_cmd);
    let dump_file = opts.output.clone() + "_dump.log";
    log(
        &format!("save dump log to {}", &dump_file),
        &log_name, 
        opts.save_log, 
        true, 
        false
    );
    log(&dumplog, &dump_file, opts.save_log, false, false);
    let mut sigchecks: Vec<SigcheckData> = Vec::new();
    parse_dump(&dumplog, &mut sigchecks, &to_trim.to_lowercase());
    log("parsing dump", &log_name, opts.save_log, true, false);

    let exts = vec!["sys", "dll", "sy_", "dl_", "cat"];
    exts.iter()
        .for_each(|ext| {
            let sc_cmd = 
                r#"sigcheck.exe -i -s ""#.to_string() +
                &pp +
                r#"\\*."# +
                &ext +
                r#"""#;
            let sc_log = cmd(&sc_cmd);
            let chain_file = opts.output.clone() + "_signingchain.log";
            log(
                &format!("save signing chain log for *.{} to {}", &ext, &chain_file),
                &log_name, 
                opts.save_log, 
                true, 
                false
            );
            log (&sc_log, &chain_file, opts.save_log, false, false);
            parse_signchain(&sc_log, &mut sigchecks, &to_trim.to_lowercase());
            log(
                &format!("parsing signing chain for {}", &ext), 
                &log_name, 
                opts.save_log, 
                true, 
                false
            );
        });
    
    gen_csv(&opts.output, &sigchecks, &to_del);
    log(
        &format!(
            "save results to {}", 
            &(opts.output.clone() + ".csv")
        ),
        &log_name,
        opts.save_log,
        true,
        true
    );
    log(
        &format!(
            "save results to {}", 
            &(opts.output.clone() + ".html")
        ),
        &log_name,
        opts.save_log,
        true,
        true
    );
    // _log_sigcheck_list(&opts.output, &sigchecks);

    let tmp_dir = Path::new("tmp");
    if tmp_dir.exists() {
        let _rm = remove_dir_all(tmp_dir);
    }
    if !to_del.is_empty() {
        clear_expanded(&to_del);
    }
    log("### end ###", &log_name, opts.save_log, false, false);
    ExitCode::SUCCESS
}

fn visit_dirs(dir: &Path, mut to_del: &mut HashMap<String, String>) -> io::Result<String> {
    let mut r = String::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                r = r.clone() + &visit_dirs(&path, &mut to_del)?;
            } else {
                let exts: Vec<String> = vec![ "dl_".to_string(), "sy_".to_string() ];
                let pairs: HashMap<&str, &str> = 
                    HashMap::from([ ("dl_", "dll"), ("sy_", "sys") ]);
                exts.iter().for_each(|f| {
                    let nn = path.to_str().unwrap();
                    if nn.ends_with(f) {
                        if let Some(&ext) = pairs.get(f.as_str()) {
                            let dest_name = path.file_stem()
                                .unwrap()
                                .to_string_lossy()
                                .to_string()
                                + "." + ext;
                            let dest = path
                                .parent()
                                .unwrap()
                                .to_string_lossy()
                                .to_string() 
                                + "\\" + &dest_name;
                            let c = "expand ".to_string() + nn + " " + &dest;
                            let exp_log = exec_cmd::cmd(&c);
                            r = r.clone() + &exp_log + "\n";
                            to_del.insert(dest, nn.to_string());
                        }
                    }
                });
            }
        }
    }
    Ok(r)
}

fn log(content: &str, path: &str, save_log: bool, add_time: bool, on_screen: bool) {
    let mut msg = content.to_string();
    if add_time {
        let time = Local::now();
        let time_stamp = time.format("%Y-%m-%d_%H:%M:%S ").to_string();
        msg = time_stamp + content;
    }
    if on_screen {
        println!("{}", msg);
    }
    if save_log {
        let _ = Logger::log(&msg, path);
    }
}

fn clear_expanded(to_del: &HashMap<String, String>) {
    for (name, _) in to_del.iter() {
        let n = Path::new(name);
        let _r = remove_file(&n);
    }
}

fn _log_sigcheck_list(file: &str, sigchecks: &Vec<SigcheckData>) {
    for sig in sigchecks.iter() {
        log(
            &sig.to_string(), 
            &(file.to_string() + "_sigchecks.log"), 
            true, 
            false, 
            false
        );
    }
}

fn gen_csv(
    path: &str, 
    sigchecks: &Vec<SigcheckData>, 
    to_del: &HashMap<String, String>
) {
    let mut csv_out = CsvOutData::new();
    let os_cfg = "config\\os.cfg";
    let mut os_ver: Vec<(String, bool)> = Vec::new();
    let _r = load_os_cfg(&os_cfg, &mut os_ver);
    csv_out.get_title(&os_ver);
    sigchecks.iter()
        .for_each(|item| {
            let mut output = CsvData::new(item, &os_ver);
            output.generate_output(to_del);
            csv_out.data.push(output);
        });
    csv_out.get_summary();
    let f_csv = path.to_string() + ".csv";
    let f_html = path.to_string() + ".html";
    log(&csv_out.to_string(), &f_csv, true, false, false);
    log(
        &HtmlHelper::to_html_table(&csv_out), 
        &f_html, 
        true, 
        false,
        false
    );
}

fn load_os_cfg(path: &str, os_ver: &mut Vec<(String, bool)>) -> Result<(), Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    for line in content.lines() {
        os_ver.push((line.to_uppercase(), false));
    }
    Ok(())
}