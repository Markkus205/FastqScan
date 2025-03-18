use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    
    #[arg(short = '1' , long)]
    pub read1: String,


    #[arg(short = '2' , long)]
    pub read2: Option<String>,
}
use std::fs::File;
pub fn read(input: &Args) {
    let file_paths = vec![Some(&input.read1), input.read2.as_ref()];

    for file_path in file_paths.into_iter().flatten() {
        match File::open(file_path) {
            Ok(_) => println!("Successfully opened file: {}", file_path),
            Err(error) => panic!("Problem opening the file {}: {error:?}", file_path),
        }
    }
}


pub mod test {
    use super::*;
    #[test]
    fn test_path() {
        let file_path = Args {
            read1: "data/test.R1.fastq.gz".to_string(),
            read2: None, // Optional field can be set to None
        };
    
        read(&file_path);
    }
    #[test]
    fn test_two_paths() {
        let file_path = Args {
            read1: "data/test.R1.fastq.gz".to_string(),
            read2: Some("data/test.R1.fastq.gz".to_string()), // Optional field can be set to None
        };
    
        read(&file_path);
    }

}