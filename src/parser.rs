use clap::Parser;
use std::{fs::File, path::PathBuf};
use std::io::{self, BufRead, BufReader};
use flate2::read::GzDecoder;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    
    #[arg(short = '1' , long)]
    pub read1: PathBuf,


    #[arg(short = '2' , long)]
    pub read2: Option<PathBuf>,
}
//change these to patchbuf 
pub fn explain_data(data_name: &str){
    let split : Vec<&str> = data_name
        .split(|c| c == ' ' || c == ':' || c == '.' || c == '_')
        .collect();
    println!("{:?}", split);
    println!("{:?}", split.len());
    println!("Probenname, der im Probenblatt für den Sequenzierungslauf angegeben ist: {}", split[0]);
    println!("Nukleotidsequenz des molekularen Barcodes, der zur Markierung der Probe für das Multiplexing verwendet wird: {}", split[1]);
    println!("Die Lane-Nummer (1--8): {}", split[2]);
    println!("Die Read-Nummer: {}", split[3]);
    println!("set number: {}", split[4]);
}
    /*
    else {
        println!("Die eindeutige Gerätebezeichnung: {}", split[0]);
        println!("Die Lauf-ID: (dies ist das {} ste Mal, dass dieses Gerät betrieben wurde)", split[1]);
        println!("Flowcell-ID: {}", split[2]);
        println!("Flowcell-Lane (Spur: 1–8): {}", split[3]);
        println!("Tile-Nummer innerhalb der Lane: {}", split[4]);
        println!("X-Koordinate des Clusters innerhalb des Tiles (d.h., der ``Kachel''): {}", split[5]);
        println!("Y-Koordinate des Clusters innerhalb des Tiles: {}", split[6]);
        println!("Mitglied eines Paares (1 oder 2; 2 kann nur für Paired-End- oder Mate-Pair-Sequenzierung verwendet werden): {}", split[7]);
        println!("Y: Read hat den Chastity-Filter verletzt (solche Reads können herausgefiltert oder in der FASTQ-Datei belassen werden); N: Read hat den Keuschheitsfilter nicht verletzt: {}", split[8]);
        println!("0, wenn keines der Kontrollbits aktiviert ist, andernfalls ist es eine gerade Zahl. Auf HiSeq X- und NextSeq-Systemen wird die Kontrollspezifikation nicht durchgeführt und diese Zahl ist immer 0.: {}", split[9]);
        println!("Indexsequenz (Barcode): {}", split[10]);
        }
        */

//split into 2.

pub fn calculate_phred(qual: u8) -> u8 {
    qual - 33
}
//check lowerbound -> produce none
pub fn read_qual(qual_string: &[u8]) -> f64 {
    let n = qual_string.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    let qual_sum: f64 = qual_string.iter().map(|&q| calculate_phred(q) as f64).sum();
    qual_sum / n
}
//overflow error
//benchmark?

pub fn process_fastq(file_path: &PathBuf) -> io::Result<f64> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);

    let mut total_quality = 0.0;
    let mut read_count = 0;

    for line in buf_reader.lines().skip(3).step_by(4) {  // Skip to every 4th line
        if let Ok(qual_line) = line {
            println!("Processing line: {} \n", qual_line);
            total_quality += read_qual(qual_line.as_bytes());
            read_count += 1;
        }
    }

    if read_count == 0 {
        return Ok(0.0);
    }

    Ok(total_quality / read_count as f64) // Return the average quality across all reads
}
//missing decompression


pub mod test {
    use super::*;
    #[test]
    fn test_process_fastq() {
        let file_path = PathBuf::from("data/test.R1.fastq");
        let res = process_fastq(&file_path).unwrap();
        let expected = 39.72222222222222; //((8+18)*40 + 10*39) / (18 +18)
        assert_eq!(expected, res);
    }

    #[test] 
    fn test_explain_data() {
        //t data_name = "@HWI-D00107:50:H6BP8ACWV:5:2204:10131:51624 2:N:0:AGGCAGAA";
        //plain_data(data_name);
        let data_name2 = "NIST7035_TAAGGCGA_L001_R1_001.fastq.gz";
        explain_data(data_name2);
    }
    #[test]
    fn test_calculate_phred_other() {
        let tests: Vec<(u8, u8)> = vec![
            (b'&', 5),
            (b'+', 10)
        ];
        for test in tests {
            let res = calculate_phred(test.0);
            assert_eq!(test.1, res);
        }
    }
    #[test]
    fn test_read_qual() {
        let qual_string = b"&&+"; //5,5,10 /3 = ~6
        let res = read_qual(qual_string);
        let expected = 6.666666666666667;
        print!("res :{} ", res);
        assert_eq!(expected, res);
    }

}