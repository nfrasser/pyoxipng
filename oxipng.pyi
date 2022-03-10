"""
Python wrapper for multithreaded .png image file optimizer oxipng
(https://github.com/shssoichiro/oxipng - written in Rust). Use this module to
reduce the file size of your PNG images.
"""
from typing import Optional, Set, Union
from pathlib import Path

StrOrBytesPath = Union[str, bytes, Path]

class PngError(Exception): ...

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
    bit_depth_reduction: bool = True,
    color_type_reduction: bool = True,
    palette_reduction: bool = True,
    grayscale_reduction: bool = True,
    idat_recoding: bool = True,
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
    bit_depth_reduction: bool = True,
    color_type_reduction: bool = True,
    palette_reduction: bool = True,
    grayscale_reduction: bool = True,
    idat_recoding: bool = True,
    use_heuristics: bool = False,
    timeout: Optional[int] = None,
) -> bytes: ...
