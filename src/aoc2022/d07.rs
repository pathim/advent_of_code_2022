use std::collections::HashMap;
use std::io::BufRead;
#[derive(Debug)]
enum Entry {
    Dir(HashMap<String, Entry>),
    File(usize),
}

fn get_dir<'a>(fs: &'a mut Entry, pos: &[String]) -> &'a mut Entry {
    if pos.is_empty() {
        return fs;
    }
    if let Entry::Dir(hm) = fs {
        return get_dir(hm.get_mut(&pos[0]).unwrap(), &pos[1..]);
    }
    panic!("Not a directory");
}

fn get_dir_size(fs: &HashMap<String, Entry>) -> usize {
    let mut size = 0;
    for v in fs.values() {
        size += match v {
            Entry::Dir(hm) => get_dir_size(hm),
            Entry::File(s) => *s,
        };
    }
    size
}

fn get_dir_sizes(fs: &HashMap<String, Entry>) -> Vec<usize> {
    let mut res = Vec::new();
    for v in fs.values() {
        match v {
            Entry::Dir(hm) => {
                let ds = get_dir_size(hm);
                res.push(ds);
                res.extend(get_dir_sizes(hm));
            }
            Entry::File(_) => {}
        }
    }
    res
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut lines = input.lines().map(|x| x.unwrap());
    let mut fs = Entry::Dir(HashMap::new());
    let mut fs_pos = Vec::new();
    let mut line = lines.next();
    while line.is_some() {
        let l = line.take().unwrap();
        let l = l.trim();
        if l.starts_with('$') {
            let (_, cmd) = l.split_once(' ').unwrap();
            if cmd.starts_with('c') {
                let (_, dir) = cmd.split_once(' ').unwrap();
                if dir == ".." {
                    fs_pos.pop();
                } else if dir == "/" {
                    fs_pos.clear();
                } else {
                    fs_pos.push(dir.to_string());
                }
                line = lines.next();
            } else if cmd.starts_with('l') {
                loop {
                    line = lines.next();
                    if let Some(ref l) = line {
                        if l.starts_with('$') {
                            break;
                        }
                        let (size, name) = l.split_once(' ').unwrap();
                        let new_entry = if size.starts_with('d') {
                            Entry::Dir(HashMap::new())
                        } else {
                            Entry::File(size.parse().unwrap())
                        };
                        let dir = get_dir(&mut fs, &fs_pos);
                        if let Entry::Dir(hm) = dir {
                            hm.insert(name.to_string(), new_entry);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
    if let Entry::Dir(hm) = &fs {
        let total_size = get_dir_size(hm);
        let needed_size = 30000000 - (70000000 - total_size);
        let mut sizes = get_dir_sizes(hm);
        sizes.sort();
        let res1: usize = sizes.iter().filter(|&x| *x <= 100000).sum();
        let res2 = sizes.iter().find(|&x| *x >= needed_size).unwrap();

        (res1, res2).into()
    } else {
        panic!("Not a dir")
    }
}
