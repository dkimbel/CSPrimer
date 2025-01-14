use super::{Process, ROOT_PARENT_PID};

use std::collections::HashMap;

pub fn run(
    filter_text: &str,
    all_processes_root: &Process,
    all_parent_pids_to_child_processes: &HashMap<usize, Vec<Process>>,
) -> HashMap<usize, Vec<Process>> {
    let filter_text_lowercased = filter_text.to_lowercase();
    let mut filtered_parent_pids_to_child_processes = HashMap::new();
    let parents: Vec<&Process> = Vec::new();

    all_processes_root.filter_by_text_recursive(
        &filter_text_lowercased,
        &parents,
        all_parent_pids_to_child_processes,
        &mut filtered_parent_pids_to_child_processes,
        false,
    );
    filtered_parent_pids_to_child_processes
}

impl Process {
    /// Filter processes by the given text (case-insensitive). Matching processes _and all of
    /// their parents and children_ will be copied from the `all` map to the `filtered` map.
    fn filter_by_text_recursive<'proc>(
        &self,
        lowercased_filter_text: &str,
        parents: &Vec<&Process>,
        all_parent_pids_to_child_processes: &HashMap<usize, Vec<Process>>,
        filtered_parent_pids_to_child_processes: &mut HashMap<usize, Vec<Process>>,
        parent_already_matched: bool,
    ) {
        let matched = if parent_already_matched {
            // To match the behavior of `pstree -s`, we display children of any match (both direct and
            // indirect children -- children's children, etc). Apparently a parent was a match, so we're
            // definitely going to include the current process in our output.
            // Since we already know we've merged all parents into our 'filtered' map, we only need to
            // merge the current process.
            let parent_pid = parents.last().unwrap().pid;
            let entry = filtered_parent_pids_to_child_processes
                .entry(parent_pid)
                .or_insert_with(Vec::new);
            entry.push(self.clone());
            true
        } else if self.command.to_lowercase().contains(lowercased_filter_text) {
            // This process matches our filter! Merge the process and its parents into our filtered map.
            // Note: since I left parent pid out of the process struct, we need to keep track of
            // it ourselves.
            let mut parent_pid = ROOT_PARENT_PID; // the first parent's parent is always root
            for process in parents.iter().chain(std::iter::once(&self)) {
                let entry = filtered_parent_pids_to_child_processes
                    .entry(parent_pid)
                    .or_insert_with(Vec::new);
                // This `contains` check runs in O(n) time (not great), but these lists shouldn't be
                // very long. If necessary, we could start to use a HashSet for O(1) lookup, and later
                // convert the set to a list (sorted by process ID!).
                if !entry.contains(process) {
                    entry.push((*process).clone());
                }
                parent_pid = process.pid;
            }
            true
        } else {
            false
        };

        // Recurse through all children of this process
        if let Some(children) = all_parent_pids_to_child_processes.get(&self.pid) {
            let mut childs_parents = parents.clone();
            childs_parents.push(self);

            for child in children {
                Self::filter_by_text_recursive(
                    child,
                    lowercased_filter_text,
                    &childs_parents,
                    all_parent_pids_to_child_processes,
                    filtered_parent_pids_to_child_processes,
                    matched,
                )
            }
        }
    }
}
