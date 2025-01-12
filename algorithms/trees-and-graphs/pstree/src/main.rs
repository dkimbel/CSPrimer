use std::env;
use std::env::Args;

mod process_tree_filter;
mod process_tree_parser;
mod process_tree_printer;

// TODO Any refactor / code cleanup?
//   - maybe have a ProcessPrinter that keeps track of max_num_process_chars for us, plus terminal
//     width and even hashmap of parent PIDs to child processes? But, big question: how to share code
//     between the ProcessPrinter and a ProcessSearcher used to filter for processes that match string?
//   - rename 'num_uncolored_chars'. It's super deceptive; many of the chars we're counting will appear
//     "colored" on screen. We mean 'num visible chars' or 'num non-ansi-escape chars'.
//   - can I split up `print_recursive`? It's quite long.
//   - can I clean up get_tree_chars at all? It's pretty long. (or otherwise clean up 'num ansi chars')
//     code, if that's still a thing?)
//     - maybe part of the solution: combine logic for root node and non-root? instead of hardcoding an
//       opening space for non-root case, just have a list of all 'child positions' (parent and self both),
//       and have each be a pair of ' ' + position_char.
//   - make sure 'get root from tree' logic isn't excessively duplicated. Related... make sure I'm not
//     unnecessarily passing root node references around. It makes me a little uncomfortable; I'd rather
//     look up the root from the tree 'just in time'. It also makes me pass in too many params to fns.
//     Solution: make a ProcessTree struct that has the root on its impl? Means renaming AllProcessesTree
//     struct to something new, but probably worth it.
//   - use a struct ('parameter grouping') to make recursive filter fn not have to take so many args.
//     Explain in a comment that the only args passed in are ones that affect the fn's behavior.
//   - only do pid == pgid check one place, not two?
//   - instead of making PrintRecursive know our filter text (lowercased), instead have it use a
//     HashSet of index ranges by pid, returned by our filter fn? In this case, flip the order of
//     filter fn's first conditional! And explain importance of ordering in a comment.
//   - any opportunities to leverage let/else?
// TODO add/update comments? add missing docstrings?
//   - add a comment on how I could have done printing in the same pass as parsing, since inputs
//     were pre-sorted. But just as well to keep that separate given optional filtering step.
//   - is there such a thing as a module-level comment with ///? Use it for each module if so.
//   - more?
// TODO Add README
//   - featuring screenshots and noting crossterm.
//     - screenshots to include comparison with `login` argument.
//   - emphasize that this is a re-implementation of pstree, maintaining the most important features to
//     me and adding color
//   - noting crossterm dependency for terminal width + colorized text
//   - describe `cargo build --release`, `./target/release/pstree`
// TODO how to properly 'install' my own pstree on my machine, to use it from any directory? Just add an
//   alias to ./Users/dk/Workspace/cs-primer/algorithms/trees-and-graphs/pstree/target/release/pstree?
// TODO Submit, mentioning screenshot in README or direct-linking it; also note crossterm for width/colors
fn main() {
    let filter_processes_by_text = parse_args(env::args());

    let all_processes_tree = process_tree_parser::execute_ps_and_parse();
    let all_processes_root = &all_processes_tree.get_root();

    // If we were given text to filter processes by, create a new filtered tree.
    let parent_pids_to_child_processes = if let Some(filter_text) = &filter_processes_by_text {
        process_tree_filter::run(
            filter_text,
            all_processes_root,
            &all_processes_tree.all_parent_pids_to_child_processes,
        )
    } else {
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
