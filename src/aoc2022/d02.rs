use std::{
    io::BufRead,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug)]
struct Ring3(i32);

impl Ring3 {
    pub fn points(self) -> i32 {
        self.0 + 2
    }
}

impl From<i32> for Ring3 {
    fn from(v: i32) -> Self {
        let v = v.rem_euclid(3);
        Self(if v == 2 { -1 } else { v })
    }
}

impl Add for Ring3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl Sub for Ring3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}
fn get_mine(l: &str) -> Ring3 {
    if l.contains('X') {
        -1
    } else if l.contains('Y') {
        0
    } else {
        1
    }
    .into()
}

fn get_theirs(l: &str) -> Ring3 {
    if l.contains('A') {
        -1
    } else if l.contains('B') {
        0
    } else {
        1
    }
    .into()
}

fn get_score(mine: Ring3, theirs: Ring3) -> i32 {
    let win = mine - theirs;
    let score = win.0;
    score * 3 + 3 + mine.points()
}

fn get_score_from_result(theirs: Ring3, result: Ring3) -> i32 {
    let mine = result + theirs;
    get_score(mine, theirs)
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let (res1, res2) = input
        .lines()
        .map(|x| x.unwrap())
        .map(|l| (get_theirs(&l), get_mine(&l)))
        .map(|(t, m)| (get_score(m, t), get_score_from_result(t, m)))
        .fold((0, 0), |(r1, r2), (x1, x2)| (r1 + x1, r2 + x2));
    (format!("{}", res1), Some(format!("{}", res2)))
}
