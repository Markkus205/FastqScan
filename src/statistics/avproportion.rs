use crate::runner::Statistic;
use crate::runner::FastqRecord;


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