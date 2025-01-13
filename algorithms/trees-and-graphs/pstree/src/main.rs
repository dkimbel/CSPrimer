use std::env;
use std::env::Args;

mod process_tree_filter;
mod process_tree_parser;
mod process_tree_printer;

fn main() {
    // Optionally, the caller will have given us a string to filter processes by.
    let filter_processes_by_text = parse_args(env::args());

    let all_processes_tree = process_tree_parser::execute_ps_and_parse();
    let all_processes_root = &all_processes_tree.get_root();

    let parent_pids_to_child_processes = if let Some(filter_text) = &filter_processes_by_text {
        // If we were given text to filter processes by, create a new filtered tree.
        process_tree_filter::run(
            filter_text,
            all_processes_root,
            &all_processes_tree.all_parent_pids_to_child_processes,
        )
    } else {
        // Otherwise just use the full tree.
        all_processes_tree.all_parent_pids_to_child_processes
    };

    process_tree_printer::print(
        &parent_pids_to_child_processes,
        all_processes_tree.max_num_pid_chars,
        filter_processes_by_text,
    );
}

fn parse_args(args: Args) -> Option<String> {
    let mut skipped = args.skip(1); // skip zeroth arg, which is path to program
    let filter_processes_by_text = skipped.next();
    if skipped.next().is_some() {
        panic!(
            "Only one argument is allowed; it will be used to filter the displayed processes. \
            To filter by a phrase containing whitespace, enclose the phrase in quotation marks."
        )
    }
    filter_processes_by_text
}

const ROOT_PARENT_PID: usize = 0;

#[derive(Clone)]
pub struct Process {
    pub pid: usize,  // process ID
    pub pgid: usize, // process group ID
    pub user: String,
    pub command: String,
}

impl PartialEq for Process {
    /// Equality should only depend on PID, not on string values. We might have one copy of
    /// a process that has had some ANSI color codes added to its strings, and another copy
    /// without color codes; they should still be 'equal' if their PIDs match.
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}
