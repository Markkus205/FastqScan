pub fn calculate_phred(qual: char) -> usize {
    let phred = (qual as usize) - 33;
    return phred;
}

pub fn read_qual(qual_string : &str) -> usize {
    let n =qual_string.len();
    let mut qual_sum = 0;
    for qual in qual_string.chars() {
        let q = calculate_phred(qual);
        qual_sum += q;
    }
    return qual_sum/n;
}

pub mod test {
    use super::*;
    #[test]
    fn test_calculate_phred_other() {
        let tests: Vec<(char, usize)> = vec![
            ('&', 5),
            ('+', 10)
        ];
        for test in tests {
            let res = calculate_phred(test.0);
            assert_eq!(test.1, res);
        }
    }
    #[test]
    fn test_read_qual() {
        let qual_string = "&&+"; //5,5,10 /3 = ~6
        let res:usize = read_qual(qual_string);
        let expected:usize = 6;
        print!("res :{} ", res);
        assert_eq!(expected, res);
    }
}