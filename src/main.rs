mod runner;
mod qual;
mod statistics;

use runner::*;

use crate::statistics::avproportion::AverageProportionsStatistic;
use crate::statistics::avbase::AvBaseQualityStatistic;
use qual::*;
use clap::Parser;
use std::fs::File;
use std::io::Write;

fn main() {
    let args = Args::parse();

    // Initialize WorkflowRunner with statistics
    let mut runner = WorkflowRunner {
        statistics: vec![
            Box::new(AvBaseQualityStatistic { mean: 0., count: 0 }),
            Box::new(AverageProportionsStatistic { ave_prop: Vec::new() }),
        ],
    };

    // Process the first read
    if let Ok(buf_reader) = decompress_gz_file(args.read1.to_str().unwrap()) {
        println!("Processing Read 1...");
        runner.process(buf_reader);
    } else {
        eprintln!("Error decompressing Read 1.");
    }

    // Process the second read if provided
    if let Some(read2_path) = args.read2 {
        if let Ok(buf_reader) = decompress_gz_file(read2_path.to_str().unwrap()) {
            println!("Processing Read 2...");
            runner.process(buf_reader);
        } else {
            eprintln!("Error decompressing Read 2.");
        }
    }


    let statistics = runner.finalize();


    let mut json_outputs = Vec::new();
    for stat in statistics {
        json_outputs.push(stat.to_json());
    }

    // to do: format it more nicely
    let combined_json = format!("[{}]", json_outputs.join(","));

    let output_file_path = "output.json";
    let mut file = File::create(output_file_path).expect("Failed to create output file");
    file.write_all(combined_json.as_bytes())
        .expect("Failed to write to output file");

    println!("Statistics written to {}", output_file_path);
}

//cargo run -- -1 data/test.R1.fastq -2 data/test.R2.fastq