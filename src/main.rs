mod parser;
use clap::Parser;
use parser::*;
use std::fs::File;

// & phredscore = 5 | fehler% = 10^-5/10 = ~0.316 (jede base hat ca 30% fehlerquote) * 1000 = ~316 erwartet sind ca 300 fehler

fn main() {
    let args = Args::parse();

    // Process first read
    match process_fastq(&args.read1) {
        Ok(avg_qual_r1) => println!("Average Quality Score (Read 1): {:.2}", avg_qual_r1),
        Err(e) => eprintln!("Error processing Read 1: {}", e),
    }

    // Process second read if provided
    if let Some(read2_path) = args.read2 {
        match process_fastq(&read2_path) {
            Ok(avg_qual_r2) => println!("Average Quality Score (Read 2): {:.2}", avg_qual_r2),
            Err(e) => eprintln!("Error processing Read 2: {}", e),
        }
    }
}
