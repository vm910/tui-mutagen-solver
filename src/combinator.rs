use crate::reagent::Reagent;
#[derive(Clone)]
pub struct Combinator {
    pub sequence: Vec<String>,
    pub reagent_path: Vec<String>,
}

impl std::fmt::Display for Combinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\tReagent Path: ")?;
        for reagent in &self.reagent_path {
            write!(f, "{} ", reagent)?;
        }
        writeln!(f)?;

        write!(f, "\tSequence: ")?;
        for atom in &self.sequence {
            write!(f, "{} ", atom)?;
        }
        writeln!(f)
    }
}

impl std::fmt::Debug for Combinator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Combinator as std::fmt::Display>::fmt(self, f)
    }
}

impl Combinator {
    pub fn add_reagent(&mut self, reagent: &Reagent) {
        self.reagent_path.push(reagent.name.clone());

        for atom in &reagent.atoms {
            if atom.starts_with('-') {
                self.sequence.retain(|a| a != &atom[1..]);
            } else if !self.sequence.contains(atom) {
                self.sequence.push(atom.clone());
            }
        }
    }

    pub fn reset(&mut self, base_sequence: &[String], reagent_path: &[String]) {
        self.sequence = base_sequence
            .iter()
            .filter(|a| !a.starts_with('-'))
            .cloned()
            .collect();

        self.reagent_path = reagent_path.to_owned();
    }
}
