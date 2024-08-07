use std::fmt;

#[derive(Clone)]
pub struct SignerData {
    pub name: String,
    //enhanced key usage
    //LifetimeSigning (1.3.6.1.4.1.311.10.3.13) = test-siging
    //AttestationSigning (1.3.6.1.4.1.311.10.3.5.1) 
    pub valid_usages: Vec<String>,
    pub signing_date: String,
    pub valid_from: String,
    pub valid_to: String,
    pub validated: bool
}

impl SignerData {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            valid_usages: Vec::new(),
            signing_date: String::new(),
            valid_from: String::new(),  
            valid_to: String::new(),
            validated: false,
        }
    }
}

impl fmt::Display for SignerData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sb: String = String::new();
        sb.push_str(&self.name);
        sb.push_str("{");
        self.valid_usages
            .iter()
            .for_each(|vu| {
                sb.push_str(&vu);
                sb.push('+');
            });
        sb = sb.trim_end_matches('+').to_string();
        sb.push_str("||signing:");
        sb.push_str(&self.signing_date);
        sb.push_str("||from:");
        sb.push_str(&self.valid_from);
        sb.push_str("||to:");
        sb.push_str(&self.valid_to);
        sb.push_str("}");
        write!(f, "{}", sb)
    }
}

#[derive(Clone)]
pub struct SigcheckData {
    pub file_name: String,
    pub signing_type: String,
    pub os_support: String,
    pub signers: Vec<SignerData>,
}

impl SigcheckData {
    pub fn new() -> Self {
        Self {
            file_name: String::new(),
            signing_type: String::new(),
            os_support: String::new(),
            signers: Vec::new(),
        }
    }
}

impl fmt::Display for SigcheckData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sb = String::new();
        sb.push_str(&self.file_name);
        sb.push_str(",");
        sb.push_str(&self.signing_type);
        sb.push_str(",");
        sb.push_str(&self.os_support);
        sb.push_str(",\n");
        self.signers
            .iter()
            .for_each(|s| {
                sb.push_str(&s.to_string());
                sb.push('\n');
            });
        write!(f, "{}", sb)
    }
}