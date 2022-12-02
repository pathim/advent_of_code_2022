use std::io::BufRead;

#[derive(Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

fn get_mine(l: &str) -> RPS {
    if l.contains('X') {
        RPS::Rock
    } else if l.contains('Y') {
        RPS::Paper
    } else {
        RPS::Scissor
    }
}

fn get_theirs(l: &str) -> RPS {
    if l.contains('A') {
        RPS::Rock
    } else if l.contains('B') {
        RPS::Paper
    } else {
        RPS::Scissor
    }
}

fn get_score(mine: RPS, theirs: RPS) -> i32 {
    let win = theirs as i32 - mine as i32;
    let win = (win + 3) % 3;
    let score: i32 = match win {
        0 => 3,
        1 => 0,
        2 => 6,
        _ => panic!("Invalid score"),
    };
    score + mine as i32
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let res1: i32 = input
        .lines()
        .map(|x| x.unwrap())
        .map(|l| (get_theirs(&l), get_mine(&l)))
        .map(|(t, m)| get_score(m, t))
        .sum();
    (format!("{}", res1), None)
}
