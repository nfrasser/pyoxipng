# Changelog

## 9.0.0

- Update to oxipng 9
- BREAKING: Removed `backup` option
- BREAKING: Removed `check` option
- BREAKING: Removed `pretend` option
- BREAKING: Removed `preserve_attrs` option
- BREAKING: Replaced `oxipng.Headers` with `oxipng.StripChunks`
- Added: `RawImage` class for optimizing raw RGBA data
- Added: `scale_16` option
- Fixed: correct `fast_evaluation` option implementation

## 8.0.1

- Python 3.12 wheels
- Drop Python 3.7 support

## 8.0.0

- Update to oxipng 8
- BREAKING: `interlace` option now expects `oxipng.Interlace` enum
- BREAKING: replace `alphas` option with `optimize_alpha` boolean
- Added: `check` option

## 7.0.0

- Upgrade to oxipng 7
- BREAKING: `filter` option now expects set of `oxipng.RowFilter` enum
- BREAKING: `deflate` option now expects instance of `oxipng.Deflaters`
- Added: `fast_evaluation` option

## 6.0.0

- Add missing alphas, strip and deflate options

## 5.0.0

- Sync version with oxipng major releases

## 0.2.0

- Update project metadata

## 0.1.0

- Initial release
