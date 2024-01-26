use std::cell::RefCell;
use Maze::*;
use Exploration::*;

#[derive(Debug, Clone, Copy)]
enum Exploration {
    UnExplored,
    Explored,
}

#[derive(Debug, Clone)]
enum Maze<'a> {
    Branch {
        label: String,
        left: &'a Maze<'a>,
        right: &'a Maze<'a>,
        status: RefCell<Exploration>,
    },
    Leaf {
        label: String,
    },
}

impl Maze<'_> {
    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Branch { label, left, right, status } => {
                let status_value = *status.borrow();
                match status_value {
                    UnExplored => {
                        status.replace(Explored);
                        trace.push(label.clone());
                        left.explore(trace);
                        right.explore(trace);
                    }
                    Explored => trace.push(label.clone()),
                }
            }
            Leaf { label } => trace.push(label.clone()),
        }
    }
}

fn main() {
    let leaf2 = create_leaf(2);
    let leaf4 = create_leaf(4);
    let leaf5 = create_leaf(5);
    let leaf8 = create_leaf(8);

    let branch3 = create_branch(3, &leaf4, &leaf5);
    let branch1 = create_branch(1, &leaf2, &branch3);
    let branch7 = create_branch(7, &leaf5, &leaf8);
    let branch6 = create_branch(6, &branch3, &branch7);
    let branch0 = create_branch(0, &branch1, &branch6);

    let mut vect: Vec<String> = Vec::new();
    branch0.explore(&mut vect);
    println!("{:?}", vect)
}

fn create_branch<'a>(number: i32, left: &'a Maze<'a>, right: &'a Maze<'_>) -> Maze<'a> {
    Branch {
        label: number.to_string(),
        left: &left,
        right: &right,
        status: RefCell::from(UnExplored),
    }
}

fn create_leaf(number: i32) -> Maze<'static> {
    Leaf {
        label: number.to_string(),
    }
}
