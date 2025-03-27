use crate::runner::Statistic;
use crate::runner::FastqRecord;

pub struct AvBaseQualityStatistic {
    pub mean: f64,  
    pub count: u32, 
}
/// Computes mean base quality for a read.
impl Statistic for AvBaseQualityStatistic {
    fn process(&mut self, record: &FastqRecord) {
        for &q in record.qual.iter() {
            let phred_score = (q - 33) as f64; 
            self.count += 1;
            self.mean += (phred_score - self.mean) / self.count as f64; // Update rolling mean
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub mod test {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader};
    #[test]
    fn test_av_base_quality_statistic_with_rolling_mean() {
        // Create a sample FastqRecord
        let record = FastqRecord {
            seq: b"AGCT".to_vec(), 
            qual: b"IIII".to_vec(), 
        };

        let mut av_stat = AvBaseQualityStatistic { mean: 0.0, count: 0 };

        av_stat.process(&record);

        println!("Final Rolling Mean: {:.2}", av_stat.mean);
        assert!((av_stat.mean - 40.0).abs() < 0.01); // Allow small rounding errors
    }
}