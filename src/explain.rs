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

pub mod test {
    use super::*;
    #[test]
    fn test_explain_data() {
        let data_name = "@HWI-D00107:50:H6BP8ACWV:5:2204:10131:51624 2:N:0:AGGCAGAA";
        explain_data(data_name);
        let data_name2 = "NIST7035_TAAGGCGA_L001_R1_001.fastq.gz";
        explain_data(data_name2);
    }
}