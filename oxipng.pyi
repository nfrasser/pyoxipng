"""
Python wrapper for multithreaded .png image file optimizer oxipng
(https://github.com/shssoichiro/oxipng - written in Rust). Use this module to
reduce the file size of your PNG images.
"""
from typing import List, Optional, Set, Union
from enum import Enum
from pathlib import Path

StrOrBytesPath = Union[str, bytes, Path]

class PngError(Exception): ...

class AlphaOptim(Enum):
    NoOp = ...
    Black = ...
    White = ...
    Up = ...
    Right = ...
    Down = ...
    Left = ...

class Headers:
    @staticmethod
    def none() -> "Headers": ...
    @staticmethod
    def strip(val: List[str]) -> "Headers": ...
    @staticmethod
    def safe() -> "Headers": ...
    @staticmethod
    def keep(val: List[str]) -> "Headers": ...
    @staticmethod
    def all() -> "Headers": ...

class Zlib:
    """
    Initialize a Zlib deflate configuration option value
    """

    def __init__(
        self,
        compression: List[int] = [9],
        strategies: List[int] = [0, 1, 2, 3],
        window: int = 15,
    ) -> None: ...

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

def optimize(
    input: StrOrBytesPath,
    output: Optional[StrOrBytesPath] = None,
    level: int = 2,
    backup: bool = False,
    fix_errors: bool = False,
    pretend: bool = False,
    force: bool = False,
    preserve_attrs: bool = False,
    filter: Set[int] = {0, 5},
    interlace: Optional[int] = None,
    alphas: Set[AlphaOptim] = {AlphaOptim.NoOp},
    bit_depth_reduction: bool = True,
    color_type_reduction: bool = True,
    palette_reduction: bool = True,
    grayscale_reduction: bool = True,
    idat_recoding: bool = True,
    strip: Headers = Headers.none(),
    deflate: Union[Zlib, Zopfli, Libdeflater] = Zlib(),
    use_heuristics: bool = False,
    timeout: Optional[int] = None,
) -> None: ...
def optimize_from_memory(
    data: bytes,
    level: int = 2,
    backup: bool = False,
    fix_errors: bool = False,
    pretend: bool = False,
    force: bool = False,
    preserve_attrs: bool = False,
    filter: Set[int] = {0, 5},
    interlace: Optional[int] = None,
    alphas: Set[AlphaOptim] = {AlphaOptim.NoOp},
    bit_depth_reduction: bool = True,
    color_type_reduction: bool = True,
    palette_reduction: bool = True,
    grayscale_reduction: bool = True,
    idat_recoding: bool = True,
    strip: Headers = Headers.none(),
    deflate: Union[Zlib, Zopfli, Libdeflater] = Zlib(),
    use_heuristics: bool = False,
    timeout: Optional[int] = None,
) -> bytes: ...
