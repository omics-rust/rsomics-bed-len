use rsomics_common::{Result, RsomicsError};
use std::io::{BufRead, BufWriter, Write};

/// Append `end - start` as a new tab-separated column to every BED record.
/// Header lines (starting with `#`) and blank lines are passed through unchanged.
pub fn lengths<R: BufRead, W: Write>(reader: R, output: W) -> Result<u64> {
    let mut out = BufWriter::with_capacity(64 * 1024, output);
    let mut count: u64 = 0;
    for line in reader.lines() {
        let line = line.map_err(RsomicsError::Io)?;
        if line.starts_with('#') || line.is_empty() {
            writeln!(out, "{line}").map_err(RsomicsError::Io)?;
            continue;
        }
        let mut fields = line.splitn(4, '\t');
        let chrom = fields.next().unwrap_or("");
        let start_str = fields.next().unwrap_or("");
        let end_str = fields.next().unwrap_or("");
        let rest = fields.next();

        let start: u64 = start_str
            .parse()
            .map_err(|e| RsomicsError::InvalidInput(format!("start: {e}")))?;
        let end: u64 = end_str
            .parse()
            .map_err(|e| RsomicsError::InvalidInput(format!("end: {e}")))?;
        let len = end.saturating_sub(start);

        if let Some(r) = rest {
            writeln!(out, "{chrom}\t{start_str}\t{end_str}\t{r}\t{len}")
                .map_err(RsomicsError::Io)?;
        } else {
            writeln!(out, "{chrom}\t{start_str}\t{end_str}\t{len}").map_err(RsomicsError::Io)?;
        }
        count += 1;
    }
    out.flush().map_err(RsomicsError::Io)?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn basic() {
        let input = "chr1\t100\t200\nchr2\t0\t50\tfeat\n";
        let mut out = Vec::new();
        lengths(Cursor::new(input), &mut out).unwrap();
        let s = String::from_utf8(out).unwrap();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[0], "chr1\t100\t200\t100");
        assert_eq!(lines[1], "chr2\t0\t50\tfeat\t50");
    }

    #[test]
    fn skip_header() {
        let input = "# header\nchr1\t0\t100\n";
        let mut out = Vec::new();
        lengths(Cursor::new(input), &mut out).unwrap();
        let s = String::from_utf8(out).unwrap();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[0], "# header");
        assert_eq!(lines[1], "chr1\t0\t100\t100");
    }
}
