pub mod options;
pub mod exec_cmd;
pub mod logger;

use options::Options;
fn main() {
    let version = env!("CARGO_PKG_VERSION");
    println!("signing-check {} by Kin|Jiaching", version);
    let opts = Options::parse();
    println!("{:?}", opts);
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