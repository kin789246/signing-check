/*
using System;
using System.Collections.Generic;
using System.IO;
using System.Text.RegularExpressions;

namespace SigningCheck
{
    internal static class DumpParser
    {
        internal static void ParseDump(string raw, List<SigcheckData> sigcheckDatas, string drvPath)
        {
            StringReader sr = new StringReader(raw);
            string line = sr.ReadLine();
            string rgxPath = @"^[a-zA-Z]\:\\.+\..{3}";
            string rgxOS = @"OS: *";
            string rgxType = @"SigningType: *";

            SigcheckData sc = new SigcheckData();
            while (line != null)
            {
                if (Regex.IsMatch(line, rgxPath))
                {
                    if (string.Equals(Path.GetExtension(line), ".cat", StringComparison.OrdinalIgnoreCase))
                    {
                        sc = new SigcheckData();
                        sigcheckDatas.Add(sc);
                        sc.FileName = line;
                        var idx = line.IndexOf(drvPath, StringComparison.OrdinalIgnoreCase);
                        if (idx != -1)
                        {
                            sc.FileName = line.Substring(idx + drvPath.Length);
                        }
                    }
                }
                else if (Regex.IsMatch(line, rgxOS))
                {
                    string os = "OS: ";
                    if (line.Contains(os))
                    {
                        sc.OsSupport = line.Substring(line.IndexOf(os) + os.Length);
                    }
                }
                else if (Regex.IsMatch(line, rgxType))
                {
                    string st = "SigningType: ";
                    if (line.Contains(st))
                    {
                        sc.SigningType = line.Substring(line.IndexOf(st) + st.Length);
                    }
                }
                line = sr.ReadLine();
            }
        }
    }
}


//string rgx = @"OS: *";
//if (Regex.IsMatch(e.Data, rgx))
//{
//    outputlog.Append(e.Data).AppendLine();
//}

//SigningType: PreProd

/*
    dump cat: sigcheck.exe -d -s *.cat

    D:\catdllsys\files\intcpmt.cat
    HWID1: pci\ven_8086&dev_7d0d
    HWID2: pci\ven_8086&dev_ad0d
    OS: _v100,_v100_X64,Server_v100_X64,Server_v100_ARM64
    SigningType: PreProd
    PackageId: 3daf5a2b-748c-44d8-a621-45d25a3a4058
    BundleID: Driver
    Submission ID:  

    D:\catdllsys\files\ishheciextensiontemplate_att.cat
    HWID1: {95511210-d1f0-4091-b373-46fdcc5329f7}\ish_heci
    OS: _v100_X64_21H2,_v100_X64_22H2,_v100_X64_24H2
    Declarative: True
    Universal: False
    BundleID: ISH
    Submission ID: 29990010_14368141040126453_1152921505697749527
 */
*/