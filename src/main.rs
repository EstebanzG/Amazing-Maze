use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
enum Exploration {
    UnExplored,
    Explored,
}

#[derive(Debug, Clone)]
enum Maze {
    Branch {
        label: String,
        left: Rc<Maze>,
        right: Rc<Maze>,
        status: RefCell<Exploration>,
    },
    Leaf {
        label: String,
    },
}

impl Maze {
    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                let status_value = status.borrow().clone();
                match status_value {
                    Exploration::UnExplored => {
                        status.replace(Exploration::Explored);
                        trace.push(label.clone());
                        left.explore(trace);
                        right.explore(trace);
                    }
                    Exploration::Explored => trace.push(label.clone()),
                }
            }
            Maze::Leaf { label } => trace.push(label.clone()),
        }
    }
}

fn main() {
    let maze = maze();
    let mut vect: Vec<String> = Vec::new();
    maze.explore(&mut vect);
    println!("{:?}", vect);
}

fn maze() -> Maze {
    let leaf2 = Rc::new(create_leaf(2));
    let leaf4 = Rc::new(create_leaf(4));
    let leaf5 = Rc::new(create_leaf(5));
    let leaf8 = Rc::new(create_leaf(8));

    let branch3 = Rc::new(create_branch(3, leaf4, Rc::clone(&leaf5)));
    let branch1 = Rc::new(create_branch(1, leaf2, Rc::clone(&branch3)));
    let branch7 = Rc::new(create_branch(7, leaf5, leaf8));
    let branch6 = Rc::new(create_branch(6, branch3, branch7));
    let branch0 = create_branch(0, branch1, branch6);

    branch0
}

fn create_branch(number: i32, left: Rc<Maze>, right: Rc<Maze>) -> Maze {
    Maze::Branch {
        label: number.to_string(),
        left,
        right,
        status: RefCell::new(Exploration::UnExplored),
    }
}

fn create_leaf(number: i32) -> Maze {
    Maze::Leaf {
        label: number.to_string(),
    }
}