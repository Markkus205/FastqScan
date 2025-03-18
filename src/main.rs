mod parser;
mod phred;
mod explain;

use clap::Parser;

use explain::*;
use parser::*;
use phred::*;
use std::fs::File;

fn main() {
    let args = Args::parse();
    read(&args);
    println!("Hello, world!");
    let data_name = "@HWI-D00107:50:H6BP8ACWV:5:2204:10131:51624 2:N:0:AGGCAGAA";
    explain_data(data_name);
}
// & phredscore = 5 | fehler% = 10^-5/10 = ~0.316 (jede base hat ca 30% fehlerquote) * 1000 = ~316 erwartet sind ca 300 fehler