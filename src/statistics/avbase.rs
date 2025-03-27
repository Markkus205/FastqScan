use crate::runner::Statistic;
use crate::runner::FastqRecord;
use crate::runner::ToJson;
use serde::Serialize;

#[derive(Serialize)]
pub struct AvBaseQualityStatistic {
    pub mean: f64, 
    #[serde(skip_serializing)] 
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
}
impl ToJson for AvBaseQualityStatistic {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
#[cfg(test)]
pub mod test {
    use super::*;

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

    #[test]
    fn test_av_base_quality_statistic_json_output() {
        let av_stat = AvBaseQualityStatistic { mean: 42.0, count: 100 };
        let json_output = av_stat.to_json();
        let expected_json = r#"{"mean":42.0}"#;
        println!("JSON Output: {}", json_output);
        assert_eq!(json_output, expected_json);
    }
}