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
                    "Filled jug {} (max capacity {}) by adding {gallons_needed_to_fill} gallons.",
                    self.id, self.max_gallons
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
                    "Emptied jug {} (max capacity {}) by dumping all {} of its gallons.",
                    self.id, self.max_gallons, self.current_gallons
                ),
            ))
        }
    }

    /// "Pour" as much volume as possible from self to the other jug. Illegal if the target
    /// jug is already full orif the current jug is empty; returns None in those cases. In
    /// the success case, returns a new copy of both jugs and a string description of the
    /// operation.
    fn pour(self, target: Jug) -> Option<(Jug, Jug, String)> {
        if self.current_gallons == 0 || target.current_gallons == target.max_gallons {
            None
        } else {
            let target_capacity = target.max_gallons - target.current_gallons;
            let gallons_transferred = std::cmp::min(target_capacity, self.current_gallons);
            Some((
                Jug {
                    current_gallons: self.current_gallons - gallons_transferred,
                    ..self
                },
                Jug {
                    current_gallons: target.current_gallons + gallons_transferred,
                    ..target
                },
                format!(
                    "Poured {gallons_transferred} gallons from jug {} (max capacity {}) \
                    to jug {} (max capacity {}).",
                    self.id, self.max_gallons, target.id, target.max_gallons
                ),
            ))
        }
    }
}

fn main() {
    let jugs = vec![Jug::new(1, 3), Jug::new(2, 5)];
    // Note: it's impossible for us to have already achieved our goal before taking any
    // actions, because every jug starts out empty (and the goal is nonzero).
    let goal_num_gallons = 4;

    let already_seen_states = HashSet::from([jugs.clone()]);
    // the collection of strings has in-order descriptions of every operation we've performed
    let mut queue: VecDeque<(Vec<Jug>, Vec<String>)> = VecDeque::from([(jugs, Vec::new())]);

    while let Some((jugs, operations)) = queue.pop_front() {
        for source_i in 0..jugs.len() {
            let source_jug = &jugs[source_i];
            if let Some((filled_source, fill_description)) = source_jug.fill() {
                let mut operations = operations.clone();
                operations.push(fill_description);
                if filled_source.current_gallons == goal_num_gallons {
                    return operations;
                }
                let jugs = jugs
                    .iter()
                    .enumerate()
                    .map(|(i, jug)| if i == source_i { filled_source } else { *jug })
                    .collect::<Vec<_>>();
                queue.push_back((jugs, operations));
            }
            // TODO try emptying
            for target_jug in jugs {
                if source_jug != target_jug {
                    // TODO try transferring
                }
            }
        }

        // TODO always check for winner and do visited addition? or is the code crazy?
        //   one option: helpers for making new (jugs, operations) for fill, empty, xfer
    }
}
