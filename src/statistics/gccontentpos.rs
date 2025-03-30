use crate::statistics::avproportion::BaseCounts;
use crate::runner::*;
/* GC-Gehalt (pro Base)

Der GC-Gehalt definiert sich als prozentualer Anteil der Nukleotidbasen Guanin und Cytosin an der Gesamtheit der vier Basen (Adenin, Thymin, Guanin, Cytosin) der DNA. 
Sie den Wikipedia-Eintrag. Geben Sie den durchschnittlichen GC-Gehalt für jede Position der Reads aus.

average G/C content per read position

*/

impl ToJson for CGContentPosStatistic {
    fn to_json(&self) -> String {
        serde_json::json!({
            "gc_content_per_position": self.results()
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
/// Computes average G/C content per read position.
pub struct CGContentPosStatistic {
    position_cg_content: Vec<f64>,  // GC content per position
    read_count: usize,              // Track the number of reads processed
    max_length: usize,              // Track the length of the longest read
}

impl CGContentPosStatistic {
    pub fn new() -> Self {
        Self {
            position_cg_content: Vec::new(),
            read_count: 0,
            max_length: 0,
        }
    }

    fn results(&self) -> Vec<f64> {
        if self.read_count == 0 {
            return Vec::new();
        }

        // Normalize by dividing by the number of processed reads
        self.position_cg_content
            .iter()
            .map(|&gc| gc / self.read_count as f64)
            .collect()
    }
}

impl Statistic for CGContentPosStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let read_length = record.seq.len();

        // Update max_length if we encounter a longer read
        if read_length > self.max_length {
            self.max_length = read_length;
            self.position_cg_content.resize(read_length, 0.0); // Resize vector if needed
        }

        // Compute GC content for this specific read
        let single_read_gc_content = gc_content_per_position(vec![record.clone()]);

        // Sum up the GC content at each position
        for (i, &gc) in single_read_gc_content.iter().enumerate() {
            if i < self.position_cg_content.len() {
                self.position_cg_content[i] += gc;
            }
        }

        // Increment the read count
        self.read_count += 1;
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