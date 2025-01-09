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
#[derive(Clone, Copy, PartialEq, Eq)]
enum ChildPosition {
    MiddleChild, // includes first child
    LastChild,
}

// Formulated to "make illegal state unrepresentable" -- there is no way to represent any
// given Process as not being a child, yet having parents.
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
    // this will be None for the root node
    maybe_child_position: Option<ChildPosition>,
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

    /// Print the process to the screen, with appropriate indentation and tree-related
    /// characters.
    fn print(&self, max_num_pid_chars: usize, is_parent: bool, child_status: ChildStatus) {
        let tree_chars = self.get_tree_chars(is_parent, child_status);
        let Self { pid, user, args } = self;
        println!("{tree_chars} {pid:0max_num_pid_chars$} {user} {args}");
    }

    fn get_tree_chars(&self, is_parent: bool, child_status: ChildStatus) -> String {
        if indentation_level == 0 {
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

        let mut s = String::from(" ");

        // TODO first, based on indentation level, add some combination of space and '|'
        //   this comes down to whether the parent(s) were middle children or not, which is tough.
        //     - instead of passing in a numeric indentation level, pass in a list of parents' childstates.
        //       this can ignore root, and can be an immutable reference to a vec we otherwise mutate/clear.
        // TODO then, upon reaching final indentation, add:
        //   - TRB or TR, based on middle vs last child
        //   - a single RL
        //   - RL or RBL, based on whether isParent
        //   - a final RL

        let tree_char = match child_status {
            ChildStatus::NotChild if is_parent => TreeChar::RBL,
            ChildStatus::NotChild => TreeChar::RL,
            ChildStatus::MiddleChild if indentation_level == 1 => TreeChar::TRB,
            ChildStatus::MiddleChild => TreeChar::TB,
            ChildStatus::LastChild => TreeChar::TR,
        };
        s.push(tree_char.to_char());
        return String::from("");
    }
    // TODO implement tree char printing logic
    // TODO was `$` after max_num_pid_chars absolutely necessary?
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
    //   - try to be guided by common arguments not having to be passed to functions, because they belong
    //     to a relevant struct already?
    //   - maybe have a ProcessPrinter that keeps track of max_num_process_chars for us?
    //   - generally split out parsing of the full process list into its own function / struct?
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

    // Use a stack to effectively do depth-first search through our tree, printing
    // every process as we go. To print out our tree, we must also track:
    // - whether the process is a middle or last child of its parent
    // - whether each of the process's parents was a middle or last child
    let mut stack: Vec<ProcessStackNode> = vec![ProcessStackNode {
        process: root,
        maybe_child_position: None, // root is not a child
    }];

    let mut non_root_parent_child_positions: Vec<ChildPosition> = Vec::new();

    while let Some(ProcessStackNode {
        process,
        maybe_child_position,
    }) = stack.pop()
    {
        let maybe_children = parent_pids_to_child_processes.get(&process.pid);
        let is_parent = maybe_children.is_some_and(|children| !children.is_empty());

        let child_status = if let Some(child_position) = maybe_child_position {
            ChildStatus::IsChild {
                position: child_position,
                non_root_parent_child_positions: &non_root_parent_child_positions,
            }
        } else {
            ChildStatus::NotChild
        };
        process.print(max_num_pid_chars, is_parent, child_status);

        // potentially pop this node's parent off the 'parents' stack
        if maybe_child_position == Some(ChildPosition::LastChild)
            && non_root_parent_child_positions.len() > 0
        {
            non_root_parent_child_positions.pop();
        }

        if let Some(children) = maybe_children {
            // Since this process has children, push it onto the 'parents' stack
            // (as long as it isn't the root process)
            if let Some(child_position) = maybe_child_position {
                non_root_parent_child_positions.push(child_position);
            }

            // Push all of this process's children onto the stack, in reverse order
            // so that the first child will be popped first
            for (rev_i, child_process) in children.iter().rev().enumerate() {
                let childs_child_position = if rev_i == 0 {
                    ChildPosition::LastChild
                } else {
                    ChildPosition::MiddleChild
                };
                stack.push(ProcessStackNode {
                    process: child_process,
                    maybe_child_position: Some(childs_child_position),
                })
            }
        }
    }
}
