use std::{io::BufRead, str::FromStr};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);
impl FromStr for Pos {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("Missing ','")?;
        let x = x.parse().map_err(|_| "Invalid number")?;
        let y = y.parse().map_err(|_| "Invalid number")?;
        Ok(Self(x, y))
    }
}

#[derive(Debug, Default)]
struct Field {
    block: std::collections::HashSet<Pos>,
    max_y: i32,
}

impl Field {
    fn add_barrier(&mut self, start: Pos, end: Pos) {
        if start.1 == end.1 {
            for i in start.0.min(end.0)..=start.0.max(end.0) {
                self.block.insert(Pos(i, start.1));
            }
        } else {
            for i in start.1.min(end.1)..=start.1.max(end.1) {
                self.block.insert(Pos(start.0, i));
            }
        }
        self.max_y = self.max_y.max(start.1).max(end.1);
    }
    fn drop_sand(&mut self) -> i32 {
        let mut pos = Pos(500, 0);
        'drops: loop {
            let nps = [
                Pos(pos.0, pos.1 + 1),
                Pos(pos.0 - 1, pos.1 + 1),
                Pos(pos.0 + 1, pos.1 + 1),
            ];
            for np in nps {
                if !self.block.contains(&np) && np.1 < self.max_y + 2 {
                    pos = np;
                    continue 'drops;
                }
            }
            self.block.insert(pos);
            return pos.1;
        }
    }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut field = Field::default();
    for line in input.lines() {
        let line = line.unwrap();
        let mut cur_vert = None;
        for vert in line.split(" -> ") {
            let pos: Pos = vert.parse().unwrap();
            if let Some(cv) = cur_vert {
                field.add_barrier(cv, pos);
            }
            cur_vert = Some(pos);
        }
    }
    let mut count = 0;

    let res1 = loop {
        let y = field.drop_sand();
        if y >= field.max_y {
            break count;
        }
        count += 1;
    };
    let res2 = loop {
        let y = field.drop_sand();
        if y == 0 {
            break count;
        }
        count += 1;
    };
    (res1, res2 + 2).into() // Why off by two?
}
