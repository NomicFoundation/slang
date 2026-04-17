//! Simple wall-time benchmark comparing the two IR-producing pipelines,
//! plus a per-phase heap-allocation profile.
//!
//! Approach A (CST → IR):  parse source with `cst_consumer`, then run the
//!                         `CstToIrBuilder` to produce the IR.
//! Approach B (direct IR): parse source with `ir_consumer`, producing the IR
//!                         directly without allocating the intermediate CST.
//!
//! Usage:
//!   cargo run --release --example bench_cst_vs_ir -- [-n <iterations>] <file.sol> [<file.sol>...]
//!
//! Defaults: 50 iterations. Pragma versions in the input are ignored; parsing
//! uses the latest supported `LanguageVersion`.
//!
//! The allocation profile is produced by a global `CountingAllocator` that
//! tracks every heap alloc/dealloc. Each timed iteration therefore pays a
//! small (~few ns) accounting cost per allocation — proportional across
//! phases, so the relative comparison still holds, but the absolute numbers
//! are slightly inflated vs. an uninstrumented build.

use std::alloc::{GlobalAlloc, Layout, System};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::cst_consumer;
use slang_solidity_v2_ir::ir;

// Dev-deps of the `slang_solidity_v2_ir` crate that this example doesn't need:
use anyhow as _;
use slang_solidity_v2_parser as _;

// -----------------------------------------------------------------------------
// Counting global allocator
// -----------------------------------------------------------------------------

struct CountingAllocator;

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static DEALLOCS: AtomicU64 = AtomicU64::new(0);
static DEALLOC_BYTES: AtomicU64 = AtomicU64::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
            ALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DEALLOCS.fetch_add(1, Ordering::Relaxed);
        DEALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.dealloc(ptr, layout) };
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

#[derive(Clone, Copy)]
struct AllocSnapshot {
    allocs: u64,
    alloc_bytes: u64,
    deallocs: u64,
    dealloc_bytes: u64,
}

impl AllocSnapshot {
    fn take() -> Self {
        Self {
            allocs: ALLOCS.load(Ordering::Relaxed),
            alloc_bytes: ALLOC_BYTES.load(Ordering::Relaxed),
            deallocs: DEALLOCS.load(Ordering::Relaxed),
            dealloc_bytes: DEALLOC_BYTES.load(Ordering::Relaxed),
        }
    }
}

const LANG_VERSION: LanguageVersion = LanguageVersion::V0_8_34;
const DEFAULT_ITERATIONS: usize = 50;

struct Sample {
    source: String,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let (iterations, paths) = match parse_args(&args) {
        Ok(parsed) => parsed,
        Err(msg) => {
            eprintln!("error: {msg}");
            eprintln!("usage: bench_cst_vs_ir [-n <iterations>] <file.sol> [<file.sol>...]");
            return ExitCode::from(2);
        }
    };

    let samples: Vec<Sample> = paths
        .into_iter()
        .map(|path| {
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
            Sample { source }
        })
        .collect();

    let total_bytes: usize = samples.iter().map(|s| s.source.len()).sum();
    println!(
        "Running {iterations} iterations over {} file(s), {total_bytes} bytes total",
        samples.len()
    );

    // Each phase is warmed up with its own work just before being measured.
    // This primes caches / branch predictors / allocator free-lists with the
    // instruction and data footprint of the phase under test, rather than
    // sharing a single upfront warm-up across all three phases.
    let warmup_iters = warmup_iterations(iterations);

    // Phase A: parse to CST (no IR build).
    let cst_parse_phase = |iters: usize| {
        time_loop(iters, || {
            for sample in &samples {
                let cst = cst_consumer::parse(&sample.source, LANG_VERSION).unwrap();
                std::hint::black_box(cst);
            }
        })
    };
    let _ = cst_parse_phase(warmup_iters);
    let cst_parse_total = cst_parse_phase(iterations);

    // Phase B: build IR from a pre-existing CST.
    // We re-parse each iteration so the input CST is freshly allocated (to
    // avoid the Rc ref-count fast-paths that would skew a repeated build over
    // the same tree). This attributes both parse and build cost to this
    // measurement, so we subtract `cst_parse_total` below to isolate B.
    let cst_to_ir_phase = |iters: usize| {
        time_loop(iters, || {
            for sample in &samples {
                let cst = cst_consumer::parse(&sample.source, LANG_VERSION).unwrap();
                let ir = ir::build(&cst, &sample.source);
                std::hint::black_box(ir);
            }
        })
    };
    let _ = cst_to_ir_phase(warmup_iters);
    let cst_parse_and_build_total = cst_to_ir_phase(iterations);

    // Phase C: direct IR parse.
    let direct_ir_phase = |iters: usize| {
        time_loop(iters, || {
            for sample in &samples {
                let ir = ir::ir_consumer::parse(&sample.source, LANG_VERSION).unwrap();
                std::hint::black_box(ir);
            }
        })
    };
    let _ = direct_ir_phase(warmup_iters);
    let direct_total = direct_ir_phase(iterations);

    let ir_build_only = cst_parse_and_build_total.saturating_sub(cst_parse_total);

    let cst_parse_per = cst_parse_total / iterations as u32;
    let pipeline_per = cst_parse_and_build_total / iterations as u32;
    let build_only_per = ir_build_only / iterations as u32;
    let direct_per = direct_total / iterations as u32;

    println!();
    println!("CST parse only (cst_consumer):");
    println!(
        "  total: {cst_parse_total:?}  per iter: {cst_parse_per:?}"
    );
    println!("CST → IR pipeline (cst_consumer + ir::build):");
    println!(
        "  total: {cst_parse_and_build_total:?}  per iter: {pipeline_per:?}"
    );
    println!("  ir::build alone (pipeline − cst-parse):");
    println!(
        "  total: {ir_build_only:?}  per iter: {build_only_per:?}"
    );
    println!("Direct IR pipeline (ir_consumer):");
    println!("  total: {direct_total:?}  per iter: {direct_per:?}");

    let pipeline_secs = cst_parse_and_build_total.as_secs_f64();
    let direct_secs = direct_total.as_secs_f64();
    println!();
    if direct_secs < pipeline_secs {
        println!("Direct IR is {:.2}x faster", pipeline_secs / direct_secs);
    } else {
        println!("Direct IR is {:.2}x slower", direct_secs / pipeline_secs);
    }
    println!(
        "CST-parse share of pipeline: {:.1}%",
        100.0 * cst_parse_total.as_secs_f64() / pipeline_secs
    );
    println!(
        "Direct-IR vs CST-parse-only: {:.2}x",
        direct_secs / cst_parse_total.as_secs_f64()
    );

    // Per-phase allocation profile. Each phase runs one iteration over all
    // samples, sandwiched between allocator snapshots. We measure three
    // moments:
    //   `before`:       baseline
    //   `after_phase`:  directly after the phase closure returns, with the
    //                   produced result(s) still alive
    //   `after_drop`:   after dropping the result(s)
    //
    // From these we derive:
    //   during-phase allocs/bytes = after_phase - before
    //   during-phase deallocs     = after_phase - before (temporaries freed
    //                               inside the phase, e.g. the CST inside the
    //                               CST → IR pipeline)
    //   net-retained alive at return = allocs - deallocs in that window
    //   freed on drop             = after_drop - after_phase
    println!();
    println!("Allocation profile (single iteration over all samples):");
    profile_phase("CST parse only (cst_consumer)", || {
        samples
            .iter()
            .map(|s| cst_consumer::parse(&s.source, LANG_VERSION).unwrap())
            .collect::<Vec<_>>()
    });
    profile_phase("CST → IR pipeline (cst_consumer + ir::build)", || {
        samples
            .iter()
            .map(|s| {
                let cst = cst_consumer::parse(&s.source, LANG_VERSION).unwrap();
                ir::build(&cst, &s.source)
                // `cst` is dropped here, inside the phase
            })
            .collect::<Vec<_>>()
    });
    profile_phase("Direct IR pipeline (ir_consumer)", || {
        samples
            .iter()
            .map(|s| ir::ir_consumer::parse(&s.source, LANG_VERSION).unwrap())
            .collect::<Vec<_>>()
    });

    ExitCode::SUCCESS
}

/// Run `phase` once, measuring allocations during the call and the size of
/// the retained return value (determined by how many deallocations happen
/// when the result is dropped).
fn profile_phase<F, R>(name: &str, phase: F)
where
    F: FnOnce() -> R,
{
    let before = AllocSnapshot::take();
    let result = phase();
    let after_phase = AllocSnapshot::take();
    drop(result);
    let after_drop = AllocSnapshot::take();

    let during_allocs = after_phase.allocs - before.allocs;
    let during_alloc_bytes = after_phase.alloc_bytes - before.alloc_bytes;
    let during_deallocs = after_phase.deallocs - before.deallocs;
    let during_dealloc_bytes = after_phase.dealloc_bytes - before.dealloc_bytes;

    let retained_objects = during_allocs - during_deallocs;
    let retained_bytes = during_alloc_bytes - during_dealloc_bytes;

    let drop_deallocs = after_drop.deallocs - after_phase.deallocs;
    let drop_bytes = after_drop.dealloc_bytes - after_phase.dealloc_bytes;

    println!("  {name}:");
    println!(
        "    during phase: {} allocs ({} KiB)  /  {} deallocs ({} KiB)",
        fmt_count(during_allocs),
        fmt_kib(during_alloc_bytes),
        fmt_count(during_deallocs),
        fmt_kib(during_dealloc_bytes),
    );
    println!(
        "    retained at return: {} objects ({} KiB)",
        fmt_count(retained_objects),
        fmt_kib(retained_bytes),
    );
    println!(
        "    freed on drop:      {} deallocs ({} KiB)",
        fmt_count(drop_deallocs),
        fmt_kib(drop_bytes),
    );
}

fn fmt_count(n: u64) -> String {
    let mut s = n.to_string();
    let mut i = s.len() as isize - 3;
    while i > 0 {
        s.insert(i as usize, ',');
        i -= 3;
    }
    s
}

fn fmt_kib(bytes: u64) -> String {
    fmt_count(bytes / 1024)
}

fn time_loop<F: FnMut()>(iterations: usize, mut body: F) -> Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        body();
    }
    start.elapsed()
}

/// Warm-up budget for a phase: ~10% of the measured run, clamped to [1, 10].
fn warmup_iterations(iterations: usize) -> usize {
    (iterations / 10).clamp(1, 10)
}

fn parse_args(args: &[String]) -> Result<(usize, Vec<PathBuf>), String> {
    let mut iterations = DEFAULT_ITERATIONS;
    let mut paths = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-n" | "--iterations" => {
                let value = args
                    .get(i + 1)
                    .ok_or_else(|| format!("missing value for {}", args[i]))?;
                iterations = value
                    .parse()
                    .map_err(|e| format!("invalid iteration count {value:?}: {e}"))?;
                if iterations == 0 {
                    return Err("iteration count must be > 0".to_owned());
                }
                i += 2;
            }
            path => {
                paths.push(PathBuf::from(path));
                i += 1;
            }
        }
    }
    if paths.is_empty() {
        return Err("no input files provided".to_owned());
    }
    Ok((iterations, paths))
}
