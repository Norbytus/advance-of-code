use clap::{ Arg, App };

type AocResult<T> = Result<T, DataSetParseError>;

#[derive(Debug)]
struct DataSetParseError(String);

fn main() {
    let matches = App::new("Advance of code 2021")
        .version("0.1")
        .author("Alex S - <norbyt93@gmail.com>")
        .arg(
            Arg::with_name("quest")
            .required(true)
            .short("q")
            .long("quest")
            .takes_value(true)
            .help("Run quest of the day example 1-0 run first of first day")
        )
        .arg(
            Arg::with_name("data")
            .short("i")
            .required(true)
            .long("input")
            .takes_value(true)
            .help("Set path to file for read data set")
        )
        .get_matches();

    let quest = matches.value_of("quest").expect("Required set day of quest");

    let file_path = matches.value_of("data").expect("Required set path to data set file");

    let data_set = std::fs::read_to_string(file_path).expect("File with data set not exist");

    match quest {
        "1-0" => {
            println!("Result: {}", d_1_0::count_increase(&parse_as_usize_vec(&data_set).expect("Error while parsing data set")));
        },
        "1-1" => {
            println!("Result: {}", d_1_0::sliding_window_increase(&parse_as_usize_vec(&data_set).expect("Error while parsing data set")));
        },
        _ => panic!("Undefined quest day"),
    }
}

fn parse_as_usize_vec(data: &str) -> AocResult<Vec<usize>> {
    let mut result = Vec::new();

    for line in data.lines() {
        match line.parse::<usize>() {
            Ok(v) => result.push(v),
            Err(e) => return Err(DataSetParseError(e.to_string())),
        }
    }

    Ok(result)
}
