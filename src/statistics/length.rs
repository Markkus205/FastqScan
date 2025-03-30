use crate::runner::*;

pub struct ReadLengthStatistic {
    pub lengths: Vec<usize>,
    pub total_length: usize,
}

impl ToJson for ReadLengthStatistic {
    fn to_json(&self) -> String {
        if self.lengths.is_empty() {
            return serde_json::json!({
                "min_length": 0,
                "max_length": 0,
                "average_length": 0.0,
                "total_length": 0,
                "lengths": []
            })
            .to_string();
        }

        let min_length = *self.lengths.iter().min().unwrap();
        let max_length = *self.lengths.iter().max().unwrap();
        let avg_length = self.lengths.iter().sum::<usize>() as f64 / self.lengths.len() as f64;

        serde_json::json!({
            "min_length": min_length,
            "max_length": max_length,
            "average_length": avg_length,
            "total_length": self.total_length,
            "lengths": self.lengths
        })
        .to_string()
    }
}

impl ReadLengthStatistic {
    pub fn new() -> Self {
        ReadLengthStatistic { lengths: Vec::new(), total_length: 0 }
    }
}

impl Statistic for ReadLengthStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let length = record.seq.len();
        self.lengths.push(length);
        self.total_length += length;
    }
}
#[allow(dead_code)]
impl ReadLengthStatistic {
    pub fn report(&self) {
        if self.lengths.is_empty() {
            println!("No reads processed.");
            return;
        }

        let min_length = *self.lengths.iter().min().unwrap();
        let max_length = *self.lengths.iter().max().unwrap();
        let avg_length = self.lengths.iter().sum::<usize>() as f64 / self.lengths.len() as f64;

        println!("Read Length Statistics:");
        println!("Min length: {}", min_length);
        println!("Max length: {}", max_length);
        println!("Average length: {:.2}", avg_length);
        println!("Total length: {}", self.total_length);
    }
}