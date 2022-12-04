use std::{
    io::BufRead,
    ops::{RangeInclusive, Sub},
};

fn range_from_str(s: &str) -> RangeInclusive<usize> {
    let (start, end) = s.trim().split_once('-').unwrap();
    let start: usize = start.parse().map_err(|e| ()).unwrap();
    let end: usize = end.parse().map_err(|e| ()).unwrap();
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

fn is_contained(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    let (long, short) = if range_len(r1) > range_len(r2) {
        (r1, r2)
    } else {
        (r2, r1)
    };
    long.contains(short.start()) && long.contains(short.end())
}

fn overlaps(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    let (long, short) = if range_len(r1) > range_len(r2) {
        (r1, r2)
    } else {
        (r2, r1)
    };
    long.contains(short.start()) || long.contains(short.end())
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let ranges = input
        .lines()
        .map(|l| l.unwrap())
        .map(|l| ranges_from_str(&l))
        .collect::<Vec<_>>();
    let res1 = ranges.iter().filter(|(a, b)| is_contained(a, b)).count();
    let res2 = ranges.iter().filter(|(a, b)| overlaps(a, b)).count();

    (res1, res2).into()
}
