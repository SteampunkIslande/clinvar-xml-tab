# Install

```bash
git clone https://github.com/SteampunkIslande/clinvar-xml-tab
# OR
git clone git@github.com:SteampunkIslande/clinvar-xml-tab.git
```

```bash
cargo build --release .
```

# Usage

## Convert XML clinvar file to TSV

```
Convert XML clinvar files to tab separated values

Usage: clinvar-xml-tab [OPTIONS]

Options:
  -i, --input <INPUT>    Input XML file
  -o, --output <OUTPUT>  Output TSV file
      --hg19             Human genome build 19 (incompatible with hg38)
      --hg38             Human genome build 38 (incompatible with hg19)
  -h, --help             Print help
  -V, --version          Print version
```

If no input is specified, it will read from stdin and detect whether it is using compression.

If no output is specified and you do not redirect stdout, it will do nothing. And if you redirect, it will write to stdout uncompressed.

It will automatically detect input compression and desired output format from the file name extension.

## After the conversion is done

What you want is a VCF file, right?

So here is a sample duckdb python code to sort the result and write it back to minimal (invalid) VCF records (with only the base header).

```python
import duckdb as db

db.sql("""COPY ( SELECT * FROM read_csv('clinvar.unsorted.vcf.gz',names=['#CHROM','POS','ID','REF','ALT','QUAL','FILTER','INFO'],header=False) ORDER BY "#CHROM" ASC, POS ASC) TO 'clinvar.vcf.gz' """)
```

Then it's up to you to use the resulting file to annotate your variants, for instance using snpSift annotate.

# Why clinvar-xml-tab ?

I needed a VCF file with the RCV accession numbers as well as the clinical significance for each variant.

Only VCF I found with the required fields were in the UCSC FTP, however the RCV numbers made no sense when I tried annotating my files with this VCF.

Clinvar, however, provides the right RCV accession numbers but in the XML file format. Thus, the need for an XML parser to get a clean VCF.

Saw [this project](https://github.com/SeqOne/clinvcf), but unfortunately it doesn't output the RCV field. And since I know nothing about the nim language and wanted to try my luck with Rust, here I am.

# Notes, Miscellaneous

## Flexing

On my machine, in release mode, it took:

```
194,91s user 0,42s system 99% cpu 3:15,35 total
```

## Contribution

All contributions are welcome!

Please feel free to contribute, if you lack a field don't hesitate to create a PR or an issue.
See `clinvar_set.xml` as an example of a ClinvarSet (which is the minimal repeated unit within the input XML, that I deserialize using quick-xml).
