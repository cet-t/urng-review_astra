"""Build every combination, including Cargo's implicit thiserror feature."""
import csv
import hashlib
import itertools
import os
from pathlib import Path
import subprocess

features = ["rand", "cabi", "wide", "simd", "sampler", "seedgen", "thiserror"]
destination = Path("target/review/features")
destination.mkdir(parents=True, exist_ok=True)
env = dict(os.environ, DOCS_RS="1")
env.pop("RUSTFLAGS", None)
env.pop("CARGO_TARGET_DIR", None)
failed = []
def source_hash():
    digest = hashlib.sha256()
    paths = sorted(Path("src").rglob("*.rs")) + [Path("build.rs"), Path("Cargo.toml"), Path("README.md")]
    for path in paths:
        digest.update(path.as_posix().encode())
        digest.update(path.read_bytes())
    return digest.hexdigest()

initial_hash = source_hash()
(destination / "source.sha256").write_text(initial_hash + "\n")
with (destination / "results.csv").open("w", newline="", encoding="utf8") as report:
    writer = csv.writer(report)
    writer.writerow(["features", "exit_code"])
    for index, enabled in enumerate(itertools.product([False, True], repeat=len(features))):
        chosen = [f for f, yes in zip(features, enabled) if yes]
        command = ["cargo", "build", "--locked", "--offline", "--lib", "--no-default-features"]
        if chosen:
            command += ["--features", ",".join(chosen)]
        with (destination / f"{index:03}.log").open("wb") as log:
            result = subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT)
        writer.writerow([",".join(chosen) or "(none)", result.returncode])
        report.flush()
        if result.returncode:
            failed.append(chosen)
        if (index + 1) % 16 == 0:
            print(f"{index+1}/128 builds; {len(failed)} failures", flush=True)
print("Failed configurations:", failed, flush=True)
assert source_hash() == initial_hash, "Sources changed during validation; rerun the feature matrix"
raise SystemExit(bool(failed))
