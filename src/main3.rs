use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Exploration {
    Explored,
    UnExplored,
}

#[derive(Debug, Clone)]
enum Maze<'a> {
    Branch {
        label: String,
        left: &'a Maze<'a>,
        right: &'a Maze<'a>,
        status: RefCell<Exploration>,
    },
    Leaf(String),
}

impl<'a> Maze<'a> {
    fn new_branch(label: &str, left: &'a Maze<'a>, right: &'a Maze<'a>) -> Self {
        Maze::Branch {
            label: label.to_string(),
            left,
            right,
            status: RefCell::new(Exploration::UnExplored),
        }
    }

    fn new_leaf(label: &str) -> Self {
        Maze::Leaf(label.to_string())
    }

    fn explore(&self) -> Vec<String> {
        match self {
            Maze::Branch { label, left, right, status } => {
                let mut trace = Vec::new();
                let mut current_status = status.borrow_mut();
                match *current_status {
                    Exploration::UnExplored => {
                        *current_status = Exploration::Explored;
                        trace.push(label.clone());
                        drop(current_status);

                        trace.extend(left.explore());
                        trace.extend(right.explore());
                    }
                    _ => {
                        trace.push(label.clone());
                    }
                }
                trace
            }
            Maze::Leaf(label) => vec![label.clone()],
        }
    }

    fn unexplore(&self) -> Vec<String> {
        match self {
            Maze::Branch { label, left, right, status } => {
                let mut trace = Vec::new();
                let mut current_status = status.borrow_mut();
                if *current_status == Exploration::Explored {
                    *current_status = Exploration::UnExplored;
                    trace.push(label.clone());
                    drop(current_status);

                    trace.extend(left.unexplore());
                    trace.extend(right.unexplore());
                } else {
                    trace.push(label.clone());
                }
                trace
            }
            Maze::Leaf(label) => vec![label.clone()],
        }
    }

    fn explore_with_trace(&self, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                let mut current_status = status.borrow_mut();
                match *current_status {
                    Exploration::UnExplored => {
                        *current_status = Exploration::Explored;
                        trace.push(label.clone());
                        drop(current_status);

                        trace.extend(left.explore());
                        trace.extend(right.explore());
                    }
                    _ => {
                        trace.push(label.clone());
                    }
                }
            }
            Maze::Leaf(label) => {
                trace.push(label.clone());
            }
        }
    }

    fn visualize(&self, indent: &str) {
        match self {
            Maze::Branch { label, left, right, status } => {
                println!("{}Branch: {} (Status: {:?})", indent, label, status.borrow());
                left.visualize(&(indent.to_string() + "  "));
                right.visualize(&(indent.to_string() + "  "));
            }
            Maze::Leaf(label) => {
                println!("{}Leaf: {}", indent, label);
            }
        }
    }
}

pub fn main() {
    println!("\nVERSION 3\n");
    let leaf2 = Maze::new_leaf("2");
    let leaf4 = Maze::new_leaf("4");
    let leaf5 = Maze::new_leaf("5");
    let leaf8 = Maze::new_leaf("8");

    let branch3 = Maze::new_branch("3", &leaf4, &leaf5);
    let branch1 = Maze::new_branch("1", &leaf2, &branch3);
    let branch7 = Maze::new_branch("7", &leaf5, &leaf8);
    let branch6 = Maze::new_branch("6", &branch3, &branch7);
    let branch0 = Maze::new_branch("0", &branch1, &branch6);

    let mut trace = Vec::new();
    branch0.explore_with_trace(&mut trace);
    println!("Trace of exploration (with mutable trace): {:?}", trace);

    branch0.unexplore();
    branch0.visualize("");

    let trace_immutable = branch0.explore();
    println!("Trace of exploration (immutable): {:?}", trace_immutable);
}