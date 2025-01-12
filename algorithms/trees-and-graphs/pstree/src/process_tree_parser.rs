use super::{Process, ROOT_PARENT_PID};

use regex::Regex;
use std::collections::HashMap;
use std::process::Command;
use std::sync::OnceLock;

pub struct AllProcessesTree {
    pub all_parent_pids_to_child_processes: HashMap<usize, Vec<Process>>,
    pub max_num_pid_chars: usize,
}

pub fn execute_ps_and_parse() -> AllProcessesTree {
    let ps_output = execute_ps();
    parse(ps_output)
}

fn execute_ps() -> String {
    let ps_stdout_bytes = Command::new("ps")
        .args(["-axwwo", "user,pid,ppid,pgid,command"]) // same args used by real pstree, I think
        .output()
        .expect("ps command failed")
        .stdout;
    String::from_utf8(ps_stdout_bytes).expect("ps failed to output valid utf-8")
}

fn parse(ps_output: String) -> AllProcessesTree {
    // To model a tree (graph where every child can have only one parent), we use a map
    // of parent PID to process instance. We could do something more elaborate where each
    // Process owns a Vec<Process> of its children, but that isn't necessary.
    let mut all_parent_pids_to_child_processes: HashMap<usize, Vec<Process>> = HashMap::new();
    let mut max_pid = 0;

    // We use skip(1) to skip the first line, which just contains headers.
    for ps_line in ps_output.lines().skip(1) {
        let (process, parent_pid) = Process::from_ps_line(ps_line);
        // Technically we don't HAVE to call `max`, lines are already sorted by PID.
        max_pid = std::cmp::max(max_pid, process.pid);

        all_parent_pids_to_child_processes
            .entry(parent_pid)
            .or_insert_with(Vec::new)
            .push(process);
    }

    // We'll want to left-pad every printed PID with zeroes until it matches the length
    // of the largest PID.
    let max_num_pid_chars = format!("{max_pid}").len();

    AllProcessesTree {
        all_parent_pids_to_child_processes,
        max_num_pid_chars,
    }
}

impl AllProcessesTree {
    pub fn get_root(&self) -> &Process {
        // The root process will always be the only child of a special parent PID.
        let root_process_list: &Vec<Process> = self
            .all_parent_pids_to_child_processes
            .get(&ROOT_PARENT_PID)
            .expect("A root process with parent pid 0 must exist");

        assert!(
            root_process_list.len() == 1,
            "There can only be one root process",
        );

        &root_process_list[0]
    }
}

// Minor optimization to only compile our regex one time throughout life of program
static PROCESS_LINE_REGEX: OnceLock<Regex> = OnceLock::new();

impl Process {
    /// Create a Process from a line outputted by a `ps` command using a particular format.
    /// Return a tuple of the new Process and its parent PID. (Keeping the parent PID
    /// separate from the struct is just a slight optimization to avoid storing extra copies
    /// of it unnecessarily, when we put the struct into a map with parent PID as the key.)
    fn from_ps_line(line: &str) -> (Self, usize) {
        let re = PROCESS_LINE_REGEX
            // example line: "root               322     1   322 /usr/libexec/keybagd -t 15"
            .get_or_init(|| Regex::new(r"^(\w+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(.*?)$").unwrap());
        let captures = re
            .captures(line)
            .expect(&format!("Failed to parse line from ps: {line}"));

        let parent_pid = captures[3]
            .parse::<usize>()
            .expect("failed to parse ppid as integer");

        (
            Self {
                user: captures[1].to_string(),
                pid: captures[2]
                    .parse::<usize>()
                    .expect("failed to parse pid as integer"),
                pgid: captures[4]
                    .parse::<usize>()
                    .expect("failed to parse pgid as integer"),
                command: captures[5].to_string(),
            },
            parent_pid,
        )
    }
}
