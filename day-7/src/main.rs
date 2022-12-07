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
    fn new(parent: Option<Weak<RefCell<Directory>>>) -> Self {
        Self {
            parent,
            subdirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_dir(&mut self, name: String, dir: Rc<RefCell<Directory>>) {
        self.subdirs.insert(name, dir);
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

    fn size2(&self) -> (usize, usize) {
        let subdirs_size: Vec<_> = self
            .subdirs
            .iter()
            .map(|(_, dir)| RefCell::borrow(dir).borrow().size2())
            .collect();

        let mut total: usize = subdirs_size.iter().map(|s| s.0).sum();
        let mut real: usize = subdirs_size.iter().map(|s| s.1).sum();

        let files_size: usize = self.files.iter().map(|(_, size)| size).sum();

        if files_size + real <= 100000 {
            total += files_size + real;
        }

        (total, real + files_size)
    }

    fn closest_to(&self, min_size: usize) -> usize {
        let subdirs = self
            .subdirs
            .iter()
            .map(|(_, dir)| RefCell::borrow(dir).borrow().closest_to(min_size))
            .filter(|s| *s >= min_size)
            .min();

        if let Some(size) = subdirs {
            size
        } else {
            self.size()
        }
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

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let root_dir = Rc::new(RefCell::new(Directory::new(None)));

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
                            RefCell::borrow_mut(&cur_dir).add_file(name.into(), size);
                        }
                        Entry::Dir(name) => {
                            let subdir = Directory::new(Some(Rc::downgrade(&cur_dir)));

                            RefCell::borrow_mut(&cur_dir)
                                .add_dir(name.into(), Rc::new(RefCell::new(subdir)));
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

                    if let Some(parent) = &RefCell::borrow(&cur_dir_clone).parent {
                        let parent = Weak::upgrade(parent).unwrap();
                        cur_dir = parent;
                    };
                }
                d => {
                    let cur_dir_clone = Rc::clone(&cur_dir);

                    let child = RefCell::borrow(&cur_dir_clone).get_subdir(d.into());

                    cur_dir = child;
                }
            },
        }
    }

    let size = RefCell::borrow(&root_dir).size2();
    size.0.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let root_dir = Rc::new(RefCell::new(Directory::new(None)));

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
                            RefCell::borrow_mut(&cur_dir).add_file(name.into(), size);
                        }
                        Entry::Dir(name) => {
                            let subdir = Directory::new(Some(Rc::downgrade(&cur_dir)));

                            RefCell::borrow_mut(&cur_dir)
                                .add_dir(name.into(), Rc::new(RefCell::new(subdir)));
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

                    if let Some(parent) = &RefCell::borrow(&cur_dir_clone).parent {
                        let parent = Weak::upgrade(parent).unwrap();
                        cur_dir = parent;
                    };
                }
                d => {
                    let cur_dir_clone = Rc::clone(&cur_dir);

                    let child = RefCell::borrow(&cur_dir_clone).get_subdir(d.into());

                    cur_dir = child;
                }
            },
        }
    }

    let total = RefCell::borrow(&root_dir).size();

    let size = RefCell::borrow(&root_dir).closest_to(30000000 - (70000000 - total));
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
