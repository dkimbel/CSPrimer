use crossterm::style::{style, Color, Stylize};
use crossterm::terminal;
use std::collections::HashMap;
use std::env;
use std::env::Args;

mod process_tree_filter;
mod process_tree_parser;

// These are the colors our tree characters will cycle through as they become
// increasingly nested.
const COLORS: [Color; 3] = [Color::Yellow, Color::Red, Color::Cyan];
const ROOT_PARENT_PID: usize = 0;

/// Chars that will be printed to the screen to reflect the structure of the tree.
/// R = Right, L = Left, T = Top, B = Bottom. So e.g. RL is a dash-like char that
/// extends from left to right. DOUBLE_RL is a special case of two lines, like
/// the equals sign.
enum TreeChar {
    RL,
    DoubleRL,
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
            DoubleRL => '=', // could use ═, but it's less visually distinct from ─
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

impl Process {
    fn print_recursive(
        &self,
        max_num_pid_chars: usize,
        terminal_width: usize,
        child_status: ChildStatus,
        parent_pids_to_child_processes: &HashMap<usize, Vec<Process>>,
        maybe_filter_text: Option<&str>,
    ) {
        let maybe_children = parent_pids_to_child_processes.get(&self.pid);
        let is_parent = maybe_children.is_some_and(|children| !children.is_empty());

        // Do the actual printing
        let (tree_chars, num_visible_tree_chars) = self.get_tree_chars(is_parent, child_status);
        let Self {
            pid, user, command, ..
        } = self;
        let formatted_pid = format!("{pid:0max_num_pid_chars$}");
        // We add 3 to deal with whitespace we added between different pieces of text. Meanwhile,
        // we assume text will be ASCII and so use len() instead of .chars().count().
        let visible_content_length =
            num_visible_tree_chars + 3 + formatted_pid.len() + user.len() + command.len();

        let formatted_command = if let Some(filter_text) = maybe_filter_text {
            if let Some(match_start_i) = command.to_lowercase().find(filter_text) {
                let match_end_i = match_start_i + filter_text.len();
                &format!(
                    "{}{}{}",
                    &command[..match_start_i],
                    &command[match_start_i..match_end_i].white(),
                    &command[match_end_i..]
                )
            } else {
                // Annoying to have to use this `else` case twice -- very soon Rust will
                // support 'if let chaining', which would clean this up
                command
            }
        } else {
            command
        };

        let process_line = format!(
            "{tree_chars} {} {} {formatted_command}",
            formatted_pid.blue(),
            style(&user).with(Color::Magenta)
        );
        // We have to calculate how many ansi color code characters are present, because we add
        // a variable amount of them to our tree chars.
        let num_ansi_color_chars = process_line.chars().count() - visible_content_length;
        let num_chars_to_print = terminal_width + num_ansi_color_chars;
        println!("{process_line:.num_chars_to_print$}");

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
                    terminal_width,
                    childs_child_status,
                    parent_pids_to_child_processes,
                    maybe_filter_text,
                );
            }
        }
    }

    fn get_tree_chars(&self, is_parent: bool, child_status: ChildStatus) -> (String, usize) {
        match child_status {
            ChildStatus::NotChild => {
                let middle_tree_char = if is_parent {
                    TreeChar::RBL
                } else {
                    TreeChar::RL
                };
                let last_tree_char = if self.pid == self.pgid {
                    TreeChar::DoubleRL
                } else {
                    TreeChar::RL
                };
                let s = [TreeChar::RL, middle_tree_char, last_tree_char]
                    .iter()
                    .map(|tc| tc.to_char())
                    .collect::<String>();
                let styled = style(&s).with(COLORS[0]).to_string();
                (styled, s.len())
            }
            ChildStatus::IsChild {
                position,
                non_root_parent_child_positions,
            } => {
                let mut s = String::from(' ');
                let mut num_uncolored_chars: usize = 1;
                let mut colors_i: usize = 0;

                // We need to indent this Process further based on how many parents
                // it has. We might also need to draw some top-to-bottom lines on
                // behalf of those parents, to reach their further-down siblings.
                for parent_child_position in non_root_parent_child_positions {
                    let position_char = match parent_child_position {
                        ChildPosition::MiddleChild => TreeChar::TB.to_char(),
                        ChildPosition::LastChild => ' ',
                    };
                    let mut unstyled = String::new();
                    unstyled.push(position_char);
                    unstyled.push(' ');
                    num_uncolored_chars += 2;
                    let styled = style(unstyled)
                        .with(COLORS[colors_i % COLORS.len()])
                        .to_string();
                    s.extend(styled.chars());

                    colors_i += 1;
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
                let last_tree_char = if self.pid == self.pgid {
                    TreeChar::DoubleRL
                } else {
                    TreeChar::RL
                };
                let final_chars = [
                    position_tree_char,
                    TreeChar::RL,
                    branch_to_children_tree_char,
                    last_tree_char,
                ]
                .map(|tc| tc.to_char())
                .iter()
                .collect::<String>();
                num_uncolored_chars += 4;
                let final_chars_styled = style(final_chars)
                    .with(COLORS[colors_i % COLORS.len()])
                    .to_string();
                s.extend(final_chars_styled.chars());
                (s, num_uncolored_chars)
            }
        }
    }
    // TODO Any refactor / code cleanup?
    //   - maybe have a ProcessPrinter that keeps track of max_num_process_chars for us, plus terminal
    //     width and even hashmap of parent PIDs to child processes? But, big question: how to share code
    //     between the ProcessPrinter and a ProcessSearcher used to filter for processes that match string?
    //   - try to be guided by common arguments not having to be passed to functions, because they belong
    //     to a relevant struct already?
    //   - make sure 'get root from tree' logic isn't excessively duplicated. Related... make sure I'm not
    //     unnecessarily passing root node references around. It makes me a little uncomfortable; I'd rather
    //     look up the root from the tree 'just in time'. It also makes me pass in too many params to fns.
    //     Solution: make a ProcessTree struct that has the root on its impl? Means renaming AllProcessesTree
    //     struct to something new, but probably worth it.
    //   - add a comment on how I could have done printing in the same pass as parsing, since inputs
    //     were pre-sorted. But just as well to keep that separate given optional filtering step.
    //   - only do pid == pgid check one place, not two?
    //   - can I clean up 'num ansi chars' calculation code?
    //   - instead of making PrintRecursive know our filter text (lowercased), instead have it use a
    //     HashSet of index ranges by pid, returned by our filter fn? In this case, flip the order of
    //     filter fn's first conditional! And explain importance of ordering in a comment.
    // TODO add/update comments? add missing docstrings?
    // TODO Add README
    //   - featuring screenshots and noting crossterm.
    //     - screenshots to include comparison with `login` argument.
    //   - noting crossterm dependency for terminal width + colorized text
    //   - describe `cargo build --release`, `./target/release/pstree`
    // TODO how to properly 'install' my own pstree on my machine, to use it from any directory? Just add an
    //   alias to ./Users/dk/Workspace/cs-primer/algorithms/trees-and-graphs/pstree/target/release/pstree?
    // TODO Submit, mentioning screenshot in README or direct-linking it; also note crossterm for width/colors
}

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

    let terminal_width = terminal::size()
        .expect("Failed to find terminal's dimensions")
        .0 as usize;

    if let Some(root) = &parent_pids_to_child_processes
        .get(&ROOT_PARENT_PID)
        .and_then(|root_process_list| root_process_list.get(0))
    {
        // Print our tree of processes -- either all of them, or all that remain after
        // filtering. In principle we always want to do this, but it's nested in a conditional
        // because we could have filtered out _everything_. In that case there isn't even
        // a root to start recursing from.
        Process::print_recursive(
            root,
            all_processes_tree.max_num_pid_chars,
            terminal_width,
            ChildStatus::NotChild,
            &parent_pids_to_child_processes,
            filter_processes_by_text
                .map(|s| s.to_lowercase())
                .as_deref(),
        );
    }
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
