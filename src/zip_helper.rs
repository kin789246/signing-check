/*
using System;
using System.Collections.Generic;
using System.IO;
using System.IO.Compression;

namespace SigningCheck
{
    internal class ZipHelper
    {
        private string extractPath = string.Empty;
        private string zipName = string.Empty;
        private string logName = string.Empty;

        public ZipHelper(string extractPath, string zipFileName, string logName)
        {
            this.extractPath = extractPath;
            this.zipName = zipFileName;
            this.logName = logName;
        }

        public void ExtractFiles(List<string> extensions)
        {
            if (Directory.Exists(extractPath))
            {
                Directory.Delete(extractPath, true);
            }
            // Ensures that the last character on the extraction path
            // is the directory separator char.
            // Without this, a malicious zip file could try to traverse outside of the expected
            // extraction path.
            if (!extractPath.EndsWith(Path.DirectorySeparatorChar.ToString(), StringComparison.Ordinal))
                extractPath += Path.DirectorySeparatorChar;

            try
            {
                using (ZipArchive archive = ZipFile.OpenRead(zipName))
                {
                    foreach (ZipArchiveEntry entry in archive.Entries)
                    {
                        foreach (var ext in extensions)
                        {
                            if (entry.Name.EndsWith(ext, StringComparison.OrdinalIgnoreCase))
                            {
                                string desDir = Path.GetDirectoryName(entry.FullName);
                                if (desDir != null)
                                {
                                    desDir = Path.Combine(extractPath, desDir);
                                    if (!Directory.Exists(desDir))
                                    {
                                        Directory.CreateDirectory(desDir);
                                    }
                                    string desPath = Path.Combine(desDir, entry.Name);
                                    entry.ExtractToFile(desPath);
                                }
                                break;
                            }
                        }
                    }
                }
            }
            catch (Exception e)
            {
                Console.Error.WriteLine(e.ToString());
                LogWriter.Log(e.ToString(), logName);
                throw;
            }
        }
    }
}
*/