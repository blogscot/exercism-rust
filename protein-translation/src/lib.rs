use std::collections::HashMap;

pub struct Proteins {
  data: HashMap<&'static str, &'static str>,
}

impl Proteins {
  pub fn name_for(&self, codon: &str) -> Result<&'static str, &'static str> {
    match self.data.get(codon) {
      Some(protein) => Ok(*protein),
      None => Err("No match found"),
    }
  }
  pub fn of_rna(&self, codons: &str) -> Result<Vec<&'static str>, &'static str> {
    let codons: Vec<String> = codons
      .chars()
      .map(|ch| ch.to_string())
      .collect::<Vec<_>>()
      .chunks(3)
      .map(|chunk| chunk.join(""))
      .collect();
    let mut proteins = Vec::new();
    for codon in codons {
      let protein = self.name_for(&codon)?;
      if protein == "stop codon" {
        return Ok(proteins);
      }
      proteins.push(protein);
    }
    Ok(proteins)
  }
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Proteins {
  let mut data = HashMap::new();
  for (key, value) in pairs {
    data.insert(key, value);
  }
  Proteins { data }
}
