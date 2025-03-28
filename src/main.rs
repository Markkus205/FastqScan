mod runner;
mod qual;
mod statistics;

use runner::*;

use crate::statistics::avproportion::AverageProportionsStatistic;
use crate::statistics::avbase::AvBaseQualityStatistic;
use crate::statistics::gc_per_read::GCContentStatistic;
use crate::statistics::length::ReadLengthStatistic;

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
            Box::new(GCContentStatistic {
                gc_percentages: Vec::new(),
                total_gc: 0,
                total_bases: 0,
            }),
            Box::new(ReadLengthStatistic::new()),

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
        json_outputs.push(stat.to_json());//where traits clash
    }

    // to do: format it "normally"
    let combined_json = format!("[{}]", json_outputs.join(","));

    let output_file_path = "output.json";
    let mut file = File::create(output_file_path).expect("Failed to create output file");
    file.write_all(combined_json.as_bytes())
        .expect("Failed to write to output file");

    println!("Statistics written to {}", output_file_path);
}
//add standard output
//only one file being output?

//cargo run -- -1 data/test.R1.fastq -2 data/test.R2.fastq