"""
Python wrapper for multithreaded .png image file optimizer oxipng
(https://github.com/shssoichiro/oxipng - written in Rust). Use this module to
reduce the file size of your PNG images.
"""

from typing import Optional, Union, Sequence
from enum import Enum
from os import PathLike

StrOrBytesPath = Union[str, bytes, PathLike]

class PngError(Exception):
    """
    Raised by optimize functions when an error is encountered while optimizing PNG files
    """

    ...

class RowFilter(Enum):
    """
    enum entries for filter option
    """

    NoOp = ...
    Sub = ...
    Up = ...
    Average = ...
    Paeth = ...
    MinSum = ...
    Entropy = ...
    Bigrams = ...
    BigEnt = ...
    Brute = ...

class Interlacing(Enum):
    """
    enum entries for interlace option
    """

    Off = ...
    Adam7 = ...

class StripChunks:
    """
    Initialization class for strip option
    """

    @staticmethod
    def none() -> "StripChunks": ...
    @staticmethod
    def strip(val: Sequence[bytes]) -> "StripChunks": ...
    @staticmethod
    def safe() -> "StripChunks": ...
    @staticmethod
    def keep(val: Sequence[bytes]) -> "StripChunks": ...
    @staticmethod
    def all() -> "StripChunks": ...

class Deflaters:
    """
    Initialization class for deflate option
    """

    @staticmethod
    def libdeflater(compression: int) -> "Deflaters": ...
    @staticmethod
    def zopfli(iterations: int) -> "Deflaters": ...

class Zopfli:
    """
    Initialize a Zopfli deflate configuration option value
    """

    def __init__(self, iterations: int) -> None: ...

class Libdeflater:
    """
    Initialize a Libdeflater deflate configuration option value
    """

    def __init__(self) -> None: ...

class ColorType:
    """
    Initialization class for RawImage color_type option
    """

    @staticmethod
    def grayscale(transparent_shade: Optional[int] = None) -> "ColorType":
        """
        Grayscale, with one color channel.
        """
        ...

    @staticmethod
    def rgb(transparent_color: Optional[Sequence[int]] = None) -> "ColorType":
        """
        RGB, with three color channels. Specify optional color value that should
        be rendered as transparent.
        """
        ...

    @staticmethod
    def indexed(palette: Sequence[Sequence[int]]) -> "ColorType":
        """
        Indexed, with one byte per pixel representing a color from the palette.
        Specify palette containing the colors used, up to 256 entries
        """
        ...

    @staticmethod
    def grayscale_alpha() -> "ColorType":
        """
        Grayscale + Alpha, with two color channels.
        """
        ...

    @staticmethod
    def rgba() -> "ColorType":
        """
        RGBA, with four color channels.
        """
        ...

class RawImage:
    """
    Create an optimized PNG file from raw image data
    """

    def __init__(
        self,
        data: Union[bytes, bytearray],
        width: int,
        height: int,
        *,
        color_type: ColorType = ...,
        bit_depth: int = 8,
    ) -> None: ...

    def add_png_chunk(self, name: bytes, data: Union[bytes, bytearray]) -> None:
        """
        Add a png chunk, such as `b"iTXt"`, to be included in the output
        """
        ...

    def add_icc_profile(self, data: bytes) -> None:
        """
        Add an ICC profile for the image
        """
        ...

    def create_optimized_png(
        self,
        *,
        level: int = 2,
        fix_errors: bool = False,
        force: bool = False,
        filter: Sequence[RowFilter] = [RowFilter.NoOp],
        interlace: Optional[Interlacing] = None,
        optimize_alpha: bool = False,
        bit_depth_reduction: bool = True,
        color_type_reduction: bool = True,
        palette_reduction: bool = True,
        grayscale_reduction: bool = True,
        idat_recoding: bool = True,
        scale_16: bool = False,
        strip: StripChunks = StripChunks.none(),
        deflate: Deflaters = Deflaters.libdeflater(11),
        fast_evaluation: bool = False,
        timeout: Optional[int] = None,
    ) -> bytes:
        """
        Create an optimized png from the raw image data. Full option
        descriptions at https://github.com/nfrasser/pyoxipng#options
        """
        ...

def optimize(
    input: StrOrBytesPath,
    output: Optional[StrOrBytesPath] = ...,
    *,
    level: int = 2,
    fix_errors: bool = False,
    force: bool = False,
    filter: Sequence[RowFilter] = [RowFilter.NoOp],
    interlace: Optional[Interlacing] = None,
    optimize_alpha: bool = False,
    bit_depth_reduction: bool = True,
    color_type_reduction: bool = True,
    palette_reduction: bool = True,
    grayscale_reduction: bool = True,
    idat_recoding: bool = True,
    scale_16: bool = False,
    strip: StripChunks = StripChunks.none(),
    deflate: Deflaters = Deflaters.libdeflater(11),
    fast_evaluation: bool = False,
    timeout: Optional[int] = None,
) -> None:
    """
    Optimize a file on disk. Full option descriptions at
    https://github.com/nfrasser/pyoxipng#options
    """
    ...

def optimize_from_memory(
    data: bytes,
    *,
    level: int = 2,
    fix_errors: bool = False,
    force: bool = False,
    filter: Sequence[RowFilter] = [RowFilter.NoOp],
    interlace: Optional[Interlacing] = None,
    optimize_alpha: bool = False,
    bit_depth_reduction: bool = True,
    color_type_reduction: bool = True,
    palette_reduction: bool = True,
    grayscale_reduction: bool = True,
    idat_recoding: bool = True,
    scale_16: bool = False,
    strip: StripChunks = StripChunks.none(),
    deflate: Deflaters = Deflaters.libdeflater(11),
    fast_evaluation: bool = False,
    timeout: Optional[int] = None,
) -> bytes:
    """
    Optimize raw data from a PNG file loaded in Python as a bytes object. Full
    option descriptions at https://github.com/nfrasser/pyoxipng#options
    """
    ...
