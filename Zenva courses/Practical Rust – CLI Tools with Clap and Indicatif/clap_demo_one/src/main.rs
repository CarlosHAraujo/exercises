use clap::{Arg,Command};

fn main() {
    let matches = Command::new("clap demo one")
        .version("1.0")
        .author("Zenva Academy")
        .about("Process images")
        .arg(
            Arg::new("input")
            .short('i')
            .long("input")
            .value_name("DIRECTORY")
            .help("Sets the input file directory")
            .required(true)
        )
        .arg(
            Arg::new("output")
            .short('o')
            .long("output")
            .value_name("DIRECTORY")
            .help("Sets the output file directory")
            .required(true)
        )
        .get_matches();

    let input = matches.get_one::<String>("input").expect("Input file directory is required");
    let output = matches.get_one::<String>("output").expect("Output file directory is required");

    println!("Input file directory: {}", input);
    println!("Output file directory: {}", output);
}
