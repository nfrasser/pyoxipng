# pyoxipng

[![CI](https://github.com/nfrasser/pyoxipng/actions/workflows/CI.yml/badge.svg)](https://github.com/nfrasser/pyoxipng/actions/workflows/CI.yml)
[![PyPI](https://badgen.net/pypi/v/pyoxipng)](https://pypi.org/project/pyoxipng/)

Python wrapper for multithreaded .png image file optimizer
[oxipng](https://github.com/shssoichiro/oxipng) (written in Rust). Use
`pyoxipng` to reduce the file size of your PNG images.

Jump to a section

- [Installation](#installation)
- [Usage](#usage)
- [Options](#options)
  - [alphas](#alphas)
  - [strip](#strip)
  - [deflate](#deflate)
- [Development](#development)
- [License](#license)

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

Both the `optimize` and `optimize_from_memory` functions accept the following
options as keyword arguments.

Example usage:

```py
oxipng.optimize("/path/to/image.png", level=6, backup=True, interlace=1)
```

| Option                 | Description                                                                                                                                | Type                   | Default                   |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------- | ------------------------- |
| `level`                | Set the optimization level to an integer between 0 and 6 (inclusive)                                                                       | `int`                  | `2`                       |
| `backup`               | Whether the input file should be backed up before writing the output                                                                       | `bool`                 | `False`                   |
| `fix_errors`           | Attempt to fix errors when decoding the input file rather than throwing `PngError`                                                         | `bool`                 | `False`                   |
| `check`                | Don't actually run any optimizations, just parse the PNG file                                                                              | `bool`                 | `False`                   |
| `pretend`              | Don't actually write any output file, just calculate the best results                                                                      | `bool`                 | `False`                   |
| `force`                | Write to output even if there was no improvement in compression                                                                            | `bool`                 | `False`                   |
| `preserve_attrs`       | Ensure the output file has the same permissions as the input file                                                                          | `bool`                 | `False`                   |
| `filter`               | Which filters to try on the file. Use Use enum values from `oxipng.RowFilter`                                                              | `set=[RowFilter.NoOp]` | `{RowFilter.NoOp}`        |
| `interlace`            | Whether to change the interlacing type of the file. `None` will not change current interlacing type                                        | `Interlacing \| None`  | `None`                    |
| `optimize_alpha`       | Whether to allow transparent pixels to be altered to improve compression                                                                   | `bool`                 | `False`                   |
| `bit_depth_reduction`  | Whether to attempt bit depth reduction                                                                                                     | `bool`                 | `True`                    |
| `color_type_reduction` | Whether to attempt color type reduction                                                                                                    | `bool`                 | `True`                    |
| `palette_reduction`    | Whether to attempt palette reduction                                                                                                       | `bool`                 | `True`                    |
| `grayscale_reduction`  | Whether to attempt grayscale reduction                                                                                                     | `bool`                 | `True`                    |
| `idat_recoding`        | If any type of reduction is performed, IDAT recoding will be performed regardless of this setting                                          | `bool`                 | `True`                    |
| `strip`                | Which headers to strip from the PNG file, if any. Specify with `oxipng.Headers`                                                            | `Headers`              | `Headers.none()`          |
| `deflate`              | Which DEFLATE algorithm to use. Specify with `oxipng.Deflaters`                                                                            | `Deflaters`            | `Deflaters.libdeflater()` |
| `use_heuristics`       | Whether to use heuristics to pick the best filter and compression. Intended for use with `level=1`                                         | `bool`                 | `False`                   |
| `timeout`              | Maximum amount of time to spend (in milliseconds) on optimizations. Further potential optimizations are skipped if the timeout is exceeded | `int \| None`          | `None`                    |

### filter

Initialize the `filter` set with any of the following enum options:

- `oxipng.RowFilter.NoOp`
- `oxipng.RowFilter.Sub`
- `oxipng.RowFilter.Up`
- `oxipng.RowFilter.Average`
- `oxipng.RowFilter.Paeth`
- `oxipng.RowFilter.Bigrams`
- `oxipng.RowFilter.BigEnt`
- `oxipng.RowFilter.Brute`

### interlace

Set `interlace` to `None` to keep existing interlacing or to one of following
enum options:

- `oxipng.Interlacing.Off` (interlace disabled)
- `oxipng.Interlacing.Adam7` (interlace enabled)

### strip

Initialize the `strip` option with one of the following static methods in the
`oxipng.Headers` class.

| Method                            | Description                                                                                        |
| --------------------------------- | -------------------------------------------------------------------------------------------------- |
| `oxipng.Headers.none()`           | None                                                                                               |
| `oxipng.Headers.strip(list[str])` | Remove specific chunks                                                                             |
| `oxipng.Headers.safe()`           | Headers that won't affect rendering (all but cHRM, gAMA, iCCP, sBIT, sRGB, bKGD, hIST, pHYs, sPLT) |
| `oxipng.Headers.keep(set[str])`   | Remove all non-critical chunks except these                                                        |
| `oxipng.Headers.all()`            | All non-critical headers                                                                           |

### deflate

Initialize the `deflate` option with one of the following static methods in the
`oxipng.Deflaters` class.

| Method                              | Description                                                 |
| ----------------------------------- | ----------------------------------------------------------- |
| `oxipng.Deflaters.libdeflater(int)` | Libdeflater with compression level [0-12]                   |
| `oxipng.Deflaters.zopfli(int)`      | Zopfli with number of compression iterations to do [1-255]. |

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
