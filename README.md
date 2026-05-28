# rsomics-bed-len

Append interval length (`end - start`) as a new column to BED records.

## Usage

```sh
rsomics-bed-len [OPTIONS] [INPUT]
rsomics-bed-len intervals.bed
cat intervals.bed | rsomics-bed-len
```

## Examples

```sh
# Input:  chr1  100  200  gene1
# Output: chr1  100  200  gene1  100

rsomics-bed-len input.bed
```

## Origin

Independent Rust implementation. Equivalent to:

```awk
awk 'BEGIN{OFS="\t"} !/^#/ && NF>=3 {print $0, $3-$2}' input.bed
```

License: MIT OR Apache-2.0.
