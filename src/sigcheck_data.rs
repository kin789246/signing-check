/*
using System.Collections.Generic;
using System.Text;

namespace SigningCheck
{
    public class SignerData
    {
        private string name;
        //enhanced key usage
        //LifetimeSigning (1.3.6.1.4.1.311.10.3.13) = test-siging
        //AttestationSigning (1.3.6.1.4.1.311.10.3.5.1) 
        private List<string> validUsages;
        private string signingDate;
        private string validFrom;
        private string validTo;
        private bool validated;
        public SignerData()
        {
            name = string.Empty;
            validUsages = new List<string>();
            signingDate = string.Empty;
            validFrom = string.Empty;  
            validTo = string.Empty;
            validated = false;
        }
        public string Name { get => name; set => name = value; }
        public List<string> ValidUsages { get => validUsages; set => validUsages = value; }
        public string SigningDate { get => signingDate; set => signingDate = value; }
        public string ValidFrom { get => validFrom;  set => validFrom = value; }
        public string ValidTo { get => validTo; set => validTo = value; }
        public bool Validated { get => validated; set => validated = value; }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append(name);
            sb.Append('{');
            foreach (var usage in validUsages)
            {
                sb.Append(usage).Append('+');
            }
            sb.Remove(sb.Length - 1, 1); //trim '+'

            sb.Append("||signing:")
                .Append(signingDate)
                .Append("||from:")
                .Append(validFrom)
                .Append("||to:")
                .Append(validTo)
                .Append('}');
            return sb.ToString();
        }
    }
    public class SigcheckData
    {
        private string fileName = string.Empty;
        private string signingType = string.Empty;
        private string osSupport = string.Empty;
        private List<SignerData> signers = new List<SignerData>();

        public string FileName { get => fileName; set => fileName = value; }
        public string OsSupport { get => osSupport; set => osSupport = value; }
        public string SigningType { get => signingType; set => signingType = value; }
        public List<SignerData> Signers { get => signers; set => signers = value; }
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append(fileName) .Append(',')
                .Append(signingType).Append(',')
                .Append(osSupport).Append(',');

            foreach (var item in signers)
            {
                sb.Append(item.ToString()).Append('-');
            }
            return sb.ToString().TrimEnd('-');
        }
    }
}
*/