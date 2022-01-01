# Bulk Scan In with Rust
Implement Alma scan in API with Rust

## Program flags
- k --apikey default_value = "NOSUCHKEY"
> You must provide APIKEY for the program to run
- f --filepath default_value = "./barcodes.txt"
> The text file holds barcodes to process
- l --library default_value = "MAIN"
> The library in which scan in is performed
- c --circdesk default_value = "DEFAULT_CIRC_DESK"
> The circulation desk where scan in happened
- b --barcode default_value = "NOSUCHCODE"
> Optional when you scan in only one item
- i --inhouseuse default_value = "false")]
> Optional if register scanned item for in house use

## Example of testing in dev mode:
`$ cargo run -- -k YOURKEY -f ./barcodes.txt`
## Example of executing the program in Windows:
`$ scanin.exe -k YOURKEY -f ./barcodes.txt`

## Notice
* Put barcodes in *barcode.txt* one barcode per line.
* Put *barcode.txt* with *scanin.exe* in the same folder.
* You can `cargo build` executable for your own OS

## Version
* 0.1.0 first commit
* 0.1.1 add flags