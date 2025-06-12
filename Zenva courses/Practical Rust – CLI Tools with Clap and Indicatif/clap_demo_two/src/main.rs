use std::path::Path;

use clap::{Arg,Command};

fn main() {
    let matches = Command::new("DemoTwo")
        .version("2.0")
        .author("Zenva Academy")
        .about("Image Processor")
        .subcommand(
            Command::new("resize")
                .about("Resizes an image")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Sets the input file path")
                        .required(true)
                        .num_args(1..)
                        .value_parser(clap::builder::ValueParser::new(|file_path: &str| {
                            if Path::new(file_path).exists() {
                                Ok(file_path.to_string())
                            } else {
                                Err(String::from("Input file does not exist"))
                            }
                        }))
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Sets the output file path")
                        .required(true)
                )
                .arg(
                    Arg::new("width")
                        .short('w')
                        .long("width")
                        .value_name("WIDTH")
                        .help("Sets the new width")
                        .required(true)                        
                )
                .arg(
                    Arg::new("height")
                        .short('h')
                        .long("height")
                        .value_name("HEIGHT")
                        .help("Sets the new height")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("convert")
                .about("Converts an image")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Sets the input file path")
                        .required(true)
                        .num_args(1..)
                        .value_parser(clap::builder::ValueParser::new(|file_path: &str| {
                            if Path::new(file_path).exists() {
                                Ok(file_path.to_string())
                            } else {
                                Err(String::from("Input file does not exist"))
                            }
                        }))
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Sets the output file path")
                        .required(true)
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .value_name("FORMAT")
                        .help("Sets the output format (e.g., png, jpg)")
                        .required(true)                        
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("resize", sub_matches)) => {
            let input_files: Vec<_> = sub_matches
                .get_many::<String>("input")
                .expect("Iput files are required")
                .cloned()
                .collect();
            let output = sub_matches.get_one::<String>("output").expect("Output file is required.");
            let width = sub_matches.get_one::<String>("width").expect("Width file is required.");
            let height = sub_matches.get_one::<String>("height").expect("Height file is required.");

            for input_file in input_files {
                println!("Resizing {} to {}x{} and saving to {}", input_file, width, height, output);
            }
        },
        Some(("convert", sub_matches)) => {
            let input_files: Vec<_> = sub_matches
                .get_many::<String>("input")
                .expect("Iput files are required")
                .cloned()
                .collect();
            let output = sub_matches.get_one::<String>("output").expect("Output file is required.");
            let format = sub_matches.get_one::<String>("format").expect("Format is required.");
            
            for input_file in input_files {
                println!("Converting {} to {} and saving to {}", input_file, format, output);
            }
        },
        _ => println!("No subcommand was used")
    }
}
