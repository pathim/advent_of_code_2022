use std::{io::BufRead, str::FromStr};

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => return Err("Invalid Operator".into()),
        })
    }
}

impl Op {
    fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    value: Option<i64>,
    op: Op,
    rhs: String,
    lhs: String,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Ok(v) = s.parse() {
            return Ok(Monkey {
                value: Some(v),
                op: Op::Add,
                rhs: "".to_string(),
                lhs: "".to_string(),
            });
        }
        let mut parts = s.split_ascii_whitespace();
        let lhs = parts.next().unwrap().to_string();
        let op = parts.next().unwrap().parse()?;
        let rhs = parts.next().unwrap().to_string();

        Ok(Monkey {
            value: None,
            op,
            rhs,
            lhs,
        })
    }
}

impl Monkey {
    fn eval(&self, monkeys: &std::collections::HashMap<String, Monkey>) -> i64 {
        if let Some(v) = self.value {
            v
        } else {
            self.op.eval(
                monkeys[&self.lhs].eval(monkeys),
                monkeys[&self.rhs].eval(monkeys),
            )
        }
    }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut monkeys = std::collections::HashMap::new();
    for l in input.lines() {
        let l = l.unwrap();
        let (name, value) = l.split_once(':').unwrap();
        let monkey: Monkey = value.parse().unwrap();
        monkeys.insert(name.to_string(), monkey);
    }
    let res1 = monkeys["root"].eval(&monkeys);
    res1.into()
}
