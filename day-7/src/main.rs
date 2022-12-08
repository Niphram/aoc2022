use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Directory<'a> {
    parent: Option<Weak<RefCell<Directory<'a>>>>,
    subdirs: HashMap<&'a str, Rc<RefCell<Directory<'a>>>>,
    files: HashMap<&'a str, usize>,
}

impl<'a> Directory<'a> {
    fn new() -> Self {
        Self {
            parent: None,
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn with_parent(parent: Weak<RefCell<Directory<'a>>>) -> Self {
        Self {
            parent: Some(parent),
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_dir(&mut self, name: &'a str, dir: Directory<'a>) {
        self.subdirs.insert(name, Rc::new(RefCell::new(dir)));
    }

    fn add_file(&mut self, name: &'a str, size: usize) {
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

    fn get_subdir(&self, name: &'a str) -> Rc<RefCell<Directory<'a>>> {
        self.subdirs
            .get(&name)
            .expect("Get child directory")
            .clone()
    }
}

#[derive(Debug)]
enum Entry<'a> {
    Dir(&'a str),
    File(&'a str, usize),
}

#[derive(Debug)]
enum Command<'a> {
    Cd(&'a str),
    Ls(Vec<Entry<'a>>),
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .split('$')
        .skip(1)
        .map(|c| {
            let command = &c[1..3];

            match command {
                "cd" => Command::Cd(&c.lines().next().unwrap()[4..]),
                "ls" => Command::Ls(
                    c.lines()
                        .skip(1)
                        .map(|e| {
                            if e.starts_with("dir") {
                                Entry::Dir(&e[4..])
                            } else {
                                let parts = e.split_once(' ').unwrap();
                                let size: usize = parts.0.parse().expect("Parse filesize");
                                let name = &parts.1;

                                Entry::File(name, size)
                            }
                        })
                        .collect(),
                ),
                _ => panic!("Unknown command"),
            }
        })
        .collect()
}

fn parse_dir(input: &str) -> Directory {
    let commands = parse_input(input);

    let root_dir = Rc::new(RefCell::new(Directory::new()));
    let mut cur_dir = Rc::clone(&root_dir);

    for command in commands {
        match command {
            Command::Ls(entries) => {
                for entry in entries {
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

    let need_to_free = root_dir.size() - 40000000;

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
