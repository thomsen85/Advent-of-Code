use std::{rc::Rc, cell::{RefCell, Ref}, borrow::Borrow};

struct Tree {
    root: Rc<RefCell<Node>>,
}

#[derive(Debug)]
struct Node {
    id: String,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
    size: usize,
}

impl Node {
    fn new(id: String, parent: Rc<RefCell<Node>>) -> Self {
        Self { id, children: Vec::new(), parent: Some(parent), size: 0 }
    }

    fn new_root(id: String) -> Self {
        Self { id, children: Vec::new(), parent: None, size: 0 }
    }

    fn new_with_size(id: String, parent: Rc<RefCell<Node>>, size: usize) -> Self {
        parent.borrow_mut().add_size(size);
        Self {id, children: Vec::new(), parent: Some(parent), size}
    }

    fn add_size(&mut self, size: usize) {
        self.size += size;
        if let Some(parent) = &self.parent {
            parent.borrow_mut().add_size(size);
        }
    }
    fn get_child(&self, id: &str) -> Option<Rc<RefCell<Node>>> {
        println!("{:?}", &self.children);
        for child in &self.children {
            let borrow: &RefCell<Node> = child.borrow();
            println!("{} =?= {}", borrow.borrow().id, id);
            if borrow.borrow().id == id {
                return Some(Rc::clone(child));
            }
        }
        None
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        if let Some(parent) = &self.parent{
            return Some(Rc::clone(parent));
        }
        None
    }
}

fn main() {
    let input = aoc2022_rust::utils::lines_from_file("inputs/day7.txt");
    
    let tree = Tree {root: Rc::new(RefCell::new(Node::new_root("/".to_string()))) };
    let mut current_node:Rc<RefCell<Node>> = Rc::clone(&tree.root);

    for line in &input[1..] {
        println!("Running: {}", line);
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        match split[0] {
            "$" => {
                if split[1] == "cd" {
                    if split[2] == ".." {
                        let borrow: &RefCell<Node> = current_node.borrow();
                        let parent = borrow.borrow().get_parent().unwrap();
                        let clone = Rc::clone(&parent);
                        
                        current_node = clone;

                    } else {
                        let borrow: &RefCell<Node> = current_node.borrow();
                        let child = borrow.borrow().get_child(split[1]);
                        let new_node = Rc::clone(&child.unwrap());
    
                        current_node = new_node;
                    }
                }
            },
            "dir" => {
                let parent = Rc::clone(&current_node);
                current_node.borrow_mut().children.push(Rc::new(RefCell::new(Node::new(split[1].to_string(), parent))));
            }
            _ => {
                current_node.borrow_mut().children.push(Rc::new(RefCell::new(Node::new_with_size(split[1].to_string(), Rc::clone(&current_node), split[0].parse().unwrap()))))
            }
        }
    }
}