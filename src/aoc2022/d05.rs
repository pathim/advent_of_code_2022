use std::{io::BufRead, str::FromStr};

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        parts.next(); //move
        let count = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|e| format!("{}", e))?;
        parts.next(); //from
        let from = parts
            .next()
            .unwrap()
            .parse::<usize>()
            .map_err(|e| format!("{}", e))?
            - 1;
        parts.next(); //to
        let to = parts
            .next()
            .unwrap()
            .parse::<usize>()
            .map_err(|e| format!("{}", e))?
            - 1;

        Ok(Self { from, to, count })
    }
}

impl Move {
    pub fn execute(&self, stacks: &mut [Vec<char>]) {
        for _ in 0..self.count {
            let val = stacks[self.from].pop().unwrap();
            stacks[self.to].push(val);
        }
    }
    pub fn execute2(&self, stacks: &mut [Vec<char>]) {
        let pos = stacks[self.from].len() - self.count;
        for _ in 0..self.count {
            let val = stacks[self.from].remove(pos);
            stacks[self.to].push(val);
        }
    }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut stacks = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new())
    }
    let mut lines = input.lines().map(Result::unwrap);
    for l in lines.by_ref() {
        if !l.contains('[') {
            break;
        }
        for (i, c) in l.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            if (i - 1) % 4 == 0 {
                stacks[(i - 1) / 4].push(c);
            }
        }
    }
    for s in stacks.iter_mut() {
        s.reverse();
    }
    let mut stacks2 = stacks.clone();
    lines.next();
    for l in lines {
        let mov: Move = l.parse().unwrap();
        mov.execute(&mut stacks);
        mov.execute2(&mut stacks2);
    }
    let res1: String = stacks.iter().map(|x| x.last().unwrap()).collect();
    let res2: String = stacks2.iter().map(|x| x.last().unwrap()).collect();

    (res1, res2).into()
}
