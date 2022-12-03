use std::io::BufRead;

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut cal = Vec::new();
    let mut current = 0;
    for line in input.lines() {
        let line = line.expect("Read error");
        let line = line.trim();
        if line.is_empty() {
            cal.push(current);
            current = 0;
            continue;
        }
        let val: u32 = line.parse().expect("Not an integer");
        current += val;
    }
    cal.push(current);
    cal.sort();

    let res1 = cal.pop().expect("No values available");
    let res2 = res1 + cal.pop().unwrap() + cal.pop().unwrap();

    (res1, res2).into()
}
