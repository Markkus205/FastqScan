use crate::runner::Statistic;
use crate::runner::*;

impl ToJson for BaseQualityPosStatistic {
    fn to_json(&self) -> String {
        serde_json::json!({
            "avg_base_quality": self.results()
        })
        .to_string()
    }
}

// Konvertiert ASCII-Symbole in Phred-Scores
// if it is out of range ?!
pub fn calculate_phred(qual: u8) -> u8 {
    (qual - 33) as u8
}

#[derive(Debug, Clone)]
pub struct QualityCounts {
    sum_quality: u64,
    count: u64,
}

impl QualityCounts {
    pub fn new() -> Self {
        QualityCounts { sum_quality: 0, count: 0 }
    }

    pub fn update(&mut self, quality: u8) {
        self.sum_quality += quality as u64;
        self.count += 1;
    }

    pub fn get_average(&self) -> f64 {
        if self.count > 0 {
            self.sum_quality as f64 / self.count as f64
        } else {
            0.0
        }
    }
}


pub fn avg_base_quality(reads: Vec<FastqRecord>) -> Vec<f64> {
    // in case reads is empty
    // -> number_of_reads != 0 (division in map)
    if reads.is_empty() {
        return Vec::new();
    }

    let read_length: usize = reads.iter().map(|record| record.qual.len()).max().unwrap_or(0); 

    let mut quality_counts: Vec<QualityCounts> = vec![QualityCounts::new(); read_length];

    for record in reads {
        for (i, &qual) in record.qual.iter().enumerate() {
            let phred_value = calculate_phred(qual);
            quality_counts[i].update(phred_value);
        }
    }

    //Compute average per position
    quality_counts.iter().map(|qc| qc.get_average()).collect()
    }
     


//////// for the runner.rs
/// Computes mean base quality for a position read.
pub struct BaseQualityPosStatistic {
    reads: Vec<FastqRecord>,
    position_avg_qualities: Vec<f64>,
    
}

impl BaseQualityPosStatistic {
    pub fn new() -> Self {
        Self {
            reads: Vec::new(),
            position_avg_qualities: Vec::new(),
        }
    }

    pub fn results(&self) -> Vec<f64> {
        
        if self.position_avg_qualities.is_empty() && !self.reads.is_empty() {
            return avg_base_quality(self.reads.clone());        }
        
        self.position_avg_qualities.clone()
    }
}

impl Statistic for BaseQualityPosStatistic {
    fn process(&mut self, record: &FastqRecord) {
        self.reads.push(record.clone());
    }

}


//////////// in the main
//Box::new(BaseQualityPosStatistic { position_avg_qualities: Vec::new() }),



#[cfg(test)]
mod test {

    use super::{avg_base_quality, calculate_phred, BaseQualityPosStatistic};
    use crate::runner::{FastqRecord, Statistic};

    // Create a helper function to generate test FASTQ records.
    fn test_reads() -> Vec<FastqRecord> {
        vec![
            FastqRecord {
                seq: b"ACGT".to_vec(),
                qual: b"AAA!".to_vec(),
            },
            FastqRecord {
                seq: b"ACGT".to_vec(),
                qual: b"BBB!".to_vec(),
            },
            FastqRecord {
                seq: b"ACGT".to_vec(),
                qual: b"xxx!".to_vec(),
            },
        ]
    }

    // Test for the BaseQualityPosStatistic trait implementation
    #[test]
    fn test_base_quality_pos_statistic() {
        
        let mut statistic = BaseQualityPosStatistic::new();

        let reads = test_reads();

        for record in reads.iter() {
            statistic.process(record);
        }

        let results = statistic.results();

        let expected: Vec<f64> = vec![50.666666666666664, 50.666666666666664, 50.666666666666664, 0.0];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_calculate_phred() {
        let qual: u8 = b'&'; // byte representation
        let expected: u8 = 5; // Phred-Score für das Zeichen '&'
        let res = calculate_phred(qual);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_calculate_phred_range() {
        for ascii in 33..=126 {
            // Alle ASCII-Werte von '!' bis '~'
            let qual = ascii as u8;
            let expected = (ascii - 33) as u8;
            assert_eq!(calculate_phred(qual), expected);
        }
    }

    // Alternativ können viele Ergebnisse auf einmal getestet werden:
    #[test]
    fn test_calculate_phred_other() {
        let tests: Vec<(u8, u8)> = vec![(b'&', 5), (b'+', 10)];
        for test in tests {
            let res = calculate_phred(test.0);
            assert_eq!(test.1, res);
        }
    }
}
