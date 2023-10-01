use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
struct Node {
    id: String,
    value: usize,
    parent: Option<Weak<Mutex<Node>>>,
    children: Vec<Arc<Mutex<Node>>>,
}

impl Node {
    fn new_root(id: String) -> Self {
        Self {
            id,
            value: 0,
            parent: None,
            children: Vec::new(),
        }
    }

    fn new_child(id: String, value: usize, parent: Weak<Mutex<Node>>) -> Self {
        let mut child = Self {
            id,
            value: 0,
            parent: Some(parent),
            children: Vec::new(),
        };
        child._backpropegate(value);
        child
    }

    fn get_child(&self, id: &str) -> Option<Arc<Mutex<Node>>> {
        for child in &self.children {
            if child.lock().unwrap().id == id {
                return Some(Arc::clone(child));
            }
        }
        None
    }

    fn _backpropegate(&mut self, value: usize) {
        self.value += value;

        if let Some(parent) = &self.parent {
            if let Some(ptr) = parent.upgrade() {
                ptr.lock().unwrap()._backpropegate(value);
            }
        }
    }
}

#[derive(Debug)]
struct Tree {
    root: Arc<Mutex<Node>>,
}

impl Tree {
    fn new(root_value: String) -> Self {
        Tree {
            root: Arc::new(Mutex::new(Node::new_root(root_value))),
        }
    }

    fn get_node(&mut self, path: &Vec<String>) -> Arc<Mutex<Node>> {
        let mut temp_child = Arc::clone(&self.root);
        for p in path {
            let child = temp_child.lock().unwrap().get_child(p);
            temp_child = child.unwrap();
        }
        temp_child
    }
}

fn main() {
    let input = aoc2022_rust::utils::lines_from_file("inputs/day7.txt");

    let mut tree = Tree::new("/".to_string());
    let mut path = Vec::new();

    for line in &input[1..] {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        match split[0] {
            "$" => {
                if split[1] == "cd" {
                    if split[2] == ".." {
                        path.pop();
                    } else {
                        path.push(split[2].to_string());
                    }
                }
            }
            "dir" => {
                let parent = tree.get_node(&path);
                let child = Node::new_child(split[1].to_string(), 0, Arc::downgrade(&parent));
                parent
                    .lock()
                    .unwrap()
                    .children
                    .push(Arc::new(Mutex::new(child)));
            }
            _ => {
                let parent = tree.get_node(&path);
                let child = Node::new_child(
                    split[1].to_string(),
                    split[0].parse().unwrap(),
                    Arc::downgrade(&parent),
                );
                parent
                    .lock()
                    .unwrap()
                    .children
                    .push(Arc::new(Mutex::new(child)));
            }
        }
    }
    let mut l = Vec::new();
    println!("{}", get_sum(&tree.root));
    get_all(&tree.root, &mut l, 0);
    l.sort_by(|a, b| a.0.cmp(&b.0));
    let la = l
        .into_iter()
        .filter(|o| o.0 > 6233734)
        .collect::<Vec<(usize, String, usize)>>();
    println!("{:?}", la.first());
}

fn get_sum(node: &Arc<Mutex<Node>>) -> usize {
    let mut sum = 0;
    if node.lock().unwrap().children.is_empty() {
        return 0;
    } else {
        if node.lock().unwrap().value < 100_000 {
            sum += node.lock().unwrap().value;
        }

        for child in &node.lock().unwrap().children {
            sum += get_sum(child)
        }
    }

    sum
}

fn get_all(node: &Arc<Mutex<Node>>, vec: &mut Vec<(usize, String, usize)>, depth: usize) {
    if node.lock().unwrap().children.is_empty() {
    } else {
        let size = node.lock().unwrap().value;
        let value = node.lock().unwrap().id.clone();
        vec.push((size, value, depth));

        for child in &node.lock().unwrap().children {
            get_all(child, vec, depth + 1)
        }
    }
}
