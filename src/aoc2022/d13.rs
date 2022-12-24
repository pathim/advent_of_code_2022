use std::{io::BufRead, str::FromStr};

fn find_inner(s: &str) -> &str {
    let mut count = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '[' {
            count += 1;
        } else if c == ']' {
            count -= 1;
            if count == 0 {
                return &s[1..i];
            }
        }
    }
    panic!("No closing bracket in '{}'", s);
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Value(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::List(l) => {
                write!(f, "[").ok();
                let mut first = true;
                for v in l.iter() {
                    if !first {
                        write!(f, ",").ok();
                    }
                    first = false;
                    write!(f, "{}", v).ok();
                }
                write!(f, "]")
            }
        }
    }
}

impl Packet {
    fn parse_one(mut s: &str) -> Option<(Self, &str)> {
        if s.is_empty() {
            return None;
        }
        if s.starts_with(',') {
            s = &s[1..];
        }
        if s.starts_with('[') {
            let mut inner = find_inner(s);
            let rest = &s[inner.len() + 2..];
            let mut list = Vec::new();
            while let Some((v, r)) = Self::parse_one(inner) {
                list.push(v);
                inner = r;
            }
            return Some((Self::List(list), rest));
        } else {
            let mut value = 0;
            for (i, c) in s.chars().enumerate() {
                if c.is_numeric() {
                    value *= 10;
                    value += (c as u8 - b'0') as i32;
                } else {
                    return Some((Self::Value(value), &s[i + 1..]));
                }
            }
            Some((Self::Value(value), ""))
        }
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_one(s)
            .map(|x| x.0)
            .ok_or(String::from("Invalid line"))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Value(v1), Self::Value(v2)) => return v1.partial_cmp(v2),
            (Self::List(l1), Self::List(l2)) => {
                let mut index = 0;
                loop {
                    let v1 = l1.get(index);
                    let v2 = l2.get(index);
                    index += 1;
                    match (v1, v2) {
                        (None, None) => return Some(std::cmp::Ordering::Equal),
                        (Some(_), None) => return Some(std::cmp::Ordering::Greater),
                        (None, Some(_)) => return Some(std::cmp::Ordering::Less),
                        (Some(x1), Some(x2)) => {
                            let c = x1.partial_cmp(x2);
                            if matches!(c, Some(std::cmp::Ordering::Equal)) {
                                continue;
                            }
                            return c;
                        }
                    }
                }
            }
            (p1 @ Self::List(_), Self::Value(x)) => {
                return p1.partial_cmp(&Self::List(vec![Self::Value(*x)]))
            }
            (Self::Value(x), p2 @ Self::List(_)) => {
                return Self::List(vec![Self::Value(*x)]).partial_cmp(p2)
            }
        }
    }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut lines = input.lines();
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    let div6 = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);
    let mut packets = vec![div2.clone(), div6.clone()];
    let mut idx = 0;
    let mut res1 = 0;
    loop {
        idx += 1;
        let l1 = lines.next().unwrap().unwrap();
        let l2 = lines.next().unwrap().unwrap();
        let p1: Packet = l1.parse().unwrap();
        let p2: Packet = l2.parse().unwrap();
        if p1 < p2 {
            res1 += idx;
        }
        packets.push(p1);
        packets.push(p2);
        if lines.next().is_none() {
            break;
        }
    }
    packets.sort();
    let i1 = packets.binary_search(&div2).unwrap() + 1;
    let i2 = packets.binary_search(&div6).unwrap() + 1;
    (res1, i1 * i2).into()
}
