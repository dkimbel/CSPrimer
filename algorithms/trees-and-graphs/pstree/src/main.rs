use regex::Regex;
use std::collections::HashMap;
use std::process::Command;
use std::sync::OnceLock;

// Minor optimization to only compile our regex one time throughout life of program
static PROCESS_LINE_REGEX: OnceLock<Regex> = OnceLock::new();

enum ChildStatus {
    NotChild,
    MiddleChild,
    LastChild,
}

struct Process {
    pid: u64,
    user: String,
    args: String,
}

struct ProcessPrintArgs<'a> {
    process: &'a Process,
    indentation_level: usize,
    // Describes the status of THIS process. If this process has no children itself
    // and is the last child of its parent, it will have ChildStatus::LastChild.
    child_status: ChildStatus,
}

impl Process {
    /// Create a Process from a line outputted by a `ps` command using a particular format.
    /// Return a tuple of the new Process and its parent PID. (Keeping the parent PID
    /// separate from the struct is just a slight optimization to avoid storing many copies
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
    fn print(
        &self,
        indentation_level: usize,
        max_num_pid_chars: usize,
        is_parent: bool,
        child_status: ChildStatus,
    ) {
        let mut s = " ".repeat(indentation_level * 4);
        let Self { pid, user, args } = self;
        s.push_str(&format!("{pid:05}"));
        println!("{}", s);
    }
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

    // NOTE: ps gives us processes sorted by process ID, no further sorting is necessary.
    // We also don't need to do any special work to figure out which process is the root:
    // besides definitely being the first line outputted by `ps`, it will have its parent
    // PID set to zero.
    for ps_line in ps_stdout.lines().skip(1) {
        // We used skip(1) above to skip the first line (just headers)
        let (process, parent_pid) = Process::from_ps_line(ps_line);
        // technically we don't HAVE to call `max`, lines are already sorted by PID
        max_pid = std::cmp::max(max_pid, process.pid);

        parent_pids_to_child_processes
            .entry(parent_pid)
            .or_insert_with(Vec::new)
            .push(process);
    }

    // We want to left-pad every printed PID with zeroes until it matches the length
    // of the largest PID.
    let max_num_pid_chars = format!("{max_pid}").len();

    // The root process will always be the one and only process with a parent PID of 0.
    let root: &Process = &parent_pids_to_child_processes.get(&0).unwrap()[0];

    // Use a stack to effectively do depth-first search through our tree, printing
    // every process as we go. Besides each individual process, we need to track
    // its level of indentation and whether it is a 'middle' versus 'last' child
    // (this affects which tree-related chars we print to the screen).
    let mut stack: Vec<ProcessPrintArgs> = vec![ProcessPrintArgs {
        process: root,
        indentation_level: 0,
        child_status: ChildStatus::NotChild, // root is not a child
    }];

    while let Some(ProcessPrintArgs {
        process,
        indentation_level,
        child_status,
    }) = stack.pop()
    {
        let maybe_children = parent_pids_to_child_processes.get(&process.pid);
        let is_parent = maybe_children.is_some_and(|children| !children.is_empty());

        process.print(
            indentation_level,
            max_num_pid_chars,
            is_parent,
            child_status,
        );

        // push all of this process's children onto the stack, in reverse order so
        // that the first child will be popped first
        if let Some(children) = maybe_children {
            for (rev_i, child_process) in children.iter().rev().enumerate() {
                let childs_child_status = if rev_i == 0 {
                    ChildStatus::LastChild
                } else {
                    ChildStatus::MiddleChild
                };
                stack.push(ProcessPrintArgs {
                    process: child_process,
                    indentation_level: indentation_level + 1,
                    child_status: childs_child_status,
                })
            }
        }
    }
}
