"""Run the identical compiled harnesses in alternating order, without builds."""
import csv
import hashlib
import io
import json
import os
from pathlib import Path
import statistics
import subprocess

destination = Path("validation/benchmark")
destination.mkdir(parents=True, exist_ok=True)
env = dict(os.environ, RAYON_NUM_THREADS="4")
metadata = {
    "baseline_commit": "19bc224",
    "source_sha256": hashlib.sha256(Path("tools/review_bench.rs").read_bytes()).hexdigest(),
    "rustc": subprocess.check_output(["rustc", "-Vv"], text=True),
    "library_command": "DOCS_RS=1 CARGO_TARGET_DIR=target/{before,after} cargo build --locked --offline --release --all-features",
    "harness_command": "rustc --edition=2024 -C opt-level=3 -C target-cpu=native -C panic=abort tools/review_bench.rs --extern urng=target/{before,after}/release/liburng.rlib -L dependency=target/{before,after}/release/deps -o target/review/{before,after}_bench.exe",
    "rayon_threads": 4, "words_per_run": 1048576, "warmups": 1, "timed_runs_per_process": 9,
    "process_pairs": 3, "order": [["before", "after"], ["after", "before"], ["before", "after"]],
    "cpu_affinity_pinned": False, "cpu_frequency_controlled": False,
}
(destination / "metadata.json").write_text(json.dumps(metadata, indent=2), encoding="utf8")
results = {"before": {}, "after": {}}
for index, order in enumerate(metadata["order"], 1):
    for version in order:
        output = subprocess.check_output([f"target/review/{version}_bench.exe"], text=True, env=env)
        (destination / f"{version}-{index}.csv").write_text(output, encoding="utf8")
        for row in csv.DictReader(io.StringIO(output)):
            results[version].setdefault(row["name"], []).append(float(row["median_ns_per_word"]))
with (destination / "comparison.csv").open("w", newline="", encoding="utf8") as report:
    writer = csv.writer(report)
    writer.writerow(["name", "before_median_ns", "after_median_ns", "after_over_before"])
    for name in results["before"]:
        before = statistics.median(results["before"][name])
        after = statistics.median(results["after"][name])
        writer.writerow([name, before, after, after / before])
        print(f"{name}: {before:.4f} -> {after:.4f} ns/word ({after/before:.2f}x)", flush=True)
