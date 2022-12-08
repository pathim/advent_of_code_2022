use std::io::BufRead;

fn check_visible(trees: &[Vec<u8>], x: usize, y: usize) -> bool {
    let tree_height = trees[y][x];
    if trees[y][..x].iter().all(|x| *x < tree_height) {
        return true;
    }
    if trees[y][x + 1..].iter().all(|x| *x < tree_height) {
        return true;
    }
    if trees[..y].iter().map(|v| v[x]).all(|x| x < tree_height) {
        return true;
    }
    if trees[y + 1..].iter().map(|v| v[x]).all(|x| x < tree_height) {
        return true;
    }
    false
}

fn get_tree_score(trees: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let tree_height = trees[y][x];
    let mut left = 0;
    let mut right = 0;
    let mut up = 0;
    let mut down = 0;
    for th in trees[y][x + 1..].iter() {
        right += 1;
        if *th >= tree_height {
            break;
        }
    }
    for th in trees[y][..x].iter().rev() {
        left += 1;
        if *th >= tree_height {
            break;
        }
    }
    for th in trees[..y].iter().rev() {
        let th = th[x];
        up += 1;
        if th >= tree_height {
            break;
        }
    }
    for th in trees[y + 1..].iter() {
        let th = th[x];
        down += 1;
        if th >= tree_height {
            break;
        }
    }
    right * left * up * down
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut trees = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        let row = line
            .trim()
            .chars()
            .map(|c| c as u8 - b'0')
            .collect::<Vec<_>>();
        trees.push(row);
    }
    let mut res1 = 0;
    let mut res2 = 0;
    let width = trees[0].len();
    let height = trees.len();
    for y in 0..height {
        for x in 0..width {
            res1 += check_visible(&trees, x, y) as u32;
            res2 = res2.max(get_tree_score(&trees, x, y));
        }
    }
    (res1, res2).into()
}
