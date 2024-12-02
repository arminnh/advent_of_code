use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::str::Lines;

struct Dir {
    name: String,
    parent: Option<Rc<RefCell<Dir>>>,
    children: Vec<Rc<RefCell<Dir>>>,
    files: Vec<Rc<RefCell<File>>>,
}

struct File {
    name: String,
    size: u32,
}

impl Dir {
    fn new(name: &str, parent: Option<Rc<RefCell<Dir>>>) -> Self {
        Dir {
            name: name.to_string(),
            parent,
            children: Vec::new(),
            files: Vec::new(),
        }
    }

    fn print(&self, depth: usize) {
        println!("{:2$}- {} (dir)", " ", self.name, 4 * depth);
        self.children
            .iter()
            .for_each(|child| child.borrow().print(depth + 1));
        self.files
            .iter()
            .for_each(|file| file.borrow().print(depth + 1));
    }
}

impl File {
    fn print(&self, depth: usize) {
        println!(
            "{:3$}- {} (file, size = {})",
            " ",
            self.name,
            self.size,
            4 * depth
        );
    }
}

fn move_wd_up(wd: &mut Rc<RefCell<Dir>>) {
    println!("MOVE UP");
    let wd_clone: Rc<RefCell<Dir>> = Rc::clone(&*wd);
    let parent: &Option<Rc<RefCell<Dir>>> = &wd_clone.borrow().parent;
    match parent {
        Some(dir) => *wd = Rc::clone(dir),
        None => panic!("No dir to move up to."),
    }
}

fn move_wd_to_dir(wd: &mut Rc<RefCell<Dir>>, dirname: &str) {
    println!("MOVE TO {:?}", dirname);
    let wd_clone: Rc<RefCell<Dir>> = Rc::clone(&*wd);
    let children: &Vec<Rc<RefCell<Dir>>> = &wd_clone.borrow().children;
    match children.iter().find(|c| c.borrow().name == dirname) {
        Some(dir) => *wd = Rc::clone(dir),
        None => panic!("Target dir does not exist."),
    }
}

fn register_dir(wd: &Rc<RefCell<Dir>>, dirname: &str) {
    println!("REGISTER DIR {:?}", dirname);
    let dir: Dir = Dir::new(dirname, Some(Rc::clone(wd)));
    wd.borrow_mut().children.push(Rc::new(RefCell::new(dir)));
}

fn register_file(wd: &Rc<RefCell<Dir>>, filename: &str, filesize: &str) {
    println!("REGISTER FILE {:?}", filename);
    wd.borrow_mut().files.push(Rc::new(RefCell::new(File {
        name: filename.to_string(),
        size: filesize.parse().unwrap(),
    })));
}

fn collect_dirs(dir: &Rc<RefCell<Dir>>, max_size: u32) -> (u32, Vec<(String, u32)>) {
    // Recursively collect total size of current dir along with list of subdirectories that are under the size limit.
    let (mut cumulative_size, mut subdirs_with_size): (u32, Vec<(String, u32)>) = dir
        .borrow()
        .children
        .iter()
        .fold((0, Vec::new()), |(total, dirs_with_sizes), subdir| {
            let (subdirs_total, subdirs) = collect_dirs(subdir, max_size);
            (total + subdirs_total, [dirs_with_sizes, subdirs].concat())
        });

    let size_files: u32 = dir
        .borrow()
        .files
        .iter()
        .fold(0, |acc, f| acc + f.borrow().size);

    // println!(
    //     "Dir {}: cumulative_size {}, size_files {}, {:?}",
    //     dir.borrow().name, cumulative_size,  size_files, subdirs_with_size
    // );

    // If size of all subdirs + files in current dir <= max, then add this dir to the collection.
    cumulative_size += size_files;
    if cumulative_size <= max_size {
        subdirs_with_size.push((dir.borrow().name.clone(), cumulative_size))
    }

    (cumulative_size, subdirs_with_size)
}

/// Process input and return root of directory tree
fn process_input(lines: &mut Lines) -> Rc<RefCell<Dir>> {
    let root: Rc<RefCell<Dir>> = Rc::new(RefCell::new(Dir::new("/", None)));
    let mut wd: Rc<RefCell<Dir>> = Rc::clone(&root);

    while let Some(line) = lines.next() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", ".."] => move_wd_up(&mut wd),
            ["$", "cd", "/"] => wd = Rc::clone(&root),
            ["$", "cd", dirname] => move_wd_to_dir(&mut wd, dirname),
            ["$", "ls"] => (),
            ["dir", dirname] => register_dir(&wd, dirname),
            [filesize, filename] => register_file(&wd, filename, filesize),
            _ => println!("Unsupported input: {:?}", line),
        }
    }
    // root.borrow().print(0);
    root
}

/// Find all of the directories with a total size of at most 100000.
/// What is the sum of the total sizes of those directories?
fn part_1(lines: &mut Lines) -> u32 {
    let root: Rc<RefCell<Dir>> = process_input(lines);
    let max_size = 100_000;
    let (_, dirs): (u32, Vec<(String, u32)>) = collect_dirs(&root, max_size);
    println!("{:?}", dirs);
    let sum = dirs.iter().fold(0, |acc, (_, size)| acc + size);
    println!("{}", sum);
    sum
}

/// Find the smallest directory that, if deleted, would free up enough space on the filesystem to
/// run the update. What is the total size of that directory?
fn part_2(lines: &mut Lines) -> u32 {
    let total_disk_space = 70_000_000;
    let target_free_space = 30_000_000;

    let root: Rc<RefCell<Dir>> = process_input(lines);
    let (used_space, mut dirs): (u32, Vec<(String, u32)>) = collect_dirs(&root, target_free_space);
    println!("{:?}", used_space);
    println!("{:?}", dirs);
    let free_space = total_disk_space - used_space;

    dirs.sort_by(|a, b| a.1.cmp(&b.1));
    let size = dirs
        .iter()
        .find(|(_, size)| size > &(target_free_space - free_space))
        .unwrap()
        .1;
    println!("{:?}", size);
    size
}

/// Day 7: No Space Left On Device
fn main() {
    if let Ok(contents) = fs::read_to_string("inputs/day_7") {
        part_1(&mut contents.lines());
        part_2(&mut contents.lines());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&mut INPUT.lines()), 95437)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&mut INPUT.lines()), 24933642)
    }
}
