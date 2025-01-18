use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Jug {
    id: u32, // a numeric identifier; ideally unique
    max_gallons: u32,
    current_gallons: u32, // how much 'water' is in the jug right now?
}

impl Jug {
    /// Create a new Jug. New jugs start empty.
    fn new(id: u32, max_gallons: u32) -> Jug {
        Jug {
            id,
            max_gallons,
            current_gallons: 0,
        }
    }

    /// "Fill" the jug with "water", as if from some external source. Illegal if the
    /// jug is already full; returns None in that case. In the success case, returns
    /// a new Jug and a string description of the operation.
    fn fill(self) -> Option<(Jug, String)> {
        if self.current_gallons == self.max_gallons {
            None
        } else {
            let gallons_needed_to_fill = self.max_gallons - self.current_gallons;
            Some((
                Jug {
                    current_gallons: self.max_gallons,
                    ..self
                },
                format!(
                    "Filled jug #{} by adding {} gallon{}.",
                    self.id,
                    gallons_needed_to_fill,
                    if gallons_needed_to_fill == 1 { "" } else { "s" }
                ),
            ))
        }
    }

    /// "Dump" the jug's water out, as if onto the ground. Illegal if the jug was already
    /// empty; returns None in that case. In the success case, returns a new Jug and a
    /// string description of the operation.
    fn dump(self) -> Option<(Jug, String)> {
        if self.current_gallons == 0 {
            None
        } else {
            Some((
                Jug {
                    current_gallons: 0,
                    ..self
                },
                format!(
                    "Emptied jug #{} by dumping out {} gallon{}.",
                    self.id,
                    self.current_gallons,
                    if self.current_gallons == 1 { "" } else { "s" }
                ),
            ))
        }
    }

    /// "Pour" as much volume as possible from self to the other jug. Illegal if the target
    /// jug is already full, the current jug is empty, or the jugs are the same (even sharing
    /// the same ID). Returns None in those cases. In the success case, returns a new copy of
    /// both jugs and a string description of the operation.
    fn pour_into(self, target: Jug) -> Option<(Jug, Jug, String)> {
        if self.current_gallons == 0
            || target.current_gallons == target.max_gallons
            || self == target
        {
            None
        } else {
            let target_capacity = target.max_gallons - target.current_gallons;
            let gallons_transferred = std::cmp::min(target_capacity, self.current_gallons);
            let source_gallons_after = self.current_gallons - gallons_transferred;
            let target_gallons_after = target.current_gallons + gallons_transferred;
            Some((
                Jug {
                    current_gallons: source_gallons_after,
                    ..self
                },
                Jug {
                    current_gallons: target_gallons_after,
                    ..target
                },
                format!(
                    "Poured {} gallon{} from jug #{} to jug #{}, leaving \
                    {} gallon{} in jug #{} and {} gallon{} in jug #{}.",
                    gallons_transferred,
                    if gallons_transferred == 1 { "" } else { "s" },
                    self.id,
                    target.id,
                    source_gallons_after,
                    if source_gallons_after == 1 { "" } else { "s" },
                    self.id,
                    target_gallons_after,
                    if target_gallons_after == 1 { "" } else { "s" },
                    target.id
                ),
            ))
        }
    }

    fn insert_at_index(&self, insert_at_i: usize, jugs: &Vec<Jug>) -> Vec<Jug> {
        jugs.iter()
            .enumerate()
            .map(|(i, jug)| if i == insert_at_i { *self } else { *jug })
            .collect::<Vec<_>>()
    }

    fn insert_at_indices(
        first_jug: Jug,
        first_i: usize,
        second_jug: Jug,
        second_i: usize,
        jugs: &Vec<Jug>,
    ) -> Vec<Jug> {
        jugs.iter()
            .enumerate()
            .map(|(i, jug)| {
                if i == first_i {
                    first_jug
                } else if i == second_i {
                    second_jug
                } else {
                    *jug
                }
            })
            .collect::<Vec<_>>()
    }
}

fn main() {
    let jugs = vec![Jug::new(1, 3), Jug::new(2, 5)];
    // Note: it's impossible for us to have already achieved our goal before taking any
    // actions, because every jug starts out empty (and the goal is nonzero).
    let goal_num_gallons = 4;

    let maybe_shortest_path_operations = find_shortest_path(jugs, goal_num_gallons);
    match maybe_shortest_path_operations {
        Some(shortest_path_operations) => {
            println!(
                "Reached goal after {} steps!",
                shortest_path_operations.len()
            );
            shortest_path_operations
                .iter()
                .enumerate()
                .for_each(|(i, operation)| println!("{}. {operation}", i + 1));
        }
        None => println!("Unable to reach goal."),
    }
}

fn find_shortest_path(jugs: Vec<Jug>, goal_num_gallons: u32) -> Option<Vec<String>> {
    let mut already_seen_states = HashSet::from([jugs.clone()]);
    // the collection of strings has in-order descriptions of every operation we've performed
    let mut queue: VecDeque<(Vec<Jug>, Vec<String>)> = VecDeque::from([(jugs, Vec::new())]);

    while let Some((jugs, operations)) = queue.pop_front() {
        for source_i in 0..jugs.len() {
            let source_jug = &jugs[source_i];

            // Try filling the jug
            if let Some((filled_source, fill_description)) = source_jug.fill() {
                let mut operations = operations.clone();
                operations.push(fill_description);
                if filled_source.current_gallons == goal_num_gallons {
                    return Some(operations);
                }
                let jugs = filled_source.insert_at_index(source_i, &jugs);
                if !already_seen_states.contains(&jugs) {
                    queue.push_back((jugs.clone(), operations));
                    already_seen_states.insert(jugs);
                }
            }

            // Try emptying the jug
            if let Some((emptied_source, empty_description)) = source_jug.dump() {
                let mut operations = operations.clone();
                operations.push(empty_description);
                // No need to check for win condition here -- 0 gallons is invalid for winning
                let jugs = emptied_source.insert_at_index(source_i, &jugs);
                if !already_seen_states.contains(&jugs) {
                    queue.push_back((jugs.clone(), operations));
                    already_seen_states.insert(jugs);
                }
            }

            // Try transferring between this jug and every other one
            for (target_i, target_jug) in jugs.iter().enumerate() {
                if let Some((reduced_source, increased_target, pour_description)) =
                    source_jug.pour_into(*target_jug)
                {
                    let mut operations = operations.clone();
                    operations.push(pour_description);
                    if reduced_source.current_gallons == goal_num_gallons
                        || increased_target.current_gallons == goal_num_gallons
                    {
                        return Some(operations);
                    }
                    let jugs = Jug::insert_at_indices(
                        reduced_source,
                        source_i,
                        increased_target,
                        target_i,
                        &jugs,
                    );
                    if !already_seen_states.contains(&jugs) {
                        queue.push_back((jugs.clone(), operations));
                        already_seen_states.insert(jugs);
                    }
                }
            }
        }
    }
    None
}
