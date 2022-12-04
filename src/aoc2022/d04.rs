use std::{io::BufRead, ops::RangeInclusive};

fn range_from_str(s: &str) -> RangeInclusive<u16> {
    let (start, end) = s.trim().split_once('-').unwrap();
    let start: u16 = start.parse().unwrap();
    let end: u16 = end.parse().unwrap();
    start..=end
}

fn ranges_from_str(s: &str) -> (RangeInclusive<u16>, RangeInclusive<u16>) {
    let (a, b) = s.split_once(',').unwrap();
    let r1 = range_from_str(a);
    let r2 = range_from_str(b);
    (r1, r2)
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut res1 = 0;
    let mut res2 = 0;
    for l in input.lines() {
        let (r1, r2) = ranges_from_str(&l.unwrap());
        let (long, short) = if r1.len() > r2.len() {
            (r1, r2)
        } else {
            (r2, r1)
        };
        let c1 = long.contains(short.start());
        let c2 = long.contains(short.end());
        if c1 && c2 {
            res1 += 1;
        }
        if c1 || c2 {
            res2 += 1;
        }
    }

    (res1, res2).into()
}
