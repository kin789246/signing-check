#[derive(Debug)]
pub struct Options {
    pub save_log: bool,
    pub gui_mode: bool,
    pub force: bool,
    pub inf_list: String
}

impl Options {
    pub fn parse() -> Self {
        let mut save_log = false;
        let mut force = false;
        let gui_mode = match std::env::args().len() {
            1 => true,
            _ => false
        };
        let mut inf_list = String::new();
        for line in std::env::args() {
            match &line {
                s if s.eq_ignore_ascii_case("-s") => save_log = true,
                s if s.contains(".txt") => inf_list = line,
                s if s.eq_ignore_ascii_case("-f") => force = true,
                _ => continue
            }
        }
        Self { save_log, gui_mode, force, inf_list }
    }
}
/*
using System;
using System.IO;
using System.Text.RegularExpressions;

namespace SigningCheck
{
    public class Options
    {
        private bool logByDir = false;
        private bool isZip;
        private bool logDetail = false;
        private string sourceName;
        private string outputName;

        public bool LogByDir { get { return logByDir; } }
        public bool IsZip { get { return isZip; } }
        public bool LogDetail { get { return logDetail; } }
        public string SourceName { get { return sourceName; } }
        public string OutputName { get { return outputName; } }

        public void Build(string[] args)
        {
            int i = 0;
            while (i < args.Length)
            {
                if (args[i] == "-f")
                {
                    logByDir = true;
                }
                if (args[i] == "-p")
                {
                    if (i + 1 < args.Length) 
                    {
                        i++;
                        if (args[i].StartsWith("-"))
                        {
                            return;
                        }
                        sourceName = args[i];
                        if (Path.GetExtension(args[i]) == ".zip")
                        {
                            isZip = true;
                            outputName = Path.GetFileNameWithoutExtension(args[i]) + DateTime.Now.ToString("_yyyyMMdd_HHmmss");
                        }
                        else
                        {
                            isZip = false;
                            string rgxDrvPath = @"\\.+\\";
                            outputName = args[i].TrimEnd('\\');
                            Match match = Regex.Match(outputName, rgxDrvPath);
                            if (match.Success)
                            {
                                outputName = outputName.Substring(match.Index + match.Length);
                            }
                            else
                            {
                                outputName = outputName.Substring(args[i].IndexOf('\\') + 1);
                            }
                            outputName = outputName + DateTime.Now.ToString("_yyyyMMdd_HHmmss");
                        }
                    }
                }
                if (args[i] == "-v")
                {
                    logDetail = true;
                }
                i++;
            }

            if (logByDir)
            {
                outputName = outputName + "\\" + outputName;
            }
        }
    }
}
*/