/*
using System;
using System.IO;
using System.Text;

namespace SigningCheck
{
    public static class HtmlHelper
    {
        private const string tagBody = @"<body>";
        private const string endTagBody = @"</body>";
        private const string tagH1 = @"<h1>";
        private const string endTagH1 = @"</h1>";
        private const string tagTable = @"<table>";
        private const string endTagTable = @"</table>";
        private const string tagTh = @"<th>";
        private const string endTagTh = @"</th>";
        private const string tagTr = @"<tr>";
        private const string endTagTr = @"</tr>";
        private const string tagTd = @"<td>";
        private const string endTagTd = @"</td>";
        private const string tagStyle = @"<style>";
        private const string tagScript = @"<script>";

        private const string styleName = "config\\style.css";
        private const string scriptName = "config\\main.js";

        private static void addStringToTd(StringBuilder sb, string str, string className="")
        {
            if (className != "") {
                sb.Append("<td class='" + className + "'>");
            }
            else
            {
                sb.Append(tagTd);
            }
            sb.Append(str);
            sb.Append(endTagTd);
        }

        private static void addOToTd(StringBuilder sb, bool support)
        {
            if (support) { addStringToTd(sb, "O"); }
            else { addStringToTd(sb, ""); }
        }
        private static string makeTable(CsvOutData data)
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("<div id='summary-block'>\n");
            sb.Append(tagH1).Append(data.Summary).Append(endTagH1).Append("\n</div>\n");
            //div for display options
            sb.Append("<div id='displayOptions'></div>\n");
            //table part
            sb.Append("<div id='results'>\n");
            sb.Append(tagTable).Append("\n");
            sb.Append(tagTr);
            //NO.,Name,Summary,Path,PreProd,Attestation,TestSigning,WHQL,24H2,22H2,21H2,OtherOS,Expiry,Signers{Signer||ValidUsages||SigningDate||ValidFrom||ValidTo}
            sb.Append("<th class='sticky'>No.").Append(endTagTh);
            string[] adjustable = ["Path", "Other", "Signers"];
            string[] stickys = ["Name", "Summary"];
            foreach (var item in data.Title.Split(','))
            {
                string addClass = "";
                string addId = "";
                foreach (var adjust in adjustable)
                {
                    if (item.Contains(adjust))
                    {
                        addClass = " class='" + adjust + "-column'";
                        break;
                    }
                }
                foreach (var sticky in stickys)
                {
                    if (item.Contains(sticky))
                    {
                        addClass = " class='sticky'";
                        addId = " id='" + sticky + "-column'";
                        break;
                    }
                }
                
                sb.Append("<th" + addId + addClass + ">");
                sb.Append(item).Append(endTagTh);
            }
            sb.Append(endTagTr).Append("\n");
            int i = 1;
            foreach (var csvData in data.Data)
            {
                sb.Append(tagTr);
                addStringToTd(sb, i.ToString(), "sticky"); i++;
                addStringToTd(sb, csvData.Name, "sticky");
                addStringToTd(sb, csvData.Summary, "sticky");
                addStringToTd(sb, csvData.FilePath, "Path-column");
                addOToTd(sb, csvData.PreProd);
                addOToTd(sb, csvData.Attestation);
                addOToTd(sb, csvData.TestSigning);
                addOToTd(sb, csvData.Whql);
                foreach (var os in csvData.OsVersion)
                {
                    addOToTd(sb, os.Item2);
                }
                addStringToTd(sb, csvData.OtherOS, "Other-column");
                addStringToTd(sb, csvData.TsExpiryDate);
                string preStr = string.Empty;
                foreach (var signer in csvData.SigcheckData.Signers)
                {
                    preStr += signer.Name + " {";
                    foreach (var vu in signer.ValidUsages)
                    {
                        preStr += vu + ", ";
                    }
                    preStr += "date:" + signer.SigningDate + "||from:" + signer.ValidFrom + "||to:" + signer.ValidTo + "} ";
                }
                addStringToTd(sb, preStr, "Signers-column");
                sb.Append(endTagTr).Append("\n");
            }
            sb.Append(endTagTable).Append("\n</div>\n");
            return sb.ToString();

        }
        private static string readFrom(string name)
        {
            StringBuilder sb = new StringBuilder();
            using (StreamReader sr = new StreamReader(name))
            {
                string line = sr.ReadLine();
                while (line != null)
                {
                    sb.Append(line).Append("\n");
                    line = sr.ReadLine();
                }
            }
            return sb.ToString();
        }
        public static string ToHtmlTable(CsvOutData data)
        {
            StringBuilder sb = new StringBuilder();
            string reportT = "config\\report";
            using (StreamReader sr = new StreamReader(reportT))
            {
                try
                {
                    string line = sr.ReadLine();
                    while (line != null)
                    {
                        sb.Append(line).Append("\n");
                        if (line.Trim().Equals("<title>", StringComparison.OrdinalIgnoreCase))
                        {
                            sb.Append(Program.Version + "\n");
                        }
                        if (line.Trim().Equals(tagStyle, StringComparison.OrdinalIgnoreCase))
                        {
                            sb.Append(readFrom(styleName));
                        }
                        if (line.Trim().Equals(tagBody, StringComparison.OrdinalIgnoreCase))
                        {
                            sb.Append(makeTable(data));
                        }
                        if (line.Trim().Equals(tagScript, StringComparison.OrdinalIgnoreCase))
                        {
                            sb.Append(readFrom(scriptName));
                        }
                        
                        line = sr.ReadLine();
                    }
                }
                catch (Exception e)
                {
                    Console.WriteLine(e.ToString());
                }
            }

            return sb.ToString();
        }
    }
}

/*
    <!DOCTYPE html>
    <html>
    <head>
    <style>
    table, th, td {
        border-style: solid;
        border-width: 1px;
        border-collapse: collapse;
    }
    </style>
    </head>
    <body>

    <h1>summary line</h1>

    <table>
      <tr>
        <th>name</th>
        <th>summary</th>
        <th>path</th>
      </tr>
      <tr>
        <td>acxdac.cat</td>
        <td>WHQL signed</td>
        <td>\INT_VGA_MTL_101.5522_WHQL_22H2\Driver\dchu_5522</td>
      </tr>
      <tr>
        <td>acxdac.cat</td>
        <td>WHQL signed</td>
        <td>\INT_VGA_MTL_101.5522_WHQL_22H2\Driver\dchu_5522</td>
      </tr>
    </table>

    </body>
    </html>
 */
*/