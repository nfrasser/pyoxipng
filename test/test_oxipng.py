from pathlib import Path
from shutil import copy2
import gzip
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


@pytest.fixture(scope="session")
def indata():
    with open(Path() / "test" / "shipitsquirrel.png", "rb") as f:
        return f.read()


@pytest.fixture(scope="session")
def rawdata():
    with gzip.GzipFile(Path() / "test" / "python-logo-only.raw.gz") as f:
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


def test_optimize_opts(infile):
    initial_size = infile.stat().st_size
    oxipng.optimize(
        infile,
        fix_errors=True,
        force=True,
        # NOTE: set args deprecated in v9.1, will change to sequence
        filter={oxipng.RowFilter.Sub, oxipng.RowFilter.Up, oxipng.RowFilter.Average},  # type: ignore
        interlace=oxipng.Interlacing.Adam7,
        optimize_alpha=True,
        bit_depth_reduction=False,
        palette_reduction=False,
        grayscale_reduction=False,
        idat_recoding=False,
        scale_16=True,
        strip=oxipng.StripChunks.strip([b"cICP", b"sRGB"]),
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
    assert oxipng.StripChunks.strip([b"sRGB"])
    assert oxipng.StripChunks.safe()
    assert oxipng.StripChunks.keep([b"sRGB", b"pHYs"])

    with pytest.raises(TypeError):
        assert oxipng.StripChunks.strip(["sRGB", 42])  # type: ignore

    with pytest.raises(TypeError):
        assert oxipng.StripChunks.keep([b"RGB"])

    with pytest.raises(TypeError):
        assert oxipng.StripChunks.keep([b"RGB123"])


def test_deflate_zopfli():
    assert oxipng.Deflaters.zopfli(1)
    assert oxipng.Deflaters.zopfli(42)
    assert oxipng.Deflaters.zopfli(255)

    with pytest.raises(TypeError):
        oxipng.Deflaters.zopfli(0)

    with pytest.raises(OverflowError):
        oxipng.Deflaters.zopfli(256)


def test_raw_image(rawdata):
    raw = oxipng.RawImage(rawdata, 269, 326)
    raw.add_png_chunk(b"sRBG", b"\0")
    raw.add_icc_profile(b"Color LCD")
    optimized = raw.create_optimized_png(
        level=2,
        fix_errors=True,
        interlace=oxipng.Interlacing.Adam7,
    )
    assert len(optimized) < len(rawdata)


def test_raw_image_rgb():
    raw = oxipng.RawImage(
        b"\1\2\3\4\5\6\7\6\5\1\2\3",
        2,
        2,
        color_type=oxipng.ColorType.rgb((4, 5, 6)),
        bit_depth=8,
    )
    assert raw.create_optimized_png()


def test_color_type():
    assert oxipng.ColorType.grayscale()
    assert oxipng.ColorType.grayscale(42)

    with pytest.raises(OverflowError):
        oxipng.ColorType.grayscale(1_000_000)

    assert oxipng.ColorType.rgb()
    assert oxipng.ColorType.rgb((0, 0, 0))
    assert oxipng.ColorType.rgb([65535, 65535, 65535])

    with pytest.raises(OverflowError):
        assert oxipng.ColorType.rgb((65535, 65536, 65535))

    assert oxipng.ColorType.indexed([[1, 2, 3, 4]])
    assert oxipng.ColorType.indexed([(i, i, i, 255) for i in range(256)])

    with pytest.raises(OverflowError):
        assert oxipng.ColorType.indexed([(255, 255, 255, 256)])

    with pytest.raises(ValueError):
        assert oxipng.ColorType.indexed([(255, 255, 255)])

    assert oxipng.ColorType.grayscale_alpha()
    assert oxipng.ColorType.rgba()
