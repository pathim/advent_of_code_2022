mod aoc2022;
mod aoc_result;
mod input;

pub use aoc_result::AocResult;

use clap::Parser;

type AocFun = fn(std::fs::File) -> AocResult;

fn wrap_func(func: &AocFun, input: Result<std::fs::File, input::Error>) -> AocResult {
    match input {
        Ok(input) => func(input),
        Err(error) => format!("{:?}", error).into(),
    }
}

fn output_result<T: FnOnce() -> AocResult>(res: T) -> std::time::Duration {
    let time_begin = std::time::SystemTime::now();
    println!("{}", res());
    println!("--- Time ---");
    let time = time_begin.elapsed().unwrap_or_default();
    println!("{}s", time.as_secs_f64());
    time
}

fn output_all_results(year: i32, funcs: &[AocFun]) {
    let start = std::time::SystemTime::now();
    let mut longest_time = (0, std::time::Duration::from_secs(0));
    for (i, res) in get_results(year, funcs) {
        println!("==== Day {} ====", i + 1);
        let time = output_result(res);
        if time > longest_time.1 {
            longest_time = (i + 1, time);
        }
        println!();
    }
    println!(
        "Total time: {}s",
        start.elapsed().unwrap_or_default().as_secs_f64()
    );
    println!(
        "Longest day: {} with {}s",
        longest_time.0,
        longest_time.1.as_secs_f64()
    )
}
fn get_results(
    year: i32,
    funcs: &[AocFun],
) -> impl Iterator<Item = (usize, Box<impl FnOnce() -> AocResult + '_>)> {
    let input = input::get_all_inputs(year);
    let results = funcs
        .iter()
        .zip(input)
        .map(|(func, input)| Box::new(move || wrap_func(func, input)))
        .enumerate();
    results
}

fn output_last_result(year: i32, funcs: &[AocFun]) {
    if let Some((i, res)) = get_results(year, funcs).last() {
        println!("==== Day {} ====", i + 1);
        output_result(res);
        println!();
    }
}

fn output_single_result(day: usize, year: i32, funcs: &[AocFun]) {
    if let Some((i, res)) = get_results(year, funcs).nth(day - 1) {
        println!("==== Day {} ====", i + 1);
        output_result(res);
        println!();
    } else {
        println!("Day {} not available", day);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // calculate all days
    #[arg(short, long)]
    pub all: bool,

    // day to run
    #[arg(short, long)]
    pub day: Option<usize>,
}

fn main() {
    let year = 2022;
    let args = Args::parse();
    let funcs2022 = aoc2022::get_funcs();
    if args.all {
        output_all_results(year, &funcs2022);
    } else if let Some(day) = args.day {
        output_single_result(day, year, &funcs2022);
    } else {
        output_last_result(year, &funcs2022);
    }
}
