#!/usr/bin/env python3
"""Three-way comparison of local gungraun results (with `frames(["*::setup"])`)
against the latest main and teofr reports on Bencher, for slang_v2.

Inputs:
  - /workspace/slang/report.txt — captured stdout from local `cargo bench`
  - Bencher main + teofr reports fetched on the fly.

Output: /workspace/slang/migration/local_comparison.md
"""

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

PROJECT = "slang-dashboard-cargo-slang-v2"

# Mapping from the human-readable label in report.txt to the bencher measure slug.
LABEL_TO_SLUG = {
    "Instructions":          "instructions",
    "L1 Hits":               "l1-hits",
    "L2 Hits":               "l2-hits",          # only present in pre-0.16 (we don't expect this in local)
    "LL Hits":               "ll-hits",
    "RAM Hits":              "ram-hits",
    "Total read+write":      "total-read-write",
    "Estimated Cycles":      "estimated-cycles",
    "Total bytes":           "total-bytes",
    "Total blocks":          "total-blocks",
    "At t-gmax bytes":       "at-t-gmax-bytes",
    "At t-gmax blocks":      "at-t-gmax-blocks",
    "At t-end bytes":        "at-t-end-bytes",
    "At t-end blocks":       "at-t-end-blocks",
    "Reads bytes":           "reads-bytes",
    "Writes bytes":          "writes-bytes",
}

# Pair benchmarks in slang_v2 by (test, phase).
PHASES = ["ir_builder", "parser", "semantic"]

def mkey(name):
    """Main-side key from slang_v2 main benchmark name."""
    m = re.match(r"slang_v2::(\w+)_full_v2::(\w+) test:", name)
    if not m:
        return None
    test, rest = m.group(1), m.group(2)
    for p in PHASES:
        if rest == f"{test}_{p}":
            return (test, p)
    return None

def tkey(name):
    """Teofr-side key from slang_v2 teofr-style benchmark name."""
    m = re.match(r"slang_v2::pipeline::(\w+) (\w+):", name)
    return (m.group(2), m.group(1)) if m else None


def parse_local_report(path: Path) -> dict:
    """Returns {(test, phase): {measure_slug: float}}."""
    out = {}
    cur_key = None
    cur_metrics = None
    # Stop at the "Gungraun result:" line — the file repeats results after that.
    text = path.read_text()
    stop = text.find("\nGungraun result:")
    if stop > 0:
        text = text[:stop]
    for line in text.splitlines():
        # New benchmark header line.
        m = re.match(r"^slang_v2::pipeline::(\w+) (\w+):", line)
        if m:
            cur_key = (m.group(2), m.group(1))
            cur_metrics = {}
            out[cur_key] = cur_metrics
            continue
        # Metric line: "  Label: ... value |N/A ..."
        m2 = re.match(r"^\s+([A-Za-z][A-Za-z0-9 +\-]+?):\s+([\d,]+)\|", line)
        if m2 and cur_metrics is not None:
            label = m2.group(1).strip()
            slug = LABEL_TO_SLUG.get(label)
            if slug is None:
                continue
            value = float(m2.group(2).replace(",", ""))
            cur_metrics[slug] = value
    return out


def fetch_latest_report(project: str, branch: str):
    out = subprocess.run(
        ["bencher", "report", "list", project, "--branch", branch,
         "--per-page", "1", "--sort", "date_time", "--direction", "desc"],
        capture_output=True, check=True, text=True,
    )
    data = json.loads(out.stdout)
    return data[0] if data else None


def report_to_map(report, key_fn):
    """Return {(test, phase): {measure_slug: value}} for benchmarks the pairing recognises."""
    out = {}
    for inner in report["results"]:
        for entry in inner:
            name = entry["benchmark"]["name"]
            k = key_fn(name)
            if k is None:
                continue
            measures = {m["measure"]["slug"]: m["metric"]["value"]
                        for m in entry["measures"]}
            out[k] = measures
    return out


# Categorisation reused from verify_measurements.py.
EXACT = ["instructions", "estimated-cycles", "l1-hits", "ram-hits", "total-read-write"]
RENAMED = ("l2-hits", "ll-hits")  # pre-0.16 / post-0.16 names for the same metric
SEMANTIC = ["total-bytes", "total-blocks",
            "at-t-gmax-bytes", "at-t-gmax-blocks",
            "at-t-end-bytes", "at-t-end-blocks",
            "reads-bytes", "writes-bytes"]


def fmt(v):
    if v is None:
        return "—"
    if isinstance(v, float) and v.is_integer():
        v = int(v)
    return f"{v:,}" if isinstance(v, int) else f"{v:.2f}"


def pct(a, b):
    if a is None or b is None or b == 0:
        return "—"
    return f"{(a - b) / b * 100:+.2f}%"


def build_report(local_path: Path, local_label: str) -> str:
    local = parse_local_report(local_path)
    print(f"{local_label} benchmarks parsed: {len(local)}", file=sys.stderr)

    main_report = fetch_latest_report(PROJECT, "main")
    teofr_report = fetch_latest_report(PROJECT, "teofr/update-iai-callgrind")
    if main_report is None or teofr_report is None:
        return "missing main or teofr report"
    main_map = report_to_map(main_report, mkey)
    teofr_map = report_to_map(teofr_report, tkey)
    print(f"main benchmarks: {len(main_map)}  teofr benchmarks: {len(teofr_map)}",
          file=sys.stderr)

    keys = sorted(set(local) & set(main_map) & set(teofr_map))
    print(f"matched all three: {len(keys)}", file=sys.stderr)

    lines = []
    lines.append(f"# {local_label} (frames-config) vs Bencher main vs Bencher teofr — slang-v2")
    lines.append("")
    lines.append(f"`{local_label}` ran with the new `Dhat::default().frames([...solidity_testing_perf_cargo::tests::*::setup, ...solidity_testing_perf_cargo::tests::setup::setup])` config.")
    lines.append("`MAIN` is the latest main report on Bencher (pre-0.16, unfiltered DHAT).")
    lines.append("`TEOFR` is the latest teofr/update-iai-callgrind report on Bencher (gungraun 0.18.1, default bench-fn entry point).")
    lines.append("")
    lines.append(f"Matched benchmarks: **{len(keys)}** across {local_label.lower()} + main + teofr.")
    lines.append("")
    lines.append("Δ percentages are computed against MAIN — i.e. how far each side")
    lines.append("is from the pre-0.16 baseline.")
    lines.append("")

    for k in keys:
        l = local[k]
        m = main_map[k]
        t = teofr_map[k]
        lines.append(f"## {k}")
        lines.append("")

        lines.append("### Should match across all three")
        lines.append("")
        lines.append(f"| measure | {local_label} | MAIN | TEOFR | {local_label} Δ vs MAIN | TEOFR Δ vs MAIN |")
        lines.append("|---|---:|---:|---:|---:|---:|")
        for measure in EXACT:
            lv = l.get(measure); mv = m.get(measure); tv = t.get(measure)
            lines.append(f"| `{measure}` | {fmt(lv)} | {fmt(mv)} | {fmt(tv)} | "
                         f"{pct(lv, mv)} | {pct(tv, mv)} |")
        # l2-hits → ll-hits: main reports l2-hits, teofr/local report ll-hits.
        m_l2 = m.get("l2-hits")
        l_ll = l.get("ll-hits")
        t_ll = t.get("ll-hits")
        lines.append(f"| `l2-hits → ll-hits` | {fmt(l_ll)} | {fmt(m_l2)} | {fmt(t_ll)} | "
                     f"{pct(l_ll, m_l2)} | {pct(t_ll, m_l2)} |")
        lines.append("")

        lines.append("### Semantics changed (DHAT)")
        lines.append("")
        lines.append(f"| measure | {local_label} | MAIN | TEOFR | {local_label} Δ vs MAIN | TEOFR Δ vs MAIN |")
        lines.append("|---|---:|---:|---:|---:|---:|")
        for measure in SEMANTIC:
            lv = l.get(measure); mv = m.get(measure); tv = t.get(measure)
            lines.append(f"| `{measure}` | {fmt(lv)} | {fmt(mv)} | {fmt(tv)} | "
                         f"{pct(lv, mv)} | {pct(tv, mv)} |")
        lines.append("")

    return "\n".join(lines)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--input", type=Path, default=Path("/workspace/slang/report.txt"),
                    help="Path to gungraun cargo bench stdout capture")
    ap.add_argument("--label", default="LOCAL",
                    help="Label for the input side in the report (e.g. LOCAL, CI)")
    ap.add_argument("--out", type=Path,
                    default=Path("/workspace/slang/migration/local_comparison.md"),
                    help="Output markdown path")
    args = ap.parse_args()
    md = build_report(args.input, args.label)
    args.out.write_text(md)
    print(f"wrote {args.out} ({len(md):,} bytes)")


if __name__ == "__main__":
    main()
