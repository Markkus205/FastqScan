mod runner;
mod qual;
mod statistics;

use runner::*;
use serde_json::to_string_pretty;
use crate::statistics::avproportion::AverageProportionsStatistic;
use crate::statistics::avbase::AvBaseQualityStatistic;
use crate::statistics::gcperread::GCContentStatistic;
use crate::statistics::length::ReadLengthStatistic;
use crate::statistics::gccontentpos::CGContentPosStatistic;

use qual::*;
use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let args = Args::parse();

    // Process Read 1
    process_and_save(&args.read1, "output_read1.json");
    
    // Process Read 2 (if provided)
    if let Some(read2_path) = args.read2 {
        process_and_save(&read2_path, "output_read2.json");
    }
}

fn process_and_save(read_path: &PathBuf, output_filename: &str) {
    if let Ok(buf_reader) = decompress_gz_file(read_path.to_str().unwrap()) {
        println!("\nProcessing {:?}...", read_path);
        let mut runner = WorkflowRunner {
            statistics: vec![
                Box::new(AvBaseQualityStatistic { mean: 0., count: 0 }),
                Box::new(AverageProportionsStatistic { ave_prop: Vec::new() }),
                Box::new(GCContentStatistic::new()),
                Box::new(ReadLengthStatistic::new()),
                Box::new(CGContentPosStatistic::new()),
            ],
        };
        runner.process(buf_reader);
        
        // Finalize statistics
        let statistics = runner.finalize();
        let json_outputs: Vec<serde_json::Value> = statistics.iter().map(|stat| serde_json::from_str(&stat.to_json()).unwrap()).collect();
        let formatted_json = to_string_pretty(&json_outputs).expect("Failed to format JSON");
        
        // Print to stdout with formatted JSON
        println!("\nStatistics for {:?}:\n", read_path);
        println!("{}\n", formatted_json);
        
        // Write to output file
        let mut file = File::create(output_filename).expect("Failed to create output file");
        file.write_all(formatted_json.as_bytes()).expect("Failed to write to output file");
        println!("Statistics written to {}\n", output_filename);
    } else {
        eprintln!("\nError decompressing {:?}\n", read_path);
    }
}

//cargo run -- -1 data/test.R1.fastq -2 data/test.R2.fastq