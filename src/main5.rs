use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Exploration {
    Explored,
    PartiallyExplored,
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

    fn explore_with_trace(&self, self_rc: Rc<Maze>, work: &mut Vec<Rc<Maze>>, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                let mut current_status = status.borrow_mut();
                match *current_status {
                    Exploration::UnExplored => {
                        *current_status = Exploration::PartiallyExplored;
                        trace.push(label.clone());
                        work.push(self_rc);
                        work.push(left.clone());
                    }
                    Exploration::PartiallyExplored => {
                        *current_status = Exploration::Explored;
                        trace.push(label.clone());
                        work.push(right.clone());
                    }
                    Exploration::Explored => {
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
    println!("\nVERSION 5\n");
    let mut work = vec![maze()];
    let mut trace = vec![];

    work[0].visualize("");

    while !work.is_empty() {
        let node = work.pop().expect("work stack should not be empty");
        node.explore_with_trace(node.clone(), &mut work, &mut trace);
        println!("Current trace: {:?}", trace);
    }
}