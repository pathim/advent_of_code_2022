use std::io::BufRead;

fn get_file_sizes(lines: &mut impl Iterator<Item = String>) -> (usize, Option<String>) {
    let mut size = 0;
    for l in lines {
        if l.starts_with('$') {
            return (size, Some(l));
        }
        let (s1, _) = l.split_once(' ').unwrap();
        if !s1.starts_with('d') {
            size += s1.parse::<usize>().unwrap();
        }
    }
    (size, None)
}

fn get_dir_sizes(lines: &mut impl Iterator<Item = String>) -> (usize, Vec<usize>) {
    let mut line = lines.next();
    let mut size = 0;
    let mut child_sizes = Vec::new();
    while let Some(l) = &line {
        if l.starts_with("$ ls") {
            let (file_size, nl) = get_file_sizes(lines);
            size += file_size;
            if nl.is_none() {
                break;
            }
            line = nl;
        } else if l.starts_with("$ cd ..") {
            break;
        } else if l.starts_with("$ cd") {
            let (dsize, csize) = get_dir_sizes(lines);
            child_sizes.extend(csize);
            child_sizes.push(dsize);
            size += dsize;
            line = lines.next();
        } else {
            panic!("Illegal command {}", l);
        }
        if line.is_none() {
            break;
        }
    }
    (size, child_sizes)
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut lines = input.lines().map(|x| x.unwrap());
    let l = lines.next().unwrap();

    if l.starts_with("$ cd") {
        let (size, mut csize) = get_dir_sizes(&mut lines);
        csize.push(size);
        csize.sort();
        let needed_size = 30000000 - (70000000 - size);
        let res1: usize = csize.iter().filter(|&x| *x <= 100000).sum();
        let res2 = csize.iter().find(|&x| *x >= needed_size).unwrap();

        return (res1, res2).into();
    }

    unreachable!()
}
