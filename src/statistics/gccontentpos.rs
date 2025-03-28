use crate::statistics::avproportion::BaseCounts;
use crate::runner::*;
/* GC-Gehalt (pro Base)

Der GC-Gehalt definiert sich als prozentualer Anteil der Nukleotidbasen Guanin und Cytosin an der Gesamtheit der vier Basen (Adenin, Thymin, Guanin, Cytosin) der DNA. 
Sie den Wikipedia-Eintrag. Geben Sie den durchschnittlichen GC-Gehalt für jede Position der Reads aus.

average G/C content per read position

*/

impl ToJson for CGContentPosStatistic {
    fn to_json(&self) -> String {
        if self.base_counts.is_empty() {
            return serde_json::json!({
                "gc_content_per_position": []
            })
            .to_string();
        }

        let gc_content_per_position: Vec<f64> = self.base_counts
            .iter()
            .map(|counts| {
                let total = counts.get_total();
                if total > 0 {
                    
                    (counts.c + counts.g) as f64 / total as f64
                } else {
                    0.0
                }
            })
            .collect();

        serde_json::json!({
            "gc_content_per_position": gc_content_per_position
        })
        .to_string()
    }
}

pub fn gc_content_per_position(reads: Vec<FastqRecord>) -> Vec<f64>{
    if reads.is_empty(){
        return Vec::new();
    }

    let read_length = reads.iter().map(|record| record.seq.len()).max().unwrap_or(0);
    let mut base_counts: Vec<BaseCounts> = vec![BaseCounts::new(); read_length];

    for record in reads {
        for ( i, &base) in record.seq.iter().enumerate(){
            base_counts[i].update(base);
        }
    }

    base_counts
        .iter()
        .map(|counts| {
            let total = counts.get_total();
            if total > 0 {
                (counts.c + counts.g) as f64 / total as f64
            } else {
                0.0
            }
        })
        .collect()
}

// runner

/// Computes average G/C content per read position.
pub struct CGContentPosStatistic {
    base_counts: Vec<BaseCounts>, // Store cumulative counts
    max_length: usize,            // Track longest read length
}

impl CGContentPosStatistic {
    pub fn new() -> Self {
        Self {
            base_counts: Vec::new(),
            max_length: 0,
        }
    }

    pub fn results(&self) -> Vec<f64> {
        self.base_counts
            .iter()
            .map(|counts| {
                let total = counts.get_total();
                if total > 0 {
                    (counts.c + counts.g) as f64 / total as f64
                } else {
                    0.0
                }
            })
            .collect()
    }
}

impl Statistic for CGContentPosStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let read_length = record.seq.len();
        if read_length > self.max_length {
            self.max_length = read_length;
            self.base_counts.resize_with(read_length, BaseCounts::new);
        }

        for (i, &base) in record.seq.iter().enumerate() {
            self.base_counts[i].update(base);
        }
    }
}
//////////// in the main
//Box::new(CGContentPosStatistic { position_cg_content: Vec::new() }),

#[cfg(test)]
mod test {
    use super::{gc_content_per_position, CGContentPosStatistic};
    use crate::runner::{FastqRecord, Statistic};

    fn test_reads() -> Vec<FastqRecord> {
        vec![
            FastqRecord {
                seq: b"ACGT".to_vec(),
                qual: b"IIII".to_vec(),  
            },
            FastqRecord {
                seq: b"GGCC".to_vec(),
                qual: b"IIII".to_vec(),
            },
            FastqRecord {
                seq: b"ATGC".to_vec(),
                qual: b"IIII".to_vec(),
            },
        ]
    }

    #[test]
    fn test_gc_content_per_position() {
        let reads = test_reads();
        let results = gc_content_per_position(reads);
        let expected: Vec<f64> = vec![0.333, 0.666, 1.0, 0.666];
        for (computed, expected) in results.iter().zip(expected.iter()) {
            assert!((computed - expected).abs() < 0.01, "Mismatch at position");
        }
    }


    #[test]
    fn test_cg_content_pos_statistic() {
        let mut statistic = CGContentPosStatistic::new();
        let reads = test_reads();

        for record in reads.iter() {
            statistic.process(record);
        }

        let results = statistic.results();

        // Using the same expected values as above
        let expected: Vec<f64> = vec![0.333, 0.666, 1.0, 0.666];

        for (computed, expected) in results.iter().zip(expected.iter()) {
            assert!((computed - expected).abs() < 0.01, "Mismatch at position");
        }
    }
}