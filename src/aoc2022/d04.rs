use std::{io::BufRead, ops::RangeInclusive};

fn range_from_str(s: &str) -> RangeInclusive<usize> {
    let (start, end) = s.trim().split_once('-').unwrap();
    let start: usize = start.parse().unwrap();
    let end: usize = end.parse().unwrap();
    start..=end
}

fn ranges_from_str(s: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let (a, b) = s.split_once(',').unwrap();
    let r1 = range_from_str(a);
    let r2 = range_from_str(b);
    (r1, r2)
}

fn range_len(r: &RangeInclusive<usize>) -> usize {
    r.end() - r.start() + 1
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut res1 = 0;
    let mut res2 = 0;
    for l in input.lines() {
        let (r1, r2) = ranges_from_str(&l.unwrap());
        let (long, short) = if range_len(&r1) > range_len(&r2) {
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
