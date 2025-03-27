mod runner;
mod qual;
mod statistics;

use runner::WorkflowRunner;
use crate::statistics::avproportion::AverageProportionsStatistic;
use crate::statistics::avbase::AvBaseQualityStatistic;
use qual::*;


use clap::Parser;

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

    // Finalize and retrieve statistics
    /*let statistics = runner.finalize();
    for stat in statistics {
        // Print or process the results of each statistic
        println!("{:?}", stat);
    }*/
}

//cargo run -- -1 data/test.R1.fastq -2 data/test.R2.fastq