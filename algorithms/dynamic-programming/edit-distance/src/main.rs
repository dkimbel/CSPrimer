use edit_distance::edit_distance_recursive;
use std::env;

const USAGE: &str = "Usage: edit-distance <from> <to>";

fn main() {
    let mut args = env::args().skip(1);
    let from = args.next().expect(USAGE);
    let to = args.next().expect(USAGE);
    if let Some(_) = args.next() {
        panic!("{}", USAGE);
    }
    let solution = edit_distance_recursive(&from, &to);
    println!("Edit distance from {from} to {to}: {solution}");
}
