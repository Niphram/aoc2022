use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Directory {
    parent: Option<Weak<RefCell<Directory>>>,
    subdirs: HashMap<String, Rc<RefCell<Directory>>>,
    files: HashMap<String, usize>,
}

impl Directory {
    fn new() -> Self {
        Self {
            parent: None,
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn with_parent(parent: Weak<RefCell<Directory>>) -> Self {
        Self {
            parent: Some(parent),
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_dir(&mut self, name: String, dir: Directory) {
        self.subdirs.insert(name, Rc::new(RefCell::new(dir)));
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.files.insert(name, size);
    }

    fn size(&self) -> usize {
        let files_size: usize = self.files.iter().map(|(_, size)| size).sum();

        let subdirs_size: usize = self
            .subdirs
            .iter()
            .map(|(_, dir)| RefCell::borrow(dir).borrow().size())
            .sum();

        files_size + subdirs_size
    }

    fn get_sizes(&self) -> Vec<usize> {
        let own_size = self.size();

        let subdir_iter = self
            .subdirs
            .iter()
            .flat_map(|(_, dir)| RefCell::borrow(dir).borrow().get_sizes());

        let mut subdir_sizes: Vec<usize> = subdir_iter.collect();

        subdir_sizes.push(own_size);

        subdir_sizes
    }

    fn get_subdir(&self, name: String) -> Rc<RefCell<Directory>> {
        self.subdirs
            .get(&name)
            .expect("Get child directory")
            .clone()
    }
}

enum Command<'a> {
    Cd(&'a str),
    Ls(),
}

fn parse_command(input: &str) -> Command {
    let command = &input[1..3];

    match command {
        "cd" => Command::Cd(&input[4..]),
        "ls" => Command::Ls(),
        _ => panic!("Unknown command"),
    }
}

enum Entry<'a> {
    Dir(&'a str),
    File(&'a str, usize),
}

fn parse_ls(input: &str) -> Entry {
    if input.starts_with("dir") {
        Entry::Dir(&input[4..])
    } else {
        let parts: Vec<_> = input.split_ascii_whitespace().collect();
        let size: usize = parts[0].parse().expect("Parse filesize");
        let name = &parts[1];

        Entry::File(name, size)
    }
}

fn parse_dir(input: &str) -> Directory {
    let root_dir = Rc::new(RefCell::new(Directory::new()));

    let mut cur_dir = Rc::clone(&root_dir);

    for command_str in input.split('$').skip(1) {
        let lines: Vec<_> = command_str.lines().collect();

        let command = parse_command(lines[0]);

        match command {
            Command::Ls() => {
                for entry in lines[1..].iter() {
                    let entry = parse_ls(entry);

                    match entry {
                        Entry::File(name, size) => {
                            cur_dir.borrow_mut().add_file(name.into(), size);
                        }
                        Entry::Dir(name) => {
                            let subdir = Directory::with_parent(Rc::downgrade(&cur_dir));
                            cur_dir.borrow_mut().add_dir(name.into(), subdir);
                        }
                    }
                }
            }
            Command::Cd(path) => match path {
                "/" => {
                    cur_dir = Rc::clone(&root_dir);
                }
                ".." => {
                    let cur_dir_clone = Rc::clone(&cur_dir);
                    let parent = &RefCell::borrow(&cur_dir_clone).parent;

                    if let Some(parent) = parent {
                        let parent = Weak::upgrade(parent).unwrap();
                        cur_dir = parent;
                    };
                }
                d => {
                    let subdir = cur_dir.borrow_mut().get_subdir(d.into());
                    cur_dir = subdir;
                }
            },
        }
    }

    Rc::try_unwrap(root_dir).unwrap().into_inner()
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let root_dir = parse_dir(input);

    let size: usize = root_dir.get_sizes().iter().filter(|s| **s <= 100000).sum();
    size.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let root_dir = parse_dir(input);

    let total = root_dir.size();

    let need_to_free = total - 400000;

    let size: usize = *root_dir
        .get_sizes()
        .iter()
        .filter(|s| **s >= need_to_free)
        .min()
        .unwrap();

    size.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 7");

    let part_1 = part_1(input);
    let part_2 = part_2(input);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
