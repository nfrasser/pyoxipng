pyoxipng
===

[![CI](https://github.com/nfrasser/pyoxipng/actions/workflows/CI.yml/badge.svg)](https://github.com/nfrasser/pyoxipng/actions/workflows/CI.yml)
[![PyPI](https://badgen.net/pypi/v/pyoxipng)](https://pypi.org/project/pyoxipng/)

Python wrapper for multithreaded .png image file optimizer
[oxipng](https://github.com/shssoichiro/oxipng) (written in Rust). Use
`pyoxipng` to reduce the file size of your PNG images.

## Installation

Install from PyPI:

```sh
pip install pyoxipng
```

Import in your Python code:

```py
import oxipng
```

## Usage

To optimize an existing file on disk:
```py
oxipng.optimize("/path/to/image.png", "/path/to/image-optimized.png")
```
Second argument is optional; if not provided, will overwrite the existing file.

To optimize a file already-loaded in Python as a `bytes` object:
```py
data = ...  # bytes of png data

optimized_data = oxipng.optimize_from_memory(data)

# Save the result
with open("/path/to/image-optimized.png", "wb") as f:
    f.write(optimized_data)
```

Both functions raise `oxipng.PngError` if the optimization cannot be completed.

## Options

Both the `optimize` and `optimize_from_memory` functions take the following
options as keyword arguments.

Example usage:

```py
oxipng.optimize("/path/to/image.png", level=6, backup=True, interlace=1)
```

Option | Description | Type | Default
-|-|-|-
`level` | Set the optimization level to an integer between 0 and 6 (inclusive) | `int` | `2`
`backup` | Whether the input file should be backed up before writing the output  | `bool` | `False`
`fix_errors` | Attempt to fix errors when decoding the input file rather than throwing `PngError` | `bool` | `False`
`pretend` | Don't actually write any output file, just calculate the best results | `bool` | `False`
`force` | Write to output even if there was no improvement in compression | `bool` | `False`
`preserve_attrs` | Ensure the output file has the same permissions as the input file | `bool` | `False`
`filter` | Which filters to try on the file [0-5] | `set[int]` | `{0,5}`
`interlace` | Whether to change the interlacing type of the file. `0` means disable interlacing. `1` means enable it. `None` means leave as is. | `int \| None` | `None`
`bit_depth_reduction` | Whether to attempt bit depth reduction | `bool` | `True`
`color_type_reduction` |  Whether to attempt color type reduction | `bool` | `True`
`palette_reduction` | Whether to attempt palette reduction | `bool` | `True`
`grayscale_reduction` | Whether to attempt grayscale reduction | `bool` | `True`
`idat_recoding` | If any type of reduction is performed, IDAT recoding will be performed regardless of this setting | `bool` | `True`
`use_heuristics` | Whether to use heuristics to pick the best filter and compression. Intended for use with `level=1` | `bool` | `False`
`timeout` | Maximum amount of time to spend (in milliseconds) on optimizations. Further potential optimizations are skipped if the timeout is exceeded. | `int \| None` | `None`

## Development

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Install [Python 3.7+](https://www.python.org/downloads/)
1. Install [Pipenv](https://pipenv.pypa.io/en/latest/)
1. Clone this repository and navigate to it via command line
   ```sh
   git clone https://github.com/nfrasser/pyoxipng.git
   cd pyoxipng
   ```
1. Install dependencies
   ```sh
   pipenv install --dev
   ```
1. Activate the dev environment
   ```
   pipenv shell
   ```
1. Build
   ```sh
   maturin develop
   ```
1. Run tests
   ```
   pytest
   ```
1. Format code
   ```
   black .
   ```

## License

MIT
