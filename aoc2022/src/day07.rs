use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum FsEntry {
    File(usize),
    Dir(HashMap<String, FsEntry>),
}

impl FsEntry {
    fn file_size(&self) -> usize {
        match self {
            FsEntry::File(size) => *size,
            FsEntry::Dir(dir) => dir.values().map(|e| e.file_size()).sum(),
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self, FsEntry::Dir(_))
    }

    fn visit(&self, f: &mut dyn FnMut(&FsEntry)) {
        f(self);
        if let FsEntry::Dir(dir) = self {
            dir.values().for_each(|e| e.visit(f));
        }
    }
}

fn navigate_to(
    dir: &'_ mut HashMap<String, FsEntry>,
    location: impl IntoIterator<Item = impl AsRef<str>>,
) -> &'_ mut HashMap<String, FsEntry> {
    let mut current = dir;
    for name in location {
        current = match current.get_mut(name.as_ref()) {
            Some(FsEntry::Dir(dir)) => dir,
            _ => panic!("invalid path"),
        };
    }
    current
}

fn main() {
    let file = File::open("aoc2022/inputs/day07.input").unwrap();
    let mut root = HashMap::new();
    let mut cursor = Vec::<String>::new();
    let mut current = &mut root;
    for line in io::BufReader::new(file).lines().skip(1).map(Result::unwrap) {
        let split = line.split(' ').collect::<Vec<_>>();
        match split[0] {
            "$" => {
                if split[1] == "cd" {
                    if split[2] == ".." {
                        cursor.pop();
                    } else {
                        cursor.push(split[2].to_owned());
                    }
                    current = navigate_to(&mut root, &cursor);
                }
            }
            "dir" => {
                current.insert(split[1].to_owned(), FsEntry::Dir(Default::default()));
            }
            size => {
                current.insert(
                    split[1].to_owned(),
                    FsEntry::File(size.parse::<usize>().unwrap()),
                );
            }
        }
    }
    let root = FsEntry::Dir(root);
    let unused = 70000000 - root.file_size();
    let to_free = 30000000 - unused;
    let mut sum = 0;
    let mut min = usize::MAX;
    root.visit(&mut |entry| {
        let size = entry.file_size();
        if size <= 100000 && entry.is_dir() {
            sum += size;
        }
        if size >= to_free && size < min {
            min = size;
        }
    });
    println!("part 1: {}", sum);
    println!("part 2: {}", min);
}
