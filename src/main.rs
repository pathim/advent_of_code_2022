mod aoc2022;
mod input;

type AocResult = (String, Option<String>);
type AocFun = fn(std::fs::File) -> AocResult;

fn wrap_func(func: &AocFun, input: Result<std::fs::File, input::Error>) -> AocResult {
    match input {
        Ok(input) => func(input),
        Err(error) => (format!("{:?}", error), None),
    }
}

fn output_result<T:FnOnce()->AocResult>(res: T) {
    let time_begin=std::time::SystemTime::now();
    let res=res();
    println!("--- Part 1 ---");
    println!("{}", res.0);
    if let Some(res) = res.1 {
        println!("--- part 2 ---");
        println!("{}", res);
    }
    println!("--- Time ---");
    println!("{}s",time_begin.elapsed().unwrap_or_default().as_secs_f64());
}

fn output_results(year: i32, funcs: & [AocFun]) {
    let results = get_results(year, funcs);
    let start = std::time::SystemTime::now();
    for (i, res) in results {
        println!("==== Day {} ====", i + 1);
        output_result(res);
        println!("");
    }
    println!(
        "Total time: {}s",
        start.elapsed().unwrap_or_default().as_secs_f64()
    );
}
fn get_results<'a>(year:i32,funcs:&'a [AocFun]) ->impl Iterator<Item=(usize,Box<impl FnOnce()->AocResult+'a>)>{
    let input = input::get_all_inputs(year);
    let results = funcs
        .into_iter()
        .zip(input)
        .map(|(func, input)| Box::new(move || wrap_func(func, input)))
        .enumerate();
        results
}

fn output_last_results(year: i32, funcs: & [AocFun]) {
    let results = get_results(year, funcs);
    for (i, res) in results.last() {
        println!("==== Day {} ====", i + 1);
        output_result(res);
        println!("");
    }
}

fn main() {
    let funcs2021 = aoc2021::get_funcs();
    let funcs2022 = aoc2022::get_funcs();
    output_last_results(2021, &funcs2021);
    output_results(2022, &funcs2022);
}
