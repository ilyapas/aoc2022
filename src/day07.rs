use std::collections::HashMap;

#[derive(Debug, Clone)]
struct File {
    id: usize,
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    id: usize,
    name: String,
    size: usize,
    subdirectories: Vec<usize>,
    files: Vec<usize>,
    parent: usize,
}

#[derive(Debug, Clone)]
enum Node {
    Directory(Directory),
    File(File),
}

type FileSystem = HashMap<usize, Node>;

fn dir_size(fs: &mut FileSystem, dir: &mut Directory) -> usize {
    let mut size = 0;
    for file_id in dir.files.iter() {
        let file_node = fs.get(file_id).unwrap();
        if let Node::File(file) = file_node {
            size += file.size;
        }
    }
    for subdirectory_id in dir.subdirectories.iter_mut() {
        let mut fs_clone = fs.clone();
        let subdirectory_node = fs.get_mut(subdirectory_id).unwrap();
        if let Node::Directory(subdirectory) = subdirectory_node {
            size += dir_size(&mut fs_clone, subdirectory);
        }
    }
    dir.size = size;
    size
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day07.prod.txt").unwrap();
    let mut fs = FileSystem::new();
    let mut id: usize = 0;

    fs.insert(
        id,
        Node::Directory(Directory {
            id,
            name: String::from("/"),
            size: 0,
            subdirectories: Vec::new(),
            files: Vec::new(),
            parent: 0,
        }),
    );
    let mut current_dir_id = id;
    for line in input.lines() {
        if line.starts_with("$ cd ..") {
            let current_node = fs.get(&current_dir_id).unwrap();
            if let Node::Directory(dir) = current_node {
                current_dir_id = dir.parent;
            }
        } else if line.starts_with("$ cd ") {
            let current_node = fs.get(&current_dir_id).unwrap();
            if let Node::Directory(dir) = current_node {
                let name = line.split_whitespace().collect::<Vec<&str>>()[2];
                for subdirectory_id in dir.subdirectories.iter() {
                    let subdirectory_node = fs.get(subdirectory_id).unwrap();
                    if let Node::Directory(subdirectory) = subdirectory_node {
                        if subdirectory.name == name {
                            current_dir_id = subdirectory.id;
                            break;
                        }
                    }
                }
            }
        } else if line.starts_with("dir") {
            // add new directory
            let current_node = fs.get_mut(&current_dir_id).unwrap();
            if let Node::Directory(dir) = current_node {
                id += 1;
                let name = line.split_whitespace().collect::<Vec<&str>>()[1];
                let subdirectories = Vec::new();
                let files = Vec::new();
                let size = 0;
                let parent = dir.id;
                let new_dir = Directory {
                    id,
                    name: name.to_string(),
                    size,
                    subdirectories,
                    files,
                    parent,
                };
                dir.subdirectories.push(id);
                fs.insert(id, Node::Directory(new_dir));
            }
        } else if line != "$ ls" {
            // add new file to current directory
            let current_node = fs.get_mut(&current_dir_id).unwrap();
            if let Node::Directory(dir) = current_node {
                id += 1;
                let words = line.split_whitespace().collect::<Vec<&str>>();
                let size = words[0].parse::<usize>().unwrap();
                let name = words[1].to_string();
                let new_file = File { id, name, size };
                dir.files.push(id);
                fs.insert(id, Node::File(new_file));
            }
        }
    }

    // calculate size of each directory
    for (_, node) in fs.clone().iter_mut() {
        if let Node::Directory(dir) = node {
            dir_size(&mut fs, dir);
        }
    }

    let result: usize = fs
        .iter()
        .filter(|(_, node)| {
            if let Node::Directory(dir) = node {
                dir.size < 100000
            } else {
                false
            }
        })
        .map(|(_, node)| {
            if let Node::Directory(dir) = node {
                dir.size
            } else {
                0
            }
        })
        .sum();

    println!("Day 07 - Part One: {}", result);
}
