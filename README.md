# pyoxipng

[![CI](https://github.com/nfrasser/pyoxipng/actions/workflows/CI.yml/badge.svg)](https://github.com/nfrasser/pyoxipng/actions/workflows/CI.yml)
[![PyPI](https://badgen.net/pypi/v/pyoxipng)](https://pypi.org/project/pyoxipng/)

Python wrapper for multithreaded .png image file optimizer
[oxipng](https://github.com/shssoichiro/oxipng) (written in Rust). Use
`pyoxipng` to reduce the file size of your PNG images.

Jump to a section

- [Installation](#installation)
- [API](#api)

  - [optimize](#oxipngoptimizeinput-outputnone-kwargs)
  - [optimize_from_memory](#oxipngoptimize_from_memorydata-kwargs)
  - [RawImage](#oxipngrawimage)

- [Options](#options)
  - [filter](#filter)
  - [interlace](#interlace)
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

## API

### oxipng.optimize(input, output=None, \*\*kwargs)

Optimize a file on disk.

**Parameters**:

- **input** _(str | bytes | PathLike)_ – path to input file to optimize
- **output** _(str | bytes | PathLike, optional)_ – path to optimized output result file. If not specified, overwrites input. Defaults to None
- **\*\*kwargs** – [Options](#options)

**Returns**

- None

**Raises**

- **oxipng.PngError** – optimization could not be completed

**Examples:**

Optimize a file on disk and overwrite

```py
oxipng.optimize("/path/to/image.png")
```

Optimize a file and save to a new location:

```py
oxipng.optimize("/path/to/image.png", "/path/to/image-optimized.png")
```

### oxipng.optimize_from_memory(data, \*\*kwargs)

Optimize raw data from a PNG file loaded in Python as a `bytes` object:

**Parameters**:

- **data** _(bytes)_ – raw PNG data to optimize
- **\*\*kwargs** – [Options](#options)

**Returns**

- _(bytes)_ – optimized raw PNG data

**Raises**

- **oxipng.PngError** – optimization could not be completed

**Examples:**

```py
data = ...  # bytes of png data
optimized_data = oxipng.optimize_from_memory(data)
with open("/path/to/image-optimized.png", "wb") as f:
    f.write(optimized_data)
```

### oxipng.RawImage

Create an optimized PNG file from raw image data:

```python
raw = oxipng.RawImage(data, width, height)
optimized_data = raw.create_optimized_png()
```

By default, assumes the input data is 8-bit, row-major RGBA, where every 4 bytes represents one pixel with Red-Green-Blue-Alpha channels. To interpret non-RGBA data, specify a `color_type` parameter with the `oxipng.ColorType` class:

| Method                                                       | Description                                                                                                                            |
| ------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------- |
| `oxipng.ColorType.grayscale(int \| None)`                    | Grayscale, with one color channel. Specify optional shade of gray that should be rendered as transparent.                              |
| `oxipng.ColorType.rgb(tuple[int, int, int])`                 | RGB, with three color channels. Specify optional color value that should be rendered as transparent.                                   |
| `oxipng.ColorType.indexed(list[[tuple[int, int, int, int]])` | Indexed, with one byte per pixel representing a color from the palette. Specify palette containing the colors used, up to 256 entries. |
| `oxipng.ColorType.grayscale_alpha()`                         | Grayscale + Alpha, with two color channels.                                                                                            |
| `oxipng.ColorType.rgba()`                                    | RGBA, with four color channels.                                                                                                        |

**Parameters:**

- **data** _(bytes | bytearray)_ – Raw image data bytes. Format depends on `color_type` and `bit_depth` parameters
- **width** _(int)_ – Width of raw image, in pixels
- **height** _(int)_ – Height of raw image, in pixels
- **color_type** _([oxipng.ColorType, optional)_ – Descriptor for color type used to represent this image. Optional, defaults to `oxipng.ColorType.rgba()`
- **bit_depth** _(int, optional)_ – Bit depth of raw image. Optional, defaults to 8

**Examples:**

Save RGB image data from a JPEG file, interpreting black pixels as transparent.

```python
from PIL import Image
import numpy as np

# Load an image file with Pillow
jpg = Image.open("/path/to/image.jpg")

# Convert to RGB numpy array
rgb_array = np.array(jpg.convert("RGB"), dtype=np.uint8)
height, width, channels = rgb_array.shape

# Create raw image with sRGB color profile
data = rgb_array.tobytes()
color_type = oxipng.ColorType.rgb((0, 0, 0))  # black is transparent
raw = oxipng.RawImage(data, width, height, color_type=color_type)
raw.add_png_chunk(b"sRGB", b"\0")

# Optimize and save
optimized = raw.create_optimized_png(level=6)
with open("/path/to/image/optimized.png", "wb") as f:
    f.write(optimized)
```

Save with data where bytes reference a color palette

```python
data = b"\0\1\2..."  # get index data
palette = [[0, 0, 0, 255], [1, 23, 234, 255], ...]
color_type = oxipng.ColorType.indexed(palette)
raw = oxipng.RawImage(data, 100, 100, color_type=color_type)
optimized = raw.create_optimized_png()
```

**Methods:**

#### add_png_chunk(name, data)

Add a png chunk, such as `b"iTXt"`, to be included in the output

**Parameters:**

- **name** _(bytes)_ – PNG chunk identifier
- **data** _(bytes | bytarray)_

**Returns:**

- None

#### add_icc_profile(data)

Add an ICC profile for the image

**Parameters:**

- **data** _(bytes)_ – ICC profile data

**Returns:**

- None

#### create_optimized_png(\*\*kwargs)

Create an optimized png from the raw image data using the options provided

**Parameters:**

- **\*\*kwargs** – [Options](#options)

**Returns:**

- _(bytes)_ optimized PNG image data

## Options

`optimize` , `optimize_from_memory` and `RawImage.create_optimized_png` accept the following options as keyword arguments.

**Example:**

```py
oxipng.optimize("/path/to/image.png", level=6, fix_errors=True, interlace=oxipng.Interlacing.Adam7)
```

| Option                 | Description                                                                                                                       | Type                              | Default                   |
| ---------------------- | --------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- | ------------------------- |
| `level`                | Set the optimization level to an integer between 0 and 6 (inclusive)                                                              | int                               | `2`                       |
| `fix_errors`           | Attempt to fix errors when decoding the input file rather than throwing `PngError`                                                | bool                              | `False`                   |
| `force`                | Write to output even if there was no improvement in compression                                                                   | bool                              | `False`                   |
| `filter`               | Which filters to try on the file. Use Use enum values from `oxipng.RowFilter`                                                     | Sequence[[RowFilter](#filter)]    | `[RowFilter.NoOp]`        |
| `interlace`            | Whether to change the interlacing type of the file. `None` will not change current interlacing type                               | [Interlacing](#interlace) \| None | `None`                    |
| `optimize_alpha`       | Whether to allow transparent pixels to be altered to improve compression                                                          | bool                              | `False`                   |
| `bit_depth_reduction`  | Whether to attempt bit depth reduction                                                                                            | bool                              | `True`                    |
| `color_type_reduction` | Whether to attempt color type reduction                                                                                           | bool                              | `True`                    |
| `palette_reduction`    | Whether to attempt palette reduction                                                                                              | bool                              | `True`                    |
| `grayscale_reduction`  | Whether to attempt grayscale reduction                                                                                            | bool                              | `True`                    |
| `idat_recoding`        | If any type of reduction is performed, IDAT recoding will be performed regardless of this setting                                 | bool                              | `True`                    |
| `scale_16`             | Whether to forcibly reduce 16-bit to 8-bit by scaling                                                                             | bool                              | `False`                   |
| `strip`                | Which headers to strip from the PNG file, if any. Specify with `oxipng.StripChunks`                                               | [StripChunks](#strip)             | `StripChunks.none()`      |
| `deflate`              | Which DEFLATE algorithm to use. Specify with `oxipng.Deflaters`                                                                   | [Deflaters](#deflate)             | `Deflaters.libdeflater()` |
| `fast_evaluation`      | Whether to use fast evaluation to pick the best filter                                                                            | bool                              | `False`                   |
| `timeout`              | Maximum amount of time to spend (in seconds) on optimizations. Further potential optimizations skipped if the timeout is exceeded | float \| None                     | `None`                    |

### filter

Initialize a `filter` list or tuple with any of the following `oxipng.RowFilter` enum options:

- `oxipng.RowFilter.NoOp`
- `oxipng.RowFilter.Sub`
- `oxipng.RowFilter.Up`
- `oxipng.RowFilter.Average`
- `oxipng.RowFilter.Paeth`
- `oxipng.RowFilter.Bigrams`
- `oxipng.RowFilter.BigEnt`
- `oxipng.RowFilter.Brute`

### interlace

Set `interlace` to `None` to keep existing interlacing or to one of following `oxipng.Interlacing` enum options:

- `oxipng.Interlacing.Off` (interlace disabled)
- `oxipng.Interlacing.Adam7` (interlace enabled)

### strip

Initialize the `strip` option with one of the following static methods in the
`oxipng.StripChunks` class.

| Method                                      | Description                                                                                 |
| ------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `oxipng.StripChunks.none()`                 | None                                                                                        |
| `oxipng.StripChunks.strip(Sequence[bytes])` | Strip chunks specified in the given list                                                    |
| `oxipng.StripChunks.safe()`                 | Strip chunks that won't affect rendering (all but cICP, iCCP, sRGB, pHYs, acTL, fcTL, fdAT) |
| `oxipng.StripChunks.keep(Sequence[bytes])`  | Strip all non-critical chunks except those in the given list                                |
| `oxipng.StripChunks.all()`                  | Strip all non-critical chunks                                                               |

### deflate

Initialize the `deflate` option with one of the following static methods in the
`oxipng.Deflaters` class.

| Method                              | Description                                                |
| ----------------------------------- | ---------------------------------------------------------- |
| `oxipng.Deflaters.libdeflater(int)` | Libdeflater with compression level [0-12]                  |
| `oxipng.Deflaters.zopfli(int)`      | Zopfli with number of compression iterations to do [1-255] |

## Development

1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Install [Python 3.8+](https://www.python.org/downloads/)
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
   ruff check .
   ruff format .
   ```

## License

MIT
