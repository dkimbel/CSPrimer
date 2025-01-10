use regex::Regex;
use std::collections::HashMap;
use std::process::Command;
use std::sync::OnceLock;

// Minor optimization to only compile our regex one time throughout life of program
static PROCESS_LINE_REGEX: OnceLock<Regex> = OnceLock::new();

/// Chars that will be printed to the screen to reflect the structure of the tree.
/// R = Right, L = Left, T = Top, B = Bottom. So e.g. RL is a dash-like char that
/// extends from left to right.
enum TreeChar {
    RL,
    RBL,
    TRB,
    TB,
    TR,
}

impl TreeChar {
    fn to_char(&self) -> char {
        use TreeChar::*;
        match self {
            RL => '─',
            RBL => '┬',
            TRB => '├',
            TB => '│',
            TR => '└',
        }
    }
}

// Any given Process's ChildPosition is that process's position relative to its own immediate
// parent. So if Process id 10 is the third of four children of Process id 2, then Process id
// 10 is a MiddleChild. Process id 10 may or may not have children of its own -- that isn't
// relevant here.
#[derive(Clone, Copy, PartialEq)]
enum ChildPosition {
    MiddleChild, // includes first child
    LastChild,
}

// Formulated to "make illegal state unrepresentable" -- there is no way to represent any
// given Process as not being a child, yet having parents.
#[derive(Clone, Copy)]
enum ChildStatus<'a> {
    NotChild,
    IsChild {
        position: ChildPosition,
        // if this is empty, then the relevant process only has one parent: the root.
        non_root_parent_child_positions: &'a Vec<ChildPosition>,
    },
}

struct Process {
    pid: u64,
    user: String,
    args: String,
}

struct ProcessStackNode<'a> {
    process: &'a Process,
    maybe_child_position: Option<ChildPosition>, // None for root node
}

impl Process {
    /// Create a Process from a line outputted by a `ps` command using a particular format.
    /// Return a tuple of the new Process and its parent PID. (Keeping the parent PID
    /// separate from the struct is just a slight optimization to avoid storing extra copies
    /// of it unnecessarily, when we put the struct into a map with parent PID as the key.)
    fn from_ps_line(line: &str) -> (Self, u64) {
        let re = PROCESS_LINE_REGEX
            // example line: "  391     1 root             /usr/libexec/keybagd -t 15"
            .get_or_init(|| Regex::new(r"^\s*(\d+)\s+(\d+)\s+(\w+)\s+(.*?)\s*$").unwrap());
        let captures = re
            .captures(line)
            .expect(&format!("Failed to parse line from ps: {line}"));

        let parent_pid = captures[2]
            .parse::<u64>()
            .expect("failed to parse ppid as integer");

        (
            Self {
                pid: captures[1]
                    .parse::<u64>()
                    .expect("failed to parse pid as integer"),
                user: captures[3].to_string(),
                args: captures[4].to_string(),
            },
            parent_pid,
        )
    }

    fn print_recursive(
        &self,
        max_num_pid_chars: usize,
        child_status: ChildStatus,
        parent_pids_to_child_processes: &HashMap<u64, Vec<Process>>,
    ) {
        let maybe_children = parent_pids_to_child_processes.get(&self.pid);
        let is_parent = maybe_children.is_some_and(|children| !children.is_empty());

        // do the actual printing
        let tree_chars = self.get_tree_chars(is_parent, child_status);
        let Self { pid, user, args } = self;
        println!("{tree_chars} {pid:0max_num_pid_chars$} {user}");

        // recursively print all children of the current process
        if let Some(children) = maybe_children {
            // Each process must know the 'child position' of ALL of its parents, to know
            // whether to print whitespace/indentation (parent was a last child) or a
            // top-to-bottom char (parent was a middle child, and so has more processes
            // below it at the same indentation level). To support this need, maintain
            // a running list of parent 'child positions'.
            let non_root_parent_child_positions: Vec<ChildPosition> = match child_status {
                ChildStatus::NotChild => Vec::new(),
                ChildStatus::IsChild {
                    position,
                    non_root_parent_child_positions,
                } => {
                    let mut v = non_root_parent_child_positions.clone();
                    v.push(position);
                    v
                }
            };

            for (i, child_process) in children.iter().enumerate() {
                // The child needs to know whether it is itself a 'middle' or 'last' child;
                // this affects the tree chars it prints.
                let childs_child_position = if i + 1 == children.len() {
                    ChildPosition::LastChild
                } else {
                    ChildPosition::MiddleChild
                };
                let childs_child_status = ChildStatus::IsChild {
                    position: childs_child_position,
                    non_root_parent_child_positions: &non_root_parent_child_positions,
                };
                Self::print_recursive(
                    child_process,
                    max_num_pid_chars,
                    childs_child_status,
                    parent_pids_to_child_processes,
                );
            }
        }
    }

    fn get_tree_chars(&self, is_parent: bool, child_status: ChildStatus) -> String {
        match child_status {
            ChildStatus::NotChild => {
                let middle_tree_char = if is_parent {
                    TreeChar::RBL
                } else {
                    TreeChar::RL
                };
                return [TreeChar::RL, middle_tree_char, TreeChar::RL]
                    .iter()
                    .map(|tc| tc.to_char())
                    .collect::<String>();
            }
            ChildStatus::IsChild {
                position,
                non_root_parent_child_positions,
            } => {
                let mut s = String::from(' ');

                // We need to indent this Process further based on how many parents
                // it has. We might also need to draw some top-to-bottom lines on
                // behalf of those parents, to reach their further-down siblings.
                for parent_child_position in non_root_parent_child_positions {
                    let position_char = match parent_child_position {
                        ChildPosition::MiddleChild => TreeChar::TB.to_char(),
                        ChildPosition::LastChild => ' ',
                    };
                    s.push(position_char);
                    s.push(' ');
                }

                // add the final tree characters, which may look like the
                // following example (among others): └─┬─
                let position_tree_char = match position {
                    ChildPosition::MiddleChild => TreeChar::TRB,
                    ChildPosition::LastChild => TreeChar::TR,
                };
                let branch_to_children_tree_char = match is_parent {
                    true => TreeChar::RBL,
                    false => TreeChar::RL,
                };
                let final_chars = [
                    position_tree_char,
                    TreeChar::RL,
                    branch_to_children_tree_char,
                    TreeChar::RL,
                ]
                .map(|tc| tc.to_char());
                s.extend(final_chars);
                return s;
            }
        }
    }
    // TODO figure out how to limit the number of chars in 'args' to what the terminal will allow, then
    //   re-introduce args to my output
    // TODO Colorize lines? Primary colors and orange, perhaps?
    // TODO Try implementing 'only show lines that match specific text'
    //   this could be done via a first-pass search through the tree where we use a single mutable vec
    //   to track which parents we're currently searching under. when we find a hit, we "merge" our current
    //   path into a parent-pid-to-process dict representing the tree so far. For extra optimization, do the
    //   'merge' in a batch at the end of the current list of children we're working through (maybe detected
    //   by indentation level dropping?), instead of immediately on finding a match.
    // TODO Note how I could have done printing in the same pass as parsing, since inputs were pre-sorted.
    //   But, keeping the processes separate will let us implement 'only show lines that show text' without
    //   disrupting the codebase much.
    // TODO Maybe find a way to get wider text length as output from ps?
    // TODO Align by username length within a level/batch?
    // TODO Any refactor / code cleanup?
    //   - could I possibly have reusable 'tree search' code that takes some kind of 'action' as an
    //     input? that action could be 'print', or it could be 'check for text match and merge into tree'.
    //     - could I at least have a StackSearch struct that keeps the mutable stack as an attr, and so
    //       can easily have separate helper functions for 'handle parent pop' and 'handle print'? BUT,
    //       I don't know how well that would work in Rust... would I have to always have mutable refs
    //       on both lists from my helpers, preventing me from handing out an immutable ref to the
    //       process printing fn?
    //   - try to be guided by common arguments not having to be passed to functions, because they belong
    //     to a relevant struct already?
    //   - maybe have a ProcessPrinter that keeps track of max_num_process_chars for us?
    //   - generally split out parsing of the full process list into its own function / struct?
    //   - do I reasonably need to split into multiple files?
    // TODO add/update comments? add missing docstrings?
    // TODO Add README featuring a screenshot
    // TODO Consider `hyperfine` for benchmarking vs pstree?
    // TODO Submit, mentioning screenshot in README or direct-linking it
}

fn main() {
    let ps_stdout_bytes = Command::new("ps")
        .args(["-axo", "pid,ppid,user,args"])
        .output()
        .expect("ps command failed")
        .stdout;
    let ps_stdout = String::from_utf8(ps_stdout_bytes).expect("ps failed to output valid utf-8");

    // To model a tree (graph where every child can have only one parent), we use a map
    // of parent PID to process instance. We could do something more elaborate where each
    // Process owns a Vec<Process> of its children, but that isn't necessary.
    let mut parent_pids_to_child_processes: HashMap<u64, Vec<Process>> = HashMap::new();
    let mut max_pid = 0;

    // We use skip(1) to skip the first line, which just contains headers.
    for ps_line in ps_stdout.lines().skip(1) {
        let (process, parent_pid) = Process::from_ps_line(ps_line);
        // Technically we don't HAVE to call `max`, lines are already sorted by PID.
        max_pid = std::cmp::max(max_pid, process.pid);

        parent_pids_to_child_processes
            .entry(parent_pid)
            .or_insert_with(Vec::new)
            .push(process);
    }

    // We'll want to left-pad every printed PID with zeroes until it matches the length
    // of the largest PID.
    let max_num_pid_chars = format!("{max_pid}").len();

    // The root process will always be the one and only process with a parent PID of 0.
    let root: &Process = &parent_pids_to_child_processes.get(&0).unwrap()[0];

    Process::print_recursive(
        root,
        max_num_pid_chars,
        ChildStatus::NotChild,
        &parent_pids_to_child_processes,
    );
}
