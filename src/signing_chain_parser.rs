/*
using System;
using System.Collections.Generic;
using System.IO;
using System.Text.RegularExpressions;

namespace SigningCheck
{
    internal static class SigningChainParser
    {
        internal static void ParseSigningChain(string raw, List<SigcheckData> sigcheckDatas, string drvPath)
        {
            StringReader sr = new StringReader(raw);
            string rgxPath = @"^[a-zA-Z]\:\\.+\..{3}";
            string rgxDate = @"\d+\/\d+\/\d+";

            SigcheckData sc = new SigcheckData();
            SignerData signer = new SignerData();
            Match match;
            string line = sr.ReadLine();
            while (line != null)
            {
                if (Regex.IsMatch(line, rgxPath))
                {
                    line = line.TrimEnd(':');
                    if (string.Equals(Path.GetExtension(line), ".cat", StringComparison.OrdinalIgnoreCase) ||
                        string.Equals(Path.GetExtension(line), ".dll", StringComparison.OrdinalIgnoreCase) ||
                        string.Equals(Path.GetExtension(line), ".sys", StringComparison.OrdinalIgnoreCase))
                    {
                        var idx = line.IndexOf(drvPath, StringComparison.OrdinalIgnoreCase);
                        if (idx != -1)
                        {
                            line = line.Substring(idx + drvPath.Length);
                        }

                        sc = sigcheckDatas.Find(x => x.FileName == line);
                        if (sc == null)
                        {
                            sc = new SigcheckData();
                            sigcheckDatas.Add(sc);
                            sc.FileName = line;
                        }
                        line = sr.ReadLine();
                        while (!line.Contains("MachineType:") && line != null)
                        {
                            if (line.Contains("Signing date:"))
                            {
                                signer = new SignerData();
                                sc.Signers.Add(signer);
                                match = Regex.Match(line, rgxDate);
                                if (match.Success)
                                {
                                    signer.SigningDate = match.ToString();
                                }
                                else
                                {
                                    signer.SigningDate = "n/a";
                                }
                            }
                            if (line.Trim() == "Signers:" || line.Trim() == "Signer:")
                            {
                                line = sr.ReadLine();
                                signer.Name = line.Trim().Replace(',', ' ');
                                //read 8 lines
                                for (int i = 0; i < 8; i++)
                                {
                                    line = sr.ReadLine();
                                    string vu = "Valid Usage:";
                                    if (line.Contains(vu))
                                    {
                                        foreach (string v in line.TrimStart().Substring(vu.Length).Split(','))
                                        {
                                            signer.ValidUsages.Add(v.Trim());
                                        }
                                    }
                                    else if (line.Contains("Valid from:"))
                                    {
                                        match = Regex.Match(line, rgxDate);
                                        if (match.Success)
                                        {
                                            signer.ValidFrom = match.ToString();
                                        }
                                    }
                                    else if (line.Contains("Valid to:"))
                                    {
                                        match = Regex.Match(line, rgxDate);
                                        if (match.Success)
                                        {
                                            signer.ValidTo = match.ToString();
                                        }
                                        if (DateTime.Now.AddMonths(1) < DateTime.Parse(signer.ValidTo))
                                        {
                                            signer.Validated = true;
                                        }
                                    }
                                }
                            }
                            line = sr.ReadLine();
                        }
                    }
                }
                line = sr.ReadLine();
            }
        }
    }
}
/*
    singers: sigcheck.exe -i -s *.*
    
    Sigcheck v2.90 - File version and signature viewer
    Copyright (C) 2004-2022 Mark Russinovich
    Sysinternals - www.sysinternals.com
    
    D:\catdllsys\files\intcpmt.cat:
    	Verified:	Signed
    	File date:	?? 08:38 2024/5/18
    	Signing date:	?? 05:58 2023/12/20
    	Catalog:	D:\catdllsys\files\intcpmt.cat
    	Signers:
    	   Microsoft Windows Hardware Compatibility Publisher
    		Cert Status:	Valid
    		Valid Usage:	Lifetime Signing, 1.3.6.1.4.1.311.10.3.39, WHQL Crypto, Code Signing
    		Cert Issuer:	Microsoft Windows PCA 2010
    		Serial Number:	33 00 00 0A F9 23 46 84 B2 8D 04 7F D7 00 00 00 00 0A F9
    		Thumbprint:	3D31A30BAB5371D14DC060D6722D0C56A46E8E01
    		Algorithm:	sha256RSA
    		Valid from:	?? 03:18 2023/10/20
    		Valid to:	?? 03:18 2024/10/17
 */
*/