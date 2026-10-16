"""Compare the preserved ABI inventory with source, DLL exports, and C# bindings.

Usage: python tools/check_exports.py [path/to/urng.dll]
Run a normal all-features Cargo build first to generate RngNative.cs.
"""
import ctypes
import json
from pathlib import Path
import re
import sys

items = json.loads(Path("tools/cabi_inventory.json").read_text())
bindings = Path("target/release/RngNative.cs").read_text(encoding="utf8")
normalize = lambda text: re.sub(r"\s", "", text).rstrip(",")
for item in items:
    source = Path(item["file"]).read_text()
    match = re.search(r'pub (unsafe )?extern "C" fn ' + item["name"] + r'\s*\((.*?)\)\s*(->\s*\*mut \w+)?\s*\{', source, re.S)
    assert match, item["name"]
    assert normalize(match[2]) == normalize(item["args"]), item["name"]
    assert normalize(match[3] or "") == normalize(item["return"]), item["name"]
    assert bool(match[1]) == (not item["name"].endswith("_new")), item["name"]
    assert re.search(r"\b" + item["name"] + r"\(", bindings), item["name"]
if len(sys.argv) > 1:
    dll = ctypes.CDLL(str(Path(sys.argv[1]).resolve()))
    for item in items:
        assert hasattr(dll, item["name"]), item["name"]
print(f"All {len(items)} signatures, unsafe boundaries, C# bindings and requested DLL exports match.")
