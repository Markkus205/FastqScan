use crate::runner::Statistic;
use crate::runner::FastqRecord;
use crate::runner::ToJson;
use serde::Serialize;

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
    }
}

impl ToJson for AverageProportionsStatistic {
    fn to_json(&self) -> String {

        let proportions: Vec<(f64, f64, f64, f64, f64)> = self
            .ave_prop
            .iter()
            .map(|base_counts| {
                let total = base_counts.get_total();
                base_counts.get_proportions(total)
            })
            .collect();

        // Serialize the proportions into JSON
        serde_json::to_string(&proportions).unwrap()
    }
}


#[derive(Debug, Clone, Serialize)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_average_proportions_statistic_to_json() {
        let ave_prop_stat = AverageProportionsStatistic {
            ave_prop: vec![
                BaseCounts { a: 10, c: 20, g: 30, t: 40, n: 0 },
                BaseCounts { a: 5, c: 15, g: 25, t: 35, n: 0 },
            ],
        };
        let json_output = ave_prop_stat.to_json();

        // Corrected expected JSON output
        let expected_json = r#"[[0.1,0.2,0.3,0.4,0.0],[0.0625,0.1875,0.3125,0.4375,0.0]]"#;
        println!("JSON Output: {}", json_output);
        assert_eq!(json_output, expected_json);
    }
}