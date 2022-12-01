mod aoc2022;
mod input;

use clap::Parser;

type AocResult = (String, Option<String>);
type AocFun = fn(std::fs::File) -> AocResult;

fn wrap_func(func: &AocFun, input: Result<std::fs::File, input::Error>) -> AocResult {
    match input {
        Ok(input) => func(input),
        Err(error) => (format!("{:?}", error), None),
    }
}

fn output_result<T: FnOnce() -> AocResult>(res: T) {
    let time_begin = std::time::SystemTime::now();
    let res = res();
    println!("--- Part 1 ---");
    println!("{}", res.0);
    if let Some(res) = res.1 {
        println!("--- part 2 ---");
        println!("{}", res);
    }
    println!("--- Time ---");
    println!(
        "{}s",
        time_begin.elapsed().unwrap_or_default().as_secs_f64()
    );
}

fn output_all_results(year: i32, funcs: &[AocFun]) {
    let results = get_results(year, funcs);
    let start = std::time::SystemTime::now();
    for (i, res) in results {
        println!("==== Day {} ====", i + 1);
        output_result(res);
        println!();
    }
    println!(
        "Total time: {}s",
        start.elapsed().unwrap_or_default().as_secs_f64()
    );
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

fn output_last_results(year: i32, funcs: &[AocFun]) {
    let results = get_results(year, funcs);
    if let Some((i, res)) = results.last() {
        println!("==== Day {} ====", i + 1);
        output_result(res);
        println!();
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // calculate all days
    #[arg(short, long)]
    pub all: bool,
}

fn main() {
    let args = Args::parse();
    let funcs2022 = aoc2022::get_funcs();
    if args.all {
        output_all_results(2022, &funcs2022);
    } else {
        output_last_results(2022, &funcs2022);
    }
}
