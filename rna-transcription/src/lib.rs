#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        // unimplemented!("Construct new Dna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", dna);
        if let Some(first) = dna.find(|x| match x {
            'A' | 'T' | 'C' | 'G' => false,
            _ => true,
        }) {
            Err(first)
        } else {
            Ok(Dna(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> Rna {
        // unimplemented!("Transform Dna {:?} into corresponding Rna", self);
        let mut rna = self
            .0
            .chars()
            .map(|x| match x {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => todo!(),
            })
            .collect::<String>();
        Rna::new(rna.as_str()).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        // unimplemented!("Construct new Rna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", rna);
        if let Some(first) = rna.find(|x| match x {
            'A' | 'U' | 'C' | 'G' => false,
            _ => true,
        }) {
            Err(first)
        } else {
            Ok(Rna(rna.to_string()))
        }
    }
}
