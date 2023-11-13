use std::collections::HashMap;
use std::fs;
use std::env;

#[derive(Debug)]
enum Command<'a> {
    Ls(Vec<Object<'a>>),
    Cd(&'a str)
}

#[derive(Debug)]
enum Object<'a> {
    Dir(&'a str),
    File(&'a str, usize)
}

#[derive(Clone, Debug)]
enum Size {
    Unknown,
    Known(usize)
}

#[derive(Clone)]
struct Directory {
    parent: Option<String>,
    dirs: Vec<String>,
    dirs_size: Size,
    file_size: Size,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() == 1 { "input.txt" } else { &args[1] };
    let input = fs::read_to_string(file_path).expect("Unable to read file");
    let commands: Vec<Command> = parse_input(&input);
    let mut directories: HashMap<String, Directory>= HashMap::new();
    directories.insert(String::from("/"), Directory {
        parent: None,
        dirs: Vec::new(),
        dirs_size: Size::Unknown,
        file_size: Size::Unknown
    });
    let mut current_directory = String::from("/");
    for command in commands {
        match command {
            Command::Ls(objects) => {
                for object in objects {
                    match object {
                        Object::Dir(s) => {
                            directories.insert(String::from(current_directory.clone() + s), Directory {
                                parent: Some(current_directory.to_owned()),
                                dirs: Vec::new(),
                                dirs_size: Size::Unknown,
                                file_size: Size::Unknown,
                            });
                            directories.entry(current_directory.to_owned()).and_modify(|e| {
                                e.dirs.push(current_directory.clone() + s)
                            });
                        },
                        Object::File(_, sz) => {
                            directories.entry(current_directory.to_owned()).and_modify(|e| {
                                match e.file_size {
                                    Size::Unknown => {
                                        e.file_size = Size::Known(sz);                                        
                                    }
                                    Size::Known(s) => {
                                        e.file_size = Size::Known(s + sz);
                                    }
                                }
                            });
                        }
                    }
                }
                check_dir_size(current_directory.to_owned(), &mut directories);
            },
            Command::Cd(s) => {
                if s == ".." {
                    let p = directories.get(&current_directory).unwrap();
                    let a = &p.parent;
                    match a {
                        Some(b) => {
                            current_directory = b.to_owned();
                        },
                        None => panic!("cd .. on directory with no parent"),
                    }
                } else if s == "/" {
                    current_directory = s.to_owned();
                } else {
                    current_directory = current_directory + s;
                }
            }
        }
    }
    let ans: usize = directories.values()
        .map(|d| {
            match d.dirs_size {
                Size::Known(s) => s,
                Size::Unknown => panic!("Unknown directory size")
            }
        })
        .filter(|&sz| {
            sz <= 100000
        })
        .sum();
    println!("ans: {}", ans);
}

fn check_dir_size(dir: String, directories: &mut HashMap<String, Directory>) {
    if let Size::Known(_) = directories.get(&dir).unwrap().dirs_size {
        return;
    }
    let dirs = &directories.get(&dir).unwrap().dirs;
    let mut cur_size = 0;
    for d in dirs {
        if let Size::Known(s) = directories.get(d).unwrap().dirs_size {
            cur_size += s;
        } else {
            return;
        }
    }
    directories.entry(dir.clone()).and_modify(|e| {
        if let Size::Known(fsz) = e.file_size {
            e.dirs_size = Size::Known(cur_size + fsz);
        } else {
            e.dirs_size = Size::Known(cur_size);
        }
    });
    if let Some(ref p) = directories.get(&dir).unwrap().parent {
        check_dir_size(p.to_owned(), directories)
    }
}

fn parse_input<'a>(input: &'a str) -> Vec<Command<'a>> {
    let v: Vec<_> = input.split('$')
        .filter(|&w| !w.is_empty())
        .map(|w| w.trim().lines().collect::<Vec<_>>())
        .map(|c| -> Command {
            if c.len() == 1 {
                // cd command
                Command::Cd(&c[0][3..])
            } else {
                // ls command
                let objects: Vec<Object> = c[1..].iter().map(|&s| {
                    let words: Vec<&str> = s.split_whitespace().collect();
                    if words[0] == "dir" {
                        Object::Dir(words[1])
                   } else {
                        let size: usize = words[0].parse().expect("Unable to parse file size");
                        Object::File(words[1], size)
                   }
                }).collect();
                Command::Ls(objects)
            }
        })
        .collect();
    v
}