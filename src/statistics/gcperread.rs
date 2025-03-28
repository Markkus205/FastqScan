use crate::runner::*;

pub struct GCContentStatistic {
    pub gc_percentages: Vec<f64>,
    pub total_gc: usize,
    pub total_bases: usize,
}

impl GCContentStatistic {
    pub fn new() -> Self {
        GCContentStatistic {
            gc_percentages: Vec::new(),
            total_gc: 0,
            total_bases: 0,
        }
    }
}
impl ToJson for GCContentStatistic {
    fn to_json(&self) -> String {
        if self.gc_percentages.is_empty() {
            return serde_json::json!({
                "total_gc_bases": 0,
                "total_bases": 0,
                "average_gc_percent": 0.0,
                "gc_percentages": []
            })
            .to_string();
        }

        let avg_gc = self.total_gc as f64 / self.total_bases as f64 * 100.0;

        serde_json::json!({
            "total_gc_bases": self.total_gc,
            "total_bases": self.total_bases,
            "average_gc_percent": avg_gc,
            "gc_percentages": self.gc_percentages
        })
        .to_string()
    }
}
impl Statistic for GCContentStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let gc_count = record.seq.iter().filter(|&&base| base == b'G' || base == b'C').count();
        let total_count = record.seq.len();
        if total_count > 0 {
            let gc_percentage = (gc_count as f64 / total_count as f64) * 100.0;
            self.gc_percentages.push(gc_percentage);
        }
        self.total_gc += gc_count;
        self.total_bases += total_count;
    }
}

impl GCContentStatistic {
    #[allow(dead_code)]
    pub fn report(&self) {
        if self.gc_percentages.is_empty() {
            println!("No reads processed.");
            return;
        }

        let avg_gc = self.total_gc as f64 / self.total_bases as f64 * 100.0;

        println!("GC Content Statistics:");
        println!("Total GC bases: {}", self.total_gc);
        println!("Total bases: {}", self.total_bases);
        println!("Average GC% per read: {:.2}%", avg_gc);
    }
}