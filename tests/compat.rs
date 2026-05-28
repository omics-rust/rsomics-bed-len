use rsomics_bed_len::lengths;
use std::io::Cursor;

#[test]
fn basic_lengths() {
    let input = "chr1\t100\t200\tfeat1\nchr2\t0\t50\n";
    let mut out = Vec::new();
    lengths(Cursor::new(input), &mut out).unwrap();
    let s = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    // 4-column: chrom, start, end, rest, len
    assert!(lines[0].ends_with("\t100"), "feat1 len wrong: {}", lines[0]);
    // 3-column BED3: chrom, start, end, len
    assert!(lines[1].ends_with("\t50"), "chr2 len wrong: {}", lines[1]);
}

#[test]
fn header_passthrough() {
    let input = "# track name=test\nchr1\t0\t200\n";
    let mut out = Vec::new();
    lengths(Cursor::new(input), &mut out).unwrap();
    let s = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines[0], "# track name=test");
    assert!(lines[1].ends_with("\t200"));
}

#[test]
fn zero_length() {
    let input = "chr1\t500\t500\n";
    let mut out = Vec::new();
    lengths(Cursor::new(input), &mut out).unwrap();
    let s = String::from_utf8(out).unwrap();
    assert!(s.trim_end().ends_with("\t0"));
}

#[test]
fn awk_equiv() {
    // Verify against known awk-computed lengths
    let cases = [
        ("chr1", 100u64, 200u64, 100u64),
        ("chr2", 0, 1000, 1000),
        ("chrX", 999, 1000, 1),
    ];
    for (chrom, start, end, expected_len) in cases {
        let input = format!("{chrom}\t{start}\t{end}\n");
        let mut out = Vec::new();
        lengths(Cursor::new(input.as_str()), &mut out).unwrap();
        let s = String::from_utf8(out).unwrap();
        let last_col = s.trim_end().rsplit('\t').next().unwrap();
        let got: u64 = last_col.parse().unwrap();
        assert_eq!(got, expected_len, "len for {chrom}:{start}-{end}");
    }
}
