# homebrew-datacell

Homebrew tap for [datacell](https://github.com/yingkitw/datacell) - A CLI tool for spreadsheet manipulation with pandas-style operations.

## Installation

```bash
brew tap yingkitw/datacell
brew install datacell
```

## Usage

```bash
# Read a CSV file
datacell read --input data.csv

# Convert between formats
datacell convert --input data.csv --output data.xlsx

# Sort data
datacell sort --input data.csv --column 2

# Filter data
datacell filter --input data.csv --column age --operator ">=" --value 18

# Get help
datacell --help
```

## Documentation

Full documentation available at: https://github.com/yingkitw/datacell

## License

Apache-2.0
