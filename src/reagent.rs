use std::hash::{Hash, Hasher};
// Module: reagent
use std::io::Read;
use std::{fs, io};

#[derive(PartialEq, Clone, Eq)]
pub struct Reagent {
    pub score: Option<usize>,
    pub name: String,
    pub atoms: Vec<String>,
}

impl Hash for Reagent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.atoms.hash(state);
    }
}

impl std::fmt::Display for Reagent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\t{} ", self.name)?;
        let atoms: Vec<String> = self.atoms.iter().map(|s| s.to_string()).collect();
        writeln!(f, "{}", atoms.join(" "))
    }
}

impl std::fmt::Debug for Reagent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Reagent as std::fmt::Display>::fmt(self, f)
    }
}

pub fn load_reagents(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// pub fn validate_reagents(reagents: &Vec<Reagent>) -> bool {
//     false
// }

pub fn parse_reagents(contents: &str) -> (Option<Reagent>, Vec<Reagent>) {
    let mut reagents = Vec::new();
    let mut exitus = None;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let reagent_name: String = parts[0].to_string();
        let atoms: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        let reagent = Reagent {
            name: reagent_name.clone(),
            atoms,
            score: None,
        };

        if reagent_name == "Exitus-1" {
            exitus = Some(reagent);
        } else {
            reagents.push(reagent);
        }
    }

    (exitus, reagents)
}
