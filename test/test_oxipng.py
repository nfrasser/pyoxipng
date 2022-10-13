from pathlib import Path
from shutil import copy2
import oxipng
import pytest


@pytest.fixture
def infile(tmpdir):
    filename = "shipitsquirrel.png"
    orig_path = Path() / "test" / filename
    input_path = Path(tmpdir) / filename
    copy2(orig_path, input_path)
    return input_path


@pytest.fixture
def outfile(tmpdir):
    return Path(tmpdir) / "shipitsquirrel-optimized.png"


@pytest.fixture
def bakfile(tmpdir):
    return Path(tmpdir) / "shipitsquirrel.bak.png"


@pytest.fixture
def indata():
    with open(Path() / "test" / "shipitsquirrel.png", "rb") as f:
        return f.read()


def test_init(outfile: Path):
    assert not outfile.exists()


def test_optimize(infile, outfile):
    oxipng.optimize(infile, outfile)
    assert outfile.exists()
    assert infile.stat().st_size > outfile.stat().st_size


def test_optimize_level(infile, outfile):
    oxipng.optimize(infile, outfile, level=6)
    assert outfile.exists()
    assert infile.stat().st_size > outfile.stat().st_size


def test_optimize_pretend(infile, outfile):
    oxipng.optimize(infile, outfile, pretend=True)
    assert not outfile.exists()


def test_optimize_opts(infile, bakfile):
    oxipng.optimize(
        infile,
        backup=True,
        fix_errors=True,
        force=True,
        preserve_attrs=True,
        filter={1, 2, 3},
        interlace=1,
        alphas={oxipng.AlphaOptim.White},
        bit_depth_reduction=False,
        palette_reduction=False,
        grayscale_reduction=False,
        idat_recoding=False,
        strip=oxipng.Headers.strip(["foo", "bar"]),
        deflate=oxipng.Libdeflater(),
        use_heuristics=False,
        timeout=100,
    )
    assert bakfile.exists()
    assert infile.stat().st_size != bakfile.stat().st_size


def test_optimize_inplace(infile):
    orig_size = infile.stat().st_size
    oxipng.optimize(infile)
    assert infile.stat().st_size < orig_size


def test_optimize_from_memory(indata):
    assert len(oxipng.optimize_from_memory(indata)) < len(indata)


def test_raises_pngerror():
    with pytest.raises(oxipng.PngError):
        oxipng.optimize_from_memory(b"Hello World!")


def test_raises_typeerror(indata):
    with pytest.raises(TypeError):
        oxipng.optimize_from_memory(indata, filter={1: 2})  # type: ignore


def test_deflate_zlib():
    assert oxipng.Zlib()
    assert oxipng.Zlib([7, 8], [2, 3])
    assert oxipng.Zlib(compression=[7, 8], strategies=[2, 3], window=8)

    with pytest.raises(OverflowError):
        oxipng.Zlib([1000])


def test_deflate_zopfli():
    assert oxipng.Zopfli(1)
    assert oxipng.Zopfli(42)
    assert oxipng.Zopfli(255)

    with pytest.raises(TypeError):
        oxipng.Zopfli(0)

    with pytest.raises(OverflowError):
        oxipng.Zopfli(256)
