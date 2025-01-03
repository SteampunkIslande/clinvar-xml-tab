# Install

## Download

```bash
git clone https://github.com/SteampunkIslande/clinvar-xml-tab
# OR
git clone git@github.com:SteampunkIslande/clinvar-xml-tab.git
```

## Install

```bash
cargo install --path .
```

## Add auto-completion for your shell

If you're using [oh-my-zsh](https://ohmyz.sh), you're in luck! Just run:

```bash
clinvar-xml-tab autocomplete --shell zsh
```
and it will (over)write autocompletions for your shell into `.oh-my-zsh/completions/_clinvar-xml-tab_ZSH`

For other shells, running `clinvar-xml-tab autocomplete --shell {bash, elvish, fish, powershell}` will write autcompletions to stdout.
If you only need it for the current session, you might as well just source it (the entire stdout is a valid completion script).

# Usage

## Convert XML clinvar file to TSV

```
Convert XML clinvar files to tab separated values

Usage: clinvar-xml-tab [OPTIONS] <COMMAND>

Commands:
  convert       Convert XML Clinvar to VCF
  debug         Only print out the very first XML element of input
  autocomplete  Generate Autocompletion
  help          Print this message or the help of the given subcommand(s)

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

For now, what you're left with is an unsorted VCF file with a far-from-perfect header.
All that remains is a `bcftools sort` and voilà! You have yourself the latest clinvar vcf file from the official XML release.

# Why clinvar-xml-tab ?

I needed a VCF file with the RCV accession numbers as well as the clinical significance for each variant.

Only VCF I found with the required fields were in the UCSC FTP, however the RCV numbers made no sense when I tried annotating my files with this VCF.

Clinvar, however, provides the right RCV accession numbers but in the XML file format. Thus, the need for an XML parser to get a clean VCF.

Saw [this project](https://github.com/SeqOne/clinvcf), but unfortunately it doesn't output the RCV field. And since I know nothing about the nim language and wanted to try my luck with Rust, here I am.

# Notes, Miscellaneous

## Flexing

On my machine, in release mode, it took:

```
435,84s user 0,91s system 99% cpu 7:16,77 total
```

## Roadmap

For now, these are the only fields that get outputted by this program:

- CLNACC
- CLNSIG

These are the ones I'm most interested in. But feel free to open an issue or a pull request if you think it's missing a field.

## Contribution

All contributions are welcome!

Please feel free to contribute, if you lack a field don't hesitate to create a PR or an issue.
See `clinvar_set.xml` as an example of a ClinvarSet (which is the minimal repeated unit within the input XML, that I deserialize using quick-xml).

