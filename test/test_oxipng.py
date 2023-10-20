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


def test_optimize_pretend(infile: Path, outfile):
    initial_stat = infile.stat()
    oxipng.optimize(infile, None)
    new_stat = infile.stat()

    assert initial_stat.st_mtime == new_stat.st_mtime
    assert initial_stat.st_size == new_stat.st_size


def test_optimize_opts(infile):
    initial_size = infile.stat().st_size
    oxipng.optimize(
        infile,
        fix_errors=True,
        force=True,
        filter={oxipng.RowFilter.Sub, oxipng.RowFilter.Up, oxipng.RowFilter.Average},
        interlace=oxipng.Interlacing.Adam7,
        optimize_alpha=True,
        bit_depth_reduction=False,
        palette_reduction=False,
        grayscale_reduction=False,
        idat_recoding=False,
        scale_16=True,
        strip=oxipng.StripChunks.strip(["cICP", "sRGB"]),
        deflate=oxipng.Deflaters.libdeflater(12),
        timeout=100,
    )
    assert infile.stat().st_size != initial_size


def test_optimize_inplace(infile: Path):
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


def test_strip_chunks():
    assert oxipng.StripChunks.none()
    assert oxipng.StripChunks.strip(["sRGB"])
    assert oxipng.StripChunks.safe()
    assert oxipng.StripChunks.keep(["sRGB", "pHYs"])

    with pytest.raises(TypeError):
        assert oxipng.StripChunks.strip(["sRGB", 42])  # type: ignore

    with pytest.raises(ValueError):
        assert oxipng.StripChunks.keep(["RGB"])

    with pytest.raises(ValueError):
        assert oxipng.StripChunks.keep(["RGB123"])


def test_deflate_zopfli():
    assert oxipng.Deflaters.zopfli(1)
    assert oxipng.Deflaters.zopfli(42)
    assert oxipng.Deflaters.zopfli(255)

    with pytest.raises(TypeError):
        oxipng.Deflaters.zopfli(0)

    with pytest.raises(OverflowError):
        oxipng.Deflaters.zopfli(256)
