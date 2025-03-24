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

    // TODO - find a way to represent the results.
    // Let's try to identify the shared parts of *any* statistic
    // and report these in some fashion.
    // fn report(self) -> ?
}

/// Computes mean base quality for a position read.
pub struct BaseQualityPosStatistic {
    pub total: u32,
    pub count: u32,
    pub avg_phred: Option<f64>,
    pub avg_read_quality: Option<f64>,

}

impl Statistic for BaseQualityPosStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let qual_sum: f64 = record.qual
            .iter()
            .map(|&q| (q - 33) as f64) // Convert ASCII to Phred score
            .sum();
    
        self.avg_phred = Some(qual_sum / record.qual.len() as f64); //Check if this works
    
        self.count += 1;
        self.total += qual_sum as u32;

        print!("Avg Phred: {:?} \n", self.avg_phred);
        print!("Count: {:?}\n", self.count);
        print!("Total: {:?} \n", self.total);
    }
}

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
    fn get_proportions(&self, total: u64) -> (f64, f64, f64, f64, f64) { 
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

        // Read exactly four lines for a FASTQ record
        for i in 0..4 {
            buffer.clear();
            if read.read_line(&mut buffer)? == 0 {
                // If we reach EOF before reading 4 lines, return an error
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Incomplete FASTQ record"));
            }

            match i {
                1 => {
                    record.seq = buffer.trim_end().as_bytes().to_vec();
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