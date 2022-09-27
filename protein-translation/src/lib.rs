use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    dict: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn new(dict: HashMap<&'a str, &'a str>) -> Self {
        Self { dict }
    }
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        // unimplemented!(
        //     "Return the protein name for a '{}' codon or None, if codon string is invalid",
        //     codon
        // );
        self.dict.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        // unimplemented!("Return a list of protein names that correspond to the '{}' RNA string or None if the RNA string is invalid", rna);
        use std::str;

        rna.as_bytes()
            .chunks(3)
            .map(|w| str::from_utf8(w).unwrap())
            .map(|x| self.name_for(x))
            .take_while(|&x| x != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    // unimplemented!(
    //     "Construct a new CodonsInfo struct from given pairs: {:?}",
    //     pairs
    // );
    let mut info = HashMap::<&'a str, &'a str>::new();
    for (codon, name) in pairs {
        info.insert(codon, name);
    }
    CodonsInfo::new(info)
}
