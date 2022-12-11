use std::{io::BufRead, str::FromStr};

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
struct Pos(i32, i32);

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl From<(i32, i32)> for Pos {
    fn from((x, y): (i32, i32)) -> Self {
        Self(x, y)
    }
}
impl From<Dir> for Pos {
    fn from(d: Dir) -> Self {
        Pos::from(match d {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        })
    }
}
impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Pos {
    pub fn absmax(&self) -> i32 {
        self.0.abs().max(self.1.abs())
    }

    pub fn clamp(&self) -> Self {
        Self(self.0.signum(), self.1.signum())
    }
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl FromStr for Dir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .chars()
            .next()
            .ok_or_else(|| "Direction is empty".to_string())?
        {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            x @ _ => Err(format!("'{}' is not a valid direction", x)),
        }
    }
}
impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::Up => write!(f, "Up"),
            Dir::Down => write!(f, "Down"),
            Dir::Left => write!(f, "Left"),
            Dir::Right => write!(f, "Right"),
        }
    }
}

struct RepeatMove {
    pub count: usize,
    pub dir: Dir,
}

impl FromStr for RepeatMove {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count) = s
            .split_once(' ')
            .ok_or_else(|| "Unable to split line into Dir and Count".to_string())?;
        let dir = dir.parse()?;
        let count = count
            .parse()
            .map_err(|_| "Unable to parse count".to_string())?;

        Ok(Self { dir, count })
    }
}
struct Rope {
    head: Pos,
    tail: Pos,
    visited: std::collections::HashSet<Pos>,
}

impl Rope {
    pub fn do_move(&mut self, m: Pos) -> Pos {
        self.head += m;
        self.move_tail()
    }

    fn move_tail(&mut self) -> Pos {
        let delta = self.head - self.tail;
        if delta.absmax() <= 1 {
            return Pos::default();
        }
        let delta = delta.clamp();
        self.tail += delta;
        self.visited.insert(self.tail);
        delta
    }
}

impl Default for Rope {
    fn default() -> Self {
        let mut visited = std::collections::HashSet::new();
        let p0 = Pos::default();
        visited.insert(p0);
        Self {
            head: p0,
            tail: p0,
            visited,
        }
    }
}

fn move_long_rope(rope: &mut [Rope], m: Pos) {
    let mut m = m;
    for r in rope {
        m = r.do_move(m);
    }
}
pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut rope: [Rope; 9] = Default::default();
    for m in input
        .lines()
        .map(|l| l.unwrap().parse::<RepeatMove>().unwrap())
    {
        for _ in 0..m.count {
            move_long_rope(&mut rope, m.dir.into())
        }
    }
    let res1 = rope[0].visited.len();
    let res2 = rope[8].visited.len();
    (res1, res2).into()
}
