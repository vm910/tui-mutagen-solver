use crate::combinator::Combinator;
use crate::reagent::Reagent;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use std::collections::HashSet;

const MAX_DEPTH: usize = 15;

pub fn filter_useless_reagents(exitus: &Reagent, reagents: &Vec<Reagent>) -> Vec<Reagent> {
    let mut prev_len = reagents.len() + 1;
    let mut reagents = reagents.clone();

    while prev_len > reagents.len() {
        prev_len = reagents.len();
        let atom_pool: HashSet<String> = reagents
            .iter()
            .flat_map(|r| r.atoms.iter().cloned())
            .collect();

        reagents.retain(|Reagent { atoms, .. }| {
            atoms.iter().all(|atom| {
                atom.starts_with('-')
                    || exitus.atoms.contains(atom)
                    || atom_pool.contains(&format!("-{}", atom))
            })
        });
    }

    reagents
}

fn contains_ordered_slice(sequence: &[String], slice: &[String]) -> bool {
    let l = slice.len();

    for i in 0..sequence.len() - l + 1 {
        if sequence[i..i + l] == slice[..] {
            return true;
        }
    }

    false
}

pub fn get_viable_start_reagents(exitus: &Reagent, reagents: &[Reagent]) -> Vec<Reagent> {
    let mut viable_starts: Vec<Reagent> = Vec::new();

    for reagent in reagents {
        let mut score: usize = 0;

        let mut j = 0;
        while contains_ordered_slice(&reagent.atoms, &exitus.atoms[0..j]) {
            score = j;
            j += 1;
        }

        if score > 0 {
            viable_starts.push(Reagent {
                name: reagent.name.clone(),
                atoms: reagent.atoms.clone(),
                score: Some(score),
            });
        }

        viable_starts.sort_by(|a, b| b.score.cmp(&a.score));
    }

    viable_starts
}

fn heuristic(current: &Combinator, exitus: &Reagent, depth: usize) -> OrderedFloat<f32> {
    let mut score = 0.0;
    let mut index_c: usize = 0;

    for i in 0..current.sequence.len() {
        if exitus.atoms.len() >= i && current.sequence[i] == exitus.atoms[i - index_c] {
            score += 3.0 / depth as f32;
        } else {
            score -= 1.0 * depth as f32;
            index_c += 1;
        }
    }

    OrderedFloat(score)
}

pub fn priority_search(
    exitus: &Reagent,
    start: &Reagent,
    reagents: &[Reagent],
) -> Option<Vec<String>> {
    let mut p_queue = PriorityQueue::new();
    let mut i = 0;
    let path = vec![start.name.clone()];

    let mut combinator = Combinator {
        sequence: Vec::new(),
        reagent_path: Vec::new(),
    };

    p_queue.push(
        (start.atoms.clone(), start.name.clone(), path.clone()),
        heuristic(&combinator, exitus, 1),
    );

    combinator.reagent_path.push(start.name.clone());

    while !p_queue.is_empty() && i < 2500 {
        let ((current, prev_name, current_path), _) = p_queue.pop().unwrap();

        if combinator.reagent_path.len() >= MAX_DEPTH {
            break;
        }

        for reagent in reagents {
            if reagent.name == prev_name {
                continue;
            }

            combinator.reset(&current, &current_path);
            let new_sequence = combinator.add_reagent(reagent);

            if combinator.sequence == exitus.atoms {
                return Some(combinator.reagent_path.clone());
            } else {
                let priority = heuristic(&combinator, exitus, combinator.reagent_path.len());
                let mut new_path = current_path.clone();
                new_path.push(reagent.name.clone());

                p_queue.push((new_sequence, reagent.name.clone(), new_path), priority);
            }
        }

        i += 1;
    }

    None
}
