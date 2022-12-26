use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Pos(i64, i64);

impl FromStr for Pos {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("No comma")?;
        let (_, x) = x.split_once("x=").ok_or("No x=")?;
        let x = x.parse().map_err(|_| "Cannot parse number")?;
        let (_, y) = y.split_once("y=").ok_or("No y=")?;
        let y = y.parse().map_err(|_| "Cannot parse number")?;
        Ok(Self(x, y))
    }
}

impl Pos {
    fn dist(&self, other: &Pos) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug, Clone)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, beacon) = s.split_once(':').ok_or("No colon")?;
        let pos = pos.parse()?;
        let beacon = beacon.parse()?;
        Ok(Self { pos, beacon })
    }
}

impl Sensor {
    fn on_row(&self, row: i64) -> Option<Range> {
        let x = self.pos.0;
        let dist = self.pos.dist(&self.beacon);
        let mut d = self.pos.dist(&Pos(x, row));
        if d >= dist {
            return None;
        }
        let mut start = x;
        let mut end = x;
        while d < dist {
            start -= 1;
            end += 1;
            d += 1;
        }
        if self.beacon.1 == row {
            if self.beacon.0 == start {
                start += 1;
            }
            if self.beacon.0 == end {
                end -= 1;
            }
        }
        Some(Range(start, end))
    }
}

#[derive(Debug, Clone)]
struct Range(i64, i64);
impl Range {
    fn overlap(&self, other: &Self) -> bool {
        let (large, small) = self.max_min(other);
        (small.0 >= large.0 && small.0 <= large.1) || (small.1 >= large.0 && small.1 <= large.1)
    }
    fn len(&self) -> i64 {
        self.1 - self.0 + 1
    }
    fn max_min(&self, other: &Self) -> (Self, Self) {
        if self.len() > other.len() {
            (self.clone(), other.clone())
        } else {
            (other.clone(), self.clone())
        }
    }
    fn union(&self, other: &Self) -> Range {
        Range(self.0.min(other.0), self.1.max(other.1))
    }
}

fn insert_range(r1: &mut Vec<Range>, r2: Range) {
    for r in r1.iter_mut() {
        if r.overlap(&r2) {
            *r = r.union(&r2);
            return;
        }
    }
    r1.push(r2);
}

fn compress(r1: &mut Vec<Range>) {
    'outer: loop {
        for i in 0..r1.len() - 1 {
            for j in i + 1..r1.len() {
                if r1[i].overlap(&r1[j]) {
                    let rj = r1.remove(j);
                    let ri = r1.remove(i);
                    r1.push(rj.union(&ri));
                    continue 'outer;
                }
            }
        }
        return;
    }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let sensors: Vec<Sensor> = input.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let mut ranges = sensors.iter().filter_map(|s| s.on_row(2000000)).collect();
    compress(&mut ranges);
    let res1: i64 = ranges.iter().map(|r| r.len()).sum();
    res1.into()
}
