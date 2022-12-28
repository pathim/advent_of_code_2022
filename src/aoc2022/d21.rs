use std::{fmt::Display, io::BufRead, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Val {
    Var,
    Num(i64),
    Op(Op, Box<Val>, Box<Val>),
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var => write!(f, "X"),
            Self::Num(v) => write!(f, "{}", v),
            Self::Op(o, v1, v2) => write!(f, "({} {} {})", v1, o, v2),
        }
    }
}

impl Val {
    fn simplify(self) -> Self {
        match self {
            Self::Op(o, v1, v2) => {
                let v1 = v1.simplify();
                let v2 = v2.simplify();
                match (v1, v2) {
                    (Self::Num(v1), Self::Num(v2)) => Self::Num(o.eval(v1, v2)),
                    (v1, v2) => Self::Op(o, Box::new(v1), Box::new(v2)),
                }
            }
            v => v,
        }
    }
    fn solve(self, other: Self) -> (Self, Self) {
        let res = match self {
            Self::Op(o, v1, v2) => match o {
                Op::Add => {
                    if v1.complexity() >= v2.complexity() {
                        (*v1, Val::Op(Op::Sub, Box::new(other), v2))
                    } else {
                        (*v2, Val::Op(Op::Sub, Box::new(other), v1))
                    }
                }
                Op::Sub => (*v1, Val::Op(Op::Add, Box::new(other), v2)),
                Op::Mul => {
                    if v1.complexity() >= v2.complexity() {
                        (*v1, Val::Op(Op::Div, Box::new(other), v2))
                    } else {
                        (*v2, Val::Op(Op::Div, Box::new(other), v1))
                    }
                }
                Op::Div => (*v1, Val::Op(Op::Mul, Box::new(other), v2)),
            },
            v => (v, other),
        };
        (res.0.simplify(), res.1.simplify())
    }

    fn complexity(&self) -> usize {
        match self {
            Self::Var => 0,
            Self::Num(_) => 0,
            Self::Op(_, v1, v2) => 1 + v1.complexity().max(v2.complexity()),
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
    fn to_val(&self, monkeys: &std::collections::HashMap<String, Monkey>) -> Val {
        if let Some(v) = self.value {
            Val::Num(v)
        } else {
            let rhs = Box::new(if self.rhs == "humn" {
                Val::Var
            } else {
                monkeys[&self.rhs].to_val(monkeys)
            });
            let lhs = Box::new(if self.lhs == "humn" {
                Val::Var
            } else {
                monkeys[&self.lhs].to_val(monkeys)
            });
            Val::Op(self.op, lhs, rhs)
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
    let mut lhs = monkeys[&monkeys["root"].lhs].to_val(&monkeys).simplify();
    let mut rhs = monkeys[&monkeys["root"].rhs].to_val(&monkeys).simplify();
    let (lhs, rhs) = loop {
        let (nlhs, nrhs) = lhs.clone().solve(rhs.clone());
        let (nrhs, nlhs) = nrhs.solve(nlhs);
        if nlhs == lhs {
            break (lhs, rhs);
        }
        lhs = nlhs;
        rhs = nrhs;
    };
    let res2 = match (lhs, rhs) {
        (Val::Var, Val::Num(v)) => v,
        (Val::Num(v), Val::Var) => v,
        _ => panic!("Could not solve rhs"),
    };
    (res1, res2).into()
}
