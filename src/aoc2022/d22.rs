use std::io::BufRead;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Coord(i32, i32);
impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl From<Dir> for Coord {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => Self(0, -1),
            Dir::Down => Self(0, 1),
            Dir::Left => Self(-1, 0),
            Dir::Right => Self(1, 0),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Clone, Copy, Default)]
enum Dir {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl From<Dir> for i32 {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        }
    }
}

impl Dir {
    fn turn(&self, d: &TurnDir) -> Self {
        match d {
            TurnDir::Cw => match self {
                Self::Up => Self::Right,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
                Self::Right => Self::Down,
            },
            TurnDir::Ccw => match self {
                Self::Up => Self::Left,
                Self::Down => Self::Right,
                Self::Left => Self::Down,
                Self::Right => Self::Up,
            },
        }
    }
}
#[derive(Debug)]
enum TurnDir {
    Cw,
    Ccw,
}
#[derive(Debug, Default)]
struct Map {
    data: std::collections::HashMap<Coord, Tile>,
    start: Coord,
    dir: Dir,
    pos: Coord,
    history: std::collections::HashMap<Coord, Dir>,
}
impl Map {
    fn insert_line(&mut self, s: &str, y: i32) {
        self.pos = self.start;
        for (i, c) in s.chars().enumerate() {
            let coord = Coord(i as i32 + 1, y);
            if self.data.is_empty() {
                self.start = coord;
            }
            match c {
                '.' => {
                    self.data.insert(coord, Tile::Open);
                }
                '#' => {
                    self.data.insert(coord, Tile::Wall);
                }
                _ => {}
            }
        }
    }
    fn next_tile(&self) -> Coord {
        let mut next_pos = self.pos + self.dir.into();
        if self.data.contains_key(&next_pos) {
            return next_pos;
        }
        loop {
            next_pos = next_pos - self.dir.into();
            if !self.data.contains_key(&next_pos) {
                return next_pos + self.dir.into();
            }
        }
    }
    fn step(&mut self) -> bool {
        self.record();
        let next_pos = self.next_tile();
        match self.data.get(&next_pos) {
            Some(Tile::Open) => {
                self.pos = next_pos;
                return false;
            }
            Some(Tile::Wall) => {
                return true;
            }
            None => panic!("Tried to step into the abyss"),
        }
    }
    fn record(&mut self) {
        self.history.insert(self.pos, self.dir);
    }
    fn execute(&mut self, cmd: &Command) {
        match cmd {
            Command::Move(n) => {
                for _ in 0..*n {
                    if self.step() {
                        break;
                    }
                }
            }
            Command::Turn(t) => {
                self.dir = self.dir.turn(t);
            }
        }
    }
    fn draw(&self) {
        let max_y = self.data.iter().max_by_key(|x| x.0 .1).unwrap().0 .1;
        for y in 1..=max_y {
            let mut started = false;
            for x in 1.. {
                let c = Coord(x, y);
                if c == self.pos {
                    print!("X");
                    continue;
                }
                if let Some(d) = self.history.get(&c) {
                    print!(
                        "{}",
                        match d {
                            Dir::Up => '^',
                            Dir::Down => 'v',
                            Dir::Left => '<',
                            Dir::Right => '>',
                        }
                    );
                    continue;
                }
                if let Some(v) = self.data.get(&c) {
                    started = true;
                    match v {
                        Tile::Open => {
                            print!(".");
                        }
                        Tile::Wall => {
                            print!("#");
                        }
                    }
                } else {
                    if started {
                        break;
                    } else {
                        print!(" ");
                    }
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
enum Command {
    Move(usize),
    Turn(TurnDir),
}
pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut map = Map::default();
    let mut lines = input.lines().enumerate();
    for (i, l) in lines.by_ref() {
        let l = l.unwrap();
        if l.is_empty() {
            break;
        }
        map.insert_line(&l, i as i32 + 1);
    }
    let path = lines.next().unwrap().1.unwrap();
    let mut cmds = Vec::new();
    let mut val = 0;
    let countR = path.chars().filter(|x| *x == 'R').count();
    let countL = path.chars().filter(|x| *x == 'L').count();
    for c in path.trim().chars() {
        if c.is_numeric() {
            val *= 10;
            val += (c as u8 - b'0') as usize;
        } else {
            cmds.push(Command::Move(val));
            val = 0;
            let turn = match c {
                'L' => TurnDir::Ccw,
                'R' => TurnDir::Cw,
                _ => panic!("Invalid Turn"),
            };
            cmds.push(Command::Turn(turn));
        }
    }
    for cmd in cmds.iter() {
        println!("pos: {:?}, dir:{:?}, cmd: {:?}", map.pos, map.dir, cmd);
        map.execute(&cmd);
    }
    map.record();

    map.draw();

    for h in map.history {
        if *map.data.get(&h.0).unwrap() != Tile::Open {
            panic!("Moved into Wall");
        }
    }

    println!(
        "Count R: {countR}, count L: {countL} => {}, dir is {:?}",
        (countR - countL) % 4,
        map.dir
    );
    println!("{:?}", map.pos);

    let res1: i32 = (1000 * map.pos.1 + 4 * map.pos.0) + <Dir as Into<i32>>::into(map.dir);
    res1.into()
}
