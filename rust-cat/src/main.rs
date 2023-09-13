use std::fs;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// File to read
    files: Vec<String>
}
fn main() {
    let args = Cli::parse();

    // Check that file exists
    let files: Vec<String> = args.files;

    for f in files{
        let file_contents: String = match fs::read_to_string(&f) {
            Ok(data) => data,
            Err(error_msg) => {
                println!("rust-cat: {}: {}", f, error_msg.to_string());
                std::process::exit(1);
            }
        };

        print!("{}", file_contents);
    }

}
