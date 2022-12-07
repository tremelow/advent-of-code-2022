use std::fs;
use std::str::Lines;
const INPUT_FILE: &str = "data/input07.txt";

const TOTAL_DISK_SPACE: u32 = 70000000;
const GOAL_DISK_SPACE: u32 = 30000000;

#[derive(Debug)]
struct FileSystNode {
    idx: usize,
    /// Identifying value
    path: String,
    size: u32,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl FileSystNode {
    fn new(idx: usize, path: &str, size: u32) -> Self {
        Self {
            idx,
            path: String::from(path),
            size,
            parent: None,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child_idx: usize) {
        self.children.push(child_idx);
    }

    fn is_dir(&self) -> bool {
        return !(self.children.is_empty());
    }
}

#[derive(Debug, Default)]
struct FileSystem {
    files: Vec<FileSystNode>,
}

impl FileSystem {
    fn new() -> Self {
        Self {files: Vec::new(),}
    }

    /// Get node index from path
    fn get_node(&self, path: &str) -> Option<usize> {
        return (&self.files).into_iter().position(|fnode| fnode.path == path);
    }

    /// Add new node assuming no node with same identifier (i.e. path) exists.
    fn add_node(&mut self, path: &str, size: u32) -> usize{
        let idx = self.files.len();
        // self.files.push(Box::new(FileSystNode::new(idx, path, size)));
        self.files.push(FileSystNode::new(idx, path, size));
        return idx;
    }
}

fn is_cmd(l: &str) -> bool {
    return l.chars().next() == Some('&');
}

fn parse_dir(l: &mut Lines, fsyst: &mut FileSystem, cur_dir: usize) {
    let path_prefix = fsyst.files[cur_dir].path.clone() + "/";
    while let Some(line) = l.next() {
        let basename = line.split(" ").last().unwrap();
        let path = path_prefix.clone() + basename;

        // Leave dir
        if basename == ".." {
            break;
        }
        // Change dir
        else if line.starts_with("$ cd ") {
            let next_dir = fsyst.get_node(&path).unwrap();
            parse_dir(l, fsyst, next_dir);
        }
        // Add file to system 
        else if line != "$ ls" {
            let fsize: u32 = match line.split(" ").next().unwrap().parse() {
                Ok(n) => n,
                Err(_) => 0, // The node is a dir
            };
            let idx = fsyst.add_node(&path,  fsize);
            fsyst.files[cur_dir].add_child(idx);
        }
    }
}

fn parse_lines(l: &mut Lines, fsyst: &mut FileSystem) {
    if let Some(_) = l.next() {
        let root = fsyst.add_node("", 0);
        parse_dir(l, fsyst, root);
    }
}

fn compute_dir_size(fsyst: &mut FileSystem, cur_node: usize) -> u32 {
    if fsyst.files[cur_node].is_dir() {
        fsyst.files[cur_node].size = fsyst.files[cur_node].children.clone()
            .into_iter()
            .map(|idx| compute_dir_size(fsyst, idx))
            .sum();
    }
    return fsyst.files[cur_node].size;
}


pub fn main() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
    let mut contents = contents.lines();
    
    let mut fsyst = FileSystem::new();
    parse_lines(&mut contents, &mut fsyst);
    compute_dir_size(&mut fsyst, 0);

    return fsyst.files.into_iter()
        .filter(|fnode| fnode.is_dir())
        .map(|fnode| fnode.size)
        .filter(|s| *s <= 100_000)
        .sum();
}

pub fn main_bonus() -> u32 {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file.");
    let mut contents = contents.lines();
    
    let mut fsyst = FileSystem::new();
    parse_lines(&mut contents, &mut fsyst);
    compute_dir_size(&mut fsyst, 0);

    let used_disk_space = fsyst.files[0].size;
    let current_free_space = TOTAL_DISK_SPACE - used_disk_space;
    let free_at_least = GOAL_DISK_SPACE - current_free_space;

    return fsyst.files.into_iter()
        .filter(|fnode| fnode.is_dir())
        .map(|fnode| fnode.size)
        .filter(|s| *s >= free_at_least)
        .min()
        .unwrap();
}