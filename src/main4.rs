use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Exploration {
    Explored,
    UnExplored,
}

#[derive(Debug, Clone)]
enum Maze {
    Branch {
        label: String,
        left: Rc<Maze>,
        right: Rc<Maze>,
        status: RefCell<Exploration>,
    },
    Leaf(String),
}

impl Maze {
    fn new_branch(label: &str, left: Rc<Maze>, right: Rc<Maze>) -> Self {
        Maze::Branch {
            label: label.to_string(),
            left,
            right,
            status: RefCell::new(Exploration::UnExplored),
        }
    }

    fn new_leaf(label: &str) -> Rc<Self> {
        Rc::new(Maze::Leaf(label.to_string()))
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

fn maze() -> Rc<Maze> {
    let leaf2 = Maze::new_leaf("2");
    let leaf4 = Maze::new_leaf("4");
    let leaf5 = Maze::new_leaf("5");
    let leaf8 = Maze::new_leaf("8");

    let branch3 = Rc::new(Maze::new_branch("3", leaf4.clone(), leaf5.clone()));
    let branch1 = Rc::new(Maze::new_branch("1", leaf2.clone(), branch3.clone()));
    let branch7 = Rc::new(Maze::new_branch("7", leaf5.clone(), leaf8.clone()));
    let branch6 = Rc::new(Maze::new_branch("6", branch3.clone(), branch7.clone()));
    let branch0 = Rc::new(Maze::new_branch("0", branch1.clone(), branch6.clone()));

    branch0
}

pub fn main() {
    println!("\nVERSION 4\n");
    let maze = maze();

    let mut trace = Vec::new();
    maze.explore_with_trace(&mut trace);
    println!("Trace of exploration (with mutable trace): {:?}", trace);

    maze.unexplore();
    maze.visualize("");

    let trace_immutable = maze.explore();
    println!("Trace of exploration (immutable): {:?}", trace_immutable);
}