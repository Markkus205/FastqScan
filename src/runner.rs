use std::io::{self, BufRead};
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FastqRecord {
    seq: Vec<u8>,
    qual: Vec<u8>,
}


pub trait Statistic {
    /* Statistics:
     * average base quality (Phred)
     * average quality of all reads
     * average proportions of `{A, C, G, T, N}` for each read position
     * ...
     */

    fn process(&mut self, record: &FastqRecord);

    fn as_any(&self) -> &dyn std::any::Any;
}

// to do: Rolling mean for Average Base Quality, work over average base quailty so it give out %, find alternative in parse record for to_vec, add test function for average proportions. Go from there. 
pub struct AvBaseQualityStatistic {
    pub total: u32,
    pub count: u32,
}
//rolling mean 
impl Statistic for AvBaseQualityStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let qual_sum: f64 = record.qual
            .iter()
            .map(|&q| (q - 33) as f64) // Convert ASCII to Phred score
            .sum();
        self.count += record.qual.len() as u32;
        self.total += qual_sum as u32;
        print!("Count: {:?}\n", self.count);
        print!("Total: {:?} \n", self.total);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
//tests
/// Computes mean base quality for a read.
pub struct AverageProportionsStatistic {
    pub ave_prop: Vec<(BaseCounts)>,
}

impl Statistic for AverageProportionsStatistic {
    fn process(&mut self, record: &FastqRecord) {
        if self.ave_prop.len() < record.seq.len() {
            self.ave_prop.resize(record.seq.len(), BaseCounts::new());
        }


        for i in 0..record.seq.len() {
            self.ave_prop[i].update(record.seq[i]);
        }
        //add get proportions functions

        println!("Average Proportions Vector:");
        for (i, base_count) in self.ave_prop.iter().enumerate() {
            println!(
                "Position {}: A: {}, C: {}, G: {}, T: {}, N: {}",
                i, base_count.a, base_count.c, base_count.g, base_count.t, base_count.n
            );
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}


#[derive(Debug, Clone)]
pub struct BaseCounts {
    a: u64,
    c: u64,
    g: u64,
    t: u64,
    n: u64,
}


impl BaseCounts {
    fn new() -> Self {
        BaseCounts {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
            n: 0,
        }
    }

    fn update(&mut self, base: u8) {
        match base {
            65 => self.a += 1,
            67 => self.c += 1,
            71 => self.g += 1,
            84 => self.t += 1,
            _ => self.n += 1, // Ambiguous or unknown
        }
    }
    fn get_proportions(&self, total: u64) -> (f64, f64, f64, f64, f64) { //New version of this similar to update. Convert struct to f64?
        if total > 0 {
            (
                self.a as f64 / total as f64, 
                self.c as f64 / total as f64, 
                self.g as f64 / total as f64, 
                self.t as f64 / total as f64, 
                self.n as f64 / total as f64, 
            )
        } else {
            (0.0, 0.0, 0.0, 0.0, 0.0) 
        }
    }
    fn get_total(&self) -> u64 {
        self.a + self.c + self.g + self.t + self.n
    }
}
pub struct WorkflowRunner {
    pub statistics: Vec<Box<dyn Statistic>>,
}

impl WorkflowRunner {
    /// Process the FASTQ file.
    ///
    /// Can return an I/O error or other errors (not in the signature at this point)
    pub fn process<R>(&mut self, mut read: R)
    where
        R: BufRead,
    {
        let mut record = FastqRecord::default();
        while let Ok(()) = WorkflowRunner::parse_record(&mut read, &mut record) {
            println!("{:?}", record);
            for statistic in self.statistics.iter_mut() {
                statistic.process(&record);
            }
        }
    }

    // Read data for a complete FASTQ record from `read`.
    pub fn parse_record<R>(read: &mut R, record: &mut FastqRecord) -> io::Result<()>
    where
        R: BufRead,
    {
        let mut buffer = String::new();

        for i in 0..4 {
            buffer.clear();
            if read.read_line(&mut buffer)? == 0 {
                // If we reach EOF before reading 4 lines, return an error
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Incomplete FASTQ record"));
            }

            match i {
                1 => {
                    record.seq = buffer.trim_end().as_bytes().to_vec(); //make sure record is empty fill with bytes that are in buffer
                }
                3 => {
                     record.qual = buffer.trim_end().as_bytes().to_vec();
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn finalize(self) -> Vec<Box<dyn Statistic>> {
        // Move out the statistics, effectively preventing the future use of the runner.
        self.statistics
    }
}

pub mod test {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader};
    #[test]
    fn test_workflow() {
        let file = File::open("data/test.R1.fastq").expect("Failed to open test file");
        let buf_reader = BufReader::new(file);

        let mut runner = WorkflowRunner {
            statistics: vec![
                Box::new(AvBaseQualityStatistic { total: 0, count: 0 }),
                //add other types here for other tests
            ],
        };
        
        runner.process(buf_reader);
        let statistics = runner.finalize();
        for stat in statistics {
            if let Some(av_stat) = stat.as_any().downcast_ref::<AvBaseQualityStatistic>() {
                println!("AvBaseQualityStatistic: {}", av_stat.total as f64/av_stat.count as f64);
                assert!(av_stat.total as f64/av_stat.count as f64 == 39.72222222222222);
            }
            //add test for other types with new else if statment
        }
    }
}