use std::{rc::Rc, cell::RefCell, vec}; 

const INPUT: &str = include_str!("../../inputs/day7/example.txt");

#[derive(Debug)]
enum Node {
    Dir(Directory),
    File(File),
}

#[derive(Debug)]
struct Tree<'a> {
    children: &'a mut Vec<Rc<RefCell<Node>>>
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new_file_node(name: String, size: u32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::File(File { name, size }))) 
    }

    fn new_dir_node(name: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node::Dir(Directory { name, children: vec![], parent: None})))
    }
}

trait TotalSize<'a> {
    fn total_size(&'a self) -> u32;
}

impl<'a> TotalSize<'a> for Tree<'a> {
    fn total_size(&'a self) -> u32 {
        let total_size: u32 = self.children.iter().fold(0, |mut acc, it| -> u32 {
            let child = &(*it.borrow()); acc += match child {
                Node::Dir(dir) => dir.total_size(),
                Node::File(file) => file.size,
            };
            acc
        });
        total_size
    }
}

impl<'a> TotalSize<'a> for Directory {
    fn total_size(&'a self) -> u32 {
        let total_size: u32 = self.children.iter().fold(0, |mut acc, it| -> u32 {
            let child = &(*it.borrow());
            acc += match child {
                Node::Dir(dir) => dir.total_size(),
                Node::File(file) => file.size,
            };
            acc
        });
        total_size
    }
}

impl Directory {
    fn add_child(&mut self, child: Node) {
        match child {
            Node::Dir(c) => self.children.push(Node::new_dir_node(c.name)),
            Node::File(c) => self.children.push(Node::new_file_node(c.name, c.size)),
        }
    }
}

fn parse_input() -> u32 {

    let tree = Tree { children: &mut vec![] };
    tree.children.push(Node::new_dir_node("/".to_string()));

    let mut current_dir = tree.children.first().unwrap();

    INPUT
        .lines()
        .fold(Tree { children: &mut vec![] }, |acc, it| {
         
            acc.children.push(Node::new_dir_node("/".to_string()));

            acc
        });

    println!("{:?}", &tree);

    return 0;
}

pub fn part_1() -> String {

    let tree: Tree = Tree { children: &mut vec![] };
    let file = Node::File(File { name: "En fil".to_string(), size: 69 });
    let tredje = Node::new_dir_node("en tredje dirnode".to_string());

//    if let Node::Dir(d) = &mut *tredje.borrow_mut() {
//        d.add_child(Node::File(File {name: "en file node".to_string(), size: 6999}));
//        d.add_child(file)
//    }

    tree.children.push(Node::new_dir_node("en ny dirnode".to_string()));
    tree.children.push(Node::new_dir_node("en annan dirnode".to_string()));
    tree.children.push(tredje.to_owned());
    tree.children.push(Node::new_dir_node("en fjärde dirnode".to_string()));

//    let total = tree.total_size();
//    println!("Total size: {:?}", &total);

    parse_input();

    return "".to_string();
}

#[test]
fn test() {
    assert_eq!(part_1(), "asd");
}
