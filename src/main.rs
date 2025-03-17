
#[cfg(test)]
pub fn calculate_phred(qual: char) -> usize {
    let phred = (qual as usize) - 33;
    return phred;
}
#[cfg(test)]
pub fn read_qual(qual_string : &str) -> usize {
    let n =qual_string.len();
    let mut qual_sum = 0;
    for qual in qual_string.chars() {
        let q = calculate_phred(qual);
        qual_sum += q;
    }
    return qual_sum/n;
}

pub fn explain_data(data_name: &str){
    let split : Vec<&str> = data_name
        .split(|c| c == ' ' || c == ':' || c == '.' || c == '_')
        .collect();
    println!("{:?}", split);
    if split[5] ==  "fastq" {
        println!("Probenname, der im Probenblatt für den Sequenzierungslauf angegeben ist: {}", split[0]);
        println!("Nukleotidsequenz des molekularen Barcodes, der zur Markierung der Probe für das Multiplexing verwendet wird: {}", split[1]);
        println!("Die Lane-Nummer (1--8): {}", split[2]);
        println!("Die Read-Nummer: {}", split[3]);
        println!("set number: {}", split[4]);

    }
    else {
        println!("Die eindeutige Gerätebezeichnung: {}", split[0]);
        println!("Die Lauf-ID: (dies ist das {} ste Mal, dass dieses Gerät betrieben wurde)", split[1]);
        println!("Flowcell-ID: {}", split[2]);
        println!("Flowcell-Lane (Spur: 1–8): {}", split[3]);
        println!("Tile-Nummer innerhalb der Lane: {}", split[4]);
        println!("X-Koordinate des Clusters innerhalb des Tiles (d.h., der ``Kachel''): {}", split[5]);
        println!("Y-Koordinate des Clusters innerhalb des Tiles: {}", split[6]);
        println!("Mitglied eines Paares (1 oder 2; 2 kann nur für Paired-End- oder Mate-Pair-Sequenzierung verwendet werden): {}", split[7]);
        println!("Y: Read hat den Chastity-Filter verletzt (solche Reads können herausgefiltert oder in der FASTQ-Datei belassen werden); N: Read hat den Keuschheitsfilter nicht verletzt: {}", split[8]);
        println!("0, wenn keines der Kontrollbits aktiviert ist, andernfalls ist es eine gerade Zahl. Auf HiSeq X- und NextSeq-Systemen wird die Kontrollspezifikation nicht durchgeführt und diese Zahl ist immer 0.: {}", split[9]);
        println!("Indexsequenz (Barcode): {}", split[10]);
        }
}

fn main() {
    println!("Hello, world!");
    let data_name = "@HWI-D00107:50:H6BP8ACWV:5:2204:10131:51624 2:N:0:AGGCAGAA";
    explain_data(data_name);
}
// & phredscore = 5 | fehler% = 10^-5/10 = ~0.316 (jede base hat ca 30% fehlerquote) * 1000 = ~316 erwartet sind ca 300 fehler
pub mod test {
    use super::read_qual;
    use super::calculate_phred;
    use super::explain_data;
    
    #[test]
    fn test_calculate_phred() {
        let qual: char = '&';
        let expected:usize =	5; // Phred-Score für das Zeichen '&'
        let res = calculate_phred(qual);
        assert_eq!(expected, res);
    }

    // Alternativ können viele Ergebnisse auf einmal getestet werden:
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
    #[test]
    fn test_explain_data() {
        let data_name = "@HWI-D00107:50:H6BP8ACWV:5:2204:10131:51624 2:N:0:AGGCAGAA";
        explain_data(data_name);
        let data_name2 = "NIST7035_TAAGGCGA_L001_R1_001.fastq.gz";
        explain_data(data_name2);
    }
    // ggf. andere Testfunktionen
}
