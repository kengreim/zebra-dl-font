```
Usage: zebra-dl-font.exe [OPTIONS] <INPUT> <NAME>

Arguments:
  <INPUT>  Relative path to font to upload
  <NAME>   Desired name of font on Zebra printer. .TTF will be appended if not included

Options:
  -p, --printer <PRINTER>  Name of Zebra or other ZPL-capable printer to upload font to. If omitted, the first printer that contains ZPL in its name will be used
  -h, --help               Print help
```

# Example
```
./zebra-dl-font.exe --input .\GrapeNuts-Regular.ttf --name HAND.TTF
```
