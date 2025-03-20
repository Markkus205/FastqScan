use std::io::BufRead;

#[derive(Debug, Clone)]
struct BaseCounts {
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

    fn update(&mut self, base: char) {
        match base {
            'A' => self.a += 1,
            'C' => self.c += 1,
            'G' => self.g += 1,
            'T' => self.t += 1,
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

fn count_bases_per_position<R: BufRead>(reader: R) -> (Vec<BaseCounts>) {
    let mut counts: Vec<BaseCounts> = Vec::new();

    for line in reader.lines().skip(1).step_by(4) {
        let seq_line = line.unwrap();

        if counts.is_empty() {
            counts.resize(seq_line.len(), BaseCounts::new());
        }

        for (i, base) in seq_line.chars().enumerate() {
            counts[i].update(base);
        }
    }

    counts
}

fn calculate_proportions(counts: Vec<BaseCounts>) -> Vec<(f64, f64, f64, f64, f64)> {
    counts.iter().map(|count| count.get_proportions(count.get_total())).collect() //iter becasue we wanna reuse the Vec for the next functions
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    #[test]
    fn test_calculate_average_proportions_with_split() {
        let file_path = "data/test.R1.fastq";

        let file = File::open(file_path).expect("Failed to open test FASTQ file");
        let reader = BufReader::new(file);
        let counts = count_bases_per_position(reader);

        println!("Counts vector: {:?}", counts);

        // Calculate proportions
        let proportions = calculate_proportions(counts);

        println!("Proportions vector: {:?}", proportions);
    }
    
}
