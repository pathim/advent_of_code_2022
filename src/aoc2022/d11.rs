use std::io::BufRead;

#[derive(Clone, Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    Sqr,
}

impl Op {
    pub fn eval(&self, x: u64) -> u64 {
        match self {
            Self::Add(v) => x + v,
            Self::Mul(v) => x * v,
            Self::Sqr => x * x,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    div: u64,
    ift: usize,
    iff: usize,
    count: u64,
}

impl Monkey {
    fn parse(lines: &mut impl Iterator<Item = String>) -> Option<Self> {
        if let Some(l) = lines.next() {
            if !l.starts_with("Monkey") {
                panic!("Invalid start: {}", l);
            }
            let l = lines.next().unwrap();
            let (_, items) = l.split_once(":").unwrap();
            let items = items
                .trim()
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            let l = lines.next().unwrap();
            let (_, op) = l.split_once(":").unwrap();
            let op = if let Some((_, v)) = op.split_once("+") {
                Op::Add(v.trim().parse().unwrap())
            } else {
                if let Some((_, v)) = op.split_once("*") {
                    if let Ok(v) = v.trim().parse() {
                        Op::Mul(v)
                    } else {
                        Op::Sqr
                    }
                } else {
                    panic!("invalid operation {}", op)
                }
            };
            let l = lines.next().unwrap();
            let (_, test) = l.split_once(":").unwrap();
            let div = test.split_once("by ").unwrap().1.parse().unwrap();

            let l = lines.next().unwrap();
            let (_, t) = l.split_once(":").unwrap();
            let ift = t.split_once("monkey ").unwrap().1.parse().unwrap();
            let l = lines.next().unwrap();
            let (_, f) = l.split_once(":").unwrap();
            let iff = f.split_once("monkey ").unwrap().1.parse().unwrap();
            lines.next();

            Some(Self {
                items,
                op,
                div,
                ift,
                iff,
                count: 0,
            })
        } else {
            None
        }
    }

    pub fn throw(&mut self, part2: bool) -> Option<(u64, usize)> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items.remove(0);
        let new_item = self.op.eval(item);
        let new_item = if part2 {
            new_item % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19)
        } else {
            new_item / 3
        };

        let target = if new_item % self.div == 0 {
            self.ift
        } else {
            self.iff
        };
        self.count += 1;
        Some((new_item, target))
    }

    pub fn catch(&mut self, item: u64) {
        self.items.push(item)
    }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let mut input = std::io::BufReader::new(file).lines().map(|x| x.unwrap());
    let mut monkeys = Vec::new();
    while let Some(monkey) = Monkey::parse(&mut input) {
        monkeys.push(monkey);
    }
    let mut monkeys2 = monkeys.clone();

    for _ in 0..20 {
        for n in 0..monkeys.len() {
            while let Some((item, target)) = monkeys[n].throw(false) {
                monkeys[target].catch(item);
            }
        }
    }
    for _ in 0..10000 {
        for n in 0..monkeys2.len() {
            while let Some((item, target)) = monkeys2[n].throw(true) {
                monkeys2[target].catch(item);
            }
        }
    }
    monkeys.sort_by_key(|x| u64::MAX - x.count);
    monkeys2.sort_by_key(|x| u64::MAX - x.count);
    let res1 = monkeys[0].count * monkeys[1].count;
    let res2 = monkeys2[0].count * monkeys2[1].count;
    (res1, res2).into()
}
