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
        &format!("save results to {}", &(opts.output + ".csv")),
        &log_name,
        true,
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

/*
using System;
using System.Collections.Generic;
using System.IO;
using System.Reflection;

namespace SigningCheck
{
    internal class Program
    {
        private static string logName;
        private static Options opts;
        private static Dictionary<string, string> dllsysToDel = new Dictionary<string, string>(); 
        static int Main(string[] args)
        {
            Console.WriteLine(Version + " by Kin|Jiaching");
            opts = parseParameter(args);
            Log("### " + Version + " ###", logName, opts.LogDetail);
            List<SigcheckData> sigcheckDatas = new List<SigcheckData>();
            if (opts.IsZip)
            {
                string extractPath = "tmp";
                ZipHelper zh = new ZipHelper(extractPath, opts.SourceName, logName);
                Log("start extracting " + opts.SourceName, logName, opts.LogDetail);
                zh.ExtractFiles(new List<string> { "cat", "dll", "sys", "dl_", "sy_" });
                processFiles(extractPath, sigcheckDatas, opts.OutputName, logName);

                if (Directory.Exists(extractPath)) Directory.Delete(extractPath, true);
            }
            else
            {
                processFiles(opts.SourceName, sigcheckDatas, opts.OutputName, logName);
                clearExpanded();
            }

            Log("### end ###", logName, opts.LogDetail);
            return 0;
        }
        private static void clearExpanded()
        {
            foreach (var name in dllsysToDel)
            {
                File.Delete(name.Key);
            }
        }
        private static Options parseParameter(string[] args)
        {
            Options options = new Options();
            string helpStr =
                    "command examples:\n" +
                    "command1: signingcheck -p path\\drv_name.zip\n" +
                    "command2: signingcheck -p path\\of\\driver\\directory\n\n" +
                    "add -f for logs in folder\n" +
                    "command3: signingcheck -f -p path\\drv_name.zip\n" +
                    "command4: signingcheck -f -p path\\of\\driver\\directory\n\n" +
                    "add -v for saving all detail logs";
            if (args.Length < 1)
            {
                Console.WriteLine(helpStr);
                Environment.Exit(1);
            }
            options.Build(args);
            if (string.IsNullOrEmpty(options.SourceName))
            {
                Console.WriteLine(helpStr);
                Environment.Exit(1);
            }
            logName = options.OutputName + "_SigningCheck.log";
            return options;
        }
        private static void processFiles(string drvPath, List<SigcheckData> sigcheckDatas, string resultName, string logName)
        {
            drvPath = Path.GetFullPath(drvPath);
            if (Directory.Exists(drvPath))
            {
                Log("start parsing", logName, opts.LogDetail);

                Log("get dump for cat files", logName, opts.LogDetail);
                string dumpCmd = "exe\\sigcheck.exe /accepteula -d -s \"" + drvPath + "\\*.cat\"";
                string dumplog = CmdHelper.Run(dumpCmd);
                string dumpName = resultName + "_dump.log";
                Log(dumplog, dumpName, opts.LogDetail, false);
                Log("dump is saved to " + dumpName, logName, opts.LogDetail);
                Log("parsing dump", logName, opts.LogDetail);
                DumpParser.ParseDump(dumplog, sigcheckDatas, drvPath);

                expandDl_Sy_(drvPath);

                Log("get signing chain", logName, opts.LogDetail);
                getSigningChain(new List<string> { "cat", "dll", "sys" }, sigcheckDatas, drvPath, resultName, logName);

                generateCSV(resultName, sigcheckDatas);
                Log("Summary is saved to " + resultName + ".csv and " + resultName + ".html", logName, opts.LogDetail, true, true);
            }
            else
            {
                Log("Can't find cat, dll, sys files in this driver package.", logName, opts.LogDetail, true, true);
            }
        }
        private static void expandDl_Sy_(string drvPath)
        {
            Dictionary<string, string> exts = new Dictionary<string, string>();
            exts.Add(".dl_", ".dll");
            exts.Add(".sy_", ".sys");
            string expandLog = opts.OutputName + "_expand.log";
            string[] fileEntries = Directory.GetFiles(drvPath);
            foreach (string fileName in fileEntries)
            {
                foreach (string ext in exts.Keys)
                {
                    if (ext == Path.GetExtension(fileName))
                    {
                        Log("Expand " + fileName, expandLog, opts.LogDetail);
                        string dest = Path.GetFileNameWithoutExtension(fileName) + exts[ext];
                        dest = Path.Combine(Path.GetDirectoryName(fileName), dest);
                        string r = CmdHelper.Run("expand " + fileName + " " + dest);
                        dllsysToDel.Add(dest, fileName);
                        Log(r, expandLog, opts.LogDetail);
                        break;
                    }
                }
            }
            // Recurse into subdirectories of this directory.
            string[] subdirectoryEntries = Directory.GetDirectories(drvPath);
            foreach (string subdirectory in subdirectoryEntries)
            {
                expandDl_Sy_(subdirectory);
            }
        }
        private static void getSigningChain(List<string> extensions, List<SigcheckData> sigcheckDatas, string drvPath, string resultName, string logName)
        {
            foreach (var ext in extensions)
            {
                string scCmd = "exe\\sigcheck /accepteula -i -s " + "\"" + drvPath + "\\*." + ext + "\"";
                string signingChain = CmdHelper.Run(scCmd);
                string chainName = resultName + "_signingChain.log";
                Log(signingChain, chainName, opts.LogDetail, false);
                Log(ext + " files signing chain information is saved to " + chainName, logName, opts.LogDetail);
                Log("parsing signing chain information for " + ext + " files", logName, opts.LogDetail);
                SigningChainParser.ParseSigningChain(signingChain, sigcheckDatas, drvPath);
            }
        }
        public static string Version
        {
            get
            {
                Assembly asm = Assembly.GetExecutingAssembly();
                AssemblyName asmName = asm.GetName();
                return String.Format("{0} v{1}", asmName.Name, asmName.Version.ToString());
            }
        }
        private static void generateCSV(string fileName, List<SigcheckData> sigcheckDatas)
        {
            CsvOutData csvOutData = new CsvOutData();
            string osCfg = "config\\os.cfg";
            List<(string, bool)> osVersion = new List<(string, bool)>();
            loadOsCfg(osCfg, osVersion);
            csvOutData.GetTitle(osVersion);
            foreach (var item in sigcheckDatas)
            {
                CsvData output = new CsvData(item, osVersion);
                output.GenerateOutput(dllsysToDel);
                csvOutData.Data.Add(output);
            }

            //sort by file path
            //csvOutData.Data = csvOutData.Data.OrderBy(x => x.FilePath).ToList<CsvData>();

            Log(csvOutData.ToCsvString(), fileName + ".csv", true, false);
            Log(HtmlHelper.ToHtmlTable(csvOutData), fileName + ".html", true, false);
        }

        private static void loadOsCfg(string osCfgFile, List<(string, bool)> osVersion)
        {
            using (StreamReader sr = new StreamReader(osCfgFile))
            {
                try
                {
                    string line = sr.ReadLine();
                    while (line != null)
                    {
                        osVersion.Add((line.ToUpper(), false));
                        line = sr.ReadLine();
                    }
                }
                catch (Exception e)
                {
                    Console.WriteLine(e.ToString());
                }
            }
        }

        private static void Log(string message, string fileName, bool logDetail, bool timeStamp = true, bool screen = false)
        {
            if (logDetail)
            {
                LogWriter.Log(message, fileName, timeStamp);
            }
            if (screen)
            {
                Console.WriteLine(message);
            }
        }
    }
}
*/