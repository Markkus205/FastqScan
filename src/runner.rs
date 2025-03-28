use std::io::{self, BufRead};
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FastqRecord {
    pub seq: Vec<u8>,
    pub qual: Vec<u8>,
}


pub trait Statistic {
    /* Statistics:
     * average base quality (Phred)
     * average quality of all reads
     * average proportions of `{A, C, G, T, N}` for each read position
     * ...
     */

    fn process(&mut self, record: &FastqRecord);
}
pub trait ToJson {
    fn to_json(&self) -> String; //mut writer -> result unit
}
// as any as extra trait?
pub trait StatisticWithJson: Statistic + ToJson {}

impl<T: Statistic + ToJson> StatisticWithJson for T {}

pub struct WorkflowRunner {
    pub statistics: Vec<Box<dyn StatisticWithJson>>, //change to public
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
            //println!("{:?}", record);
            for statistic in self.statistics.iter_mut() {
                statistic.process(&record);
            }
        }
    }

    // Read data for a complete FASTQ record from `read`.
    fn parse_record<R>(read: &mut R, record: &mut FastqRecord) -> io::Result<()>
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
            // make sure record is empty and then fill it with the bytes from the buffer
            match i {
                1 => {
                    record.seq.clear(); 
                    record.seq.extend(buffer.trim_end().as_bytes());
                }
                3 => {
                    record.qual.clear(); 
                    record.qual.extend(buffer.trim_end().as_bytes());
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn finalize(self) -> Vec<Box<dyn StatisticWithJson>> {
        // Move out the statistics, effectively preventing the future use of the runner.
        self.statistics
    }
}
