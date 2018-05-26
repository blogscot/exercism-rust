#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    rna: String,
}

impl RibonucleicAcid {
    pub fn new(rna: &str) -> Self {
        RibonucleicAcid { rna: rna.into() }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {
    dna: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(dna: &str) -> Self {
        DeoxyribonucleicAcid { dna: dna.into() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let rna = self.dna.chars().map(convert).collect::<String>();

        RibonucleicAcid { rna }
    }
}

fn convert(chr: char) -> char {
    match chr {
        'C' => 'G',
        'G' => 'C',
        'A' => 'U',
        'T' => 'A',
        _ => chr,
    }
}
