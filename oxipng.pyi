"""
Python wrapper for multithreaded .png image file optimizer oxipng
(https://github.com/shssoichiro/oxipng - written in Rust). Use this module to
reduce the file size of your PNG images.
"""
from collections.abc import Collection
from typing import List, Optional, Union
from enum import Enum
from os import PathLike

StrOrBytesPath = Union[str, bytes, PathLike]


class PngError(Exception):
    ...


class RowFilter(Enum):
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
    Off = ...
    Adam7 = ...


class StripChunks:
    @staticmethod
    def none() -> "StripChunks":
        ...

    @staticmethod
    def strip(val: Collection[str]) -> "StripChunks":
        ...

    @staticmethod
    def safe() -> "StripChunks":
        ...

    @staticmethod
    def keep(val: Collection[str]) -> "StripChunks":
        ...

    @staticmethod
    def all() -> "StripChunks":
        ...


class Deflaters:
    @staticmethod
    def libdeflater(compression: int) -> "Deflaters":
        ...

    @staticmethod
    def zopfli(iterations: int) -> "Deflaters":
        ...


class Zopfli:
    """
    Initialize a Zopfli deflate configuration option value
    """

    def __init__(self, iterations: int) -> None:
        ...


class Libdeflater:
    """
    Initialize a Libdeflater deflate configuration option value
    """

    def __init__(self) -> None:
        ...


def optimize(
    input: StrOrBytesPath,
    output: Optional[StrOrBytesPath] = ...,
    level: int = 2,
    fix_errors: bool = False,
    force: bool = False,
    filter: Collection[RowFilter] = {RowFilter.NoOp},
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
    use_heuristics: bool = False,
    timeout: Optional[int] = None,
) -> None:
    ...


def optimize_from_memory(
    data: bytes,
    level: int = 2,
    fix_errors: bool = False,
    force: bool = False,
    filter: Collection[RowFilter] = {RowFilter.NoOp},
    interlace: Optional[Interlacing] = None,
    optimize_alpha: bool = False,
    bit_depth_reduction: bool = True,
    color_type_reduction: bool = True,
    palette_reduction: bool = True,
    grayscale_reduction: bool = True,
    idat_recoding: bool = True,
    strip: StripChunks = StripChunks.none(),
    deflate: Deflaters = Deflaters.libdeflater(11),
    use_heuristics: bool = False,
    timeout: Optional[int] = None,
) -> bytes:
    ...
