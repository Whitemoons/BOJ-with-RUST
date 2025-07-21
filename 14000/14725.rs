use std::io::{stdin, Read};

#[derive(Clone)]
struct Node<'a> {
    value: &'a str,
    children: Vec<Node<'a>>,
}

struct Tree<'a> {
    root: Option<Box<Node<'a>>>,
}

fn add_tree<'a, I: Iterator<Item = &'a str>>(tree: &mut Tree<'a>, food_iter: &mut I) {
    if tree.root.is_none() {
        if let Some(food) = food_iter.next() {
            tree.root = Some(Box::new(Node { value: food, children: vec![] }));
        } else {
            return;
        }
    }
    let mut current: &mut Node = tree.root.as_mut().unwrap();
    for food in food_iter {
        if let Some(idx) = current.children.iter().position(|c| c.value == food) {
            current = &mut current.children[idx];
        } else {
            current.children.push(Node { value: food, children: vec![] });
            let len = current.children.len();
            current = &mut current.children[len - 1];
        }
    }
}

fn print_tree(node: &Node, depth: i8) {
    for _ in 0..depth {
        print!("--");
    }
    if depth != -1 {
        println!("{}", node.value);
    }
    let mut children = node.children.clone();
    children.sort_by(|a, b| a.value.cmp(b.value));
    for child in &children {
        print_tree(child, depth + 1);
    }
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut tree = Tree { root: Some(Box::new(Node { value: "root", children: vec![] })) };

    let mut iter = buffer.split_ascii_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let k: usize = iter.next().unwrap().parse().unwrap();
        let mut food_iter = iter.by_ref().take(k);
        add_tree(&mut tree, &mut food_iter);
    }

    print_tree(&tree.root.as_ref().unwrap(), -1);
}