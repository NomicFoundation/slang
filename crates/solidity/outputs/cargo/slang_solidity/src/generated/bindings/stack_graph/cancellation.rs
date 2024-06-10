// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::BitOr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use thiserror::Error;

use crate::bindings::{
    CancellationError as GraphCancellationError, CancellationFlag as GraphCancellationFlag,
};

/// Trait to signal that the execution is cancelled
pub trait CancellationFlag: Sync {
    fn check(&self, at: &'static str) -> Result<(), CancellationError>;
}

#[derive(Clone, Debug, Error)]
#[error("Cancelled at \"{0}\"")]
pub struct CancellationError(pub &'static str);

impl stack_graphs::CancellationFlag for &dyn CancellationFlag {
    fn check(&self, at: &'static str) -> Result<(), stack_graphs::CancellationError> {
        CancellationFlag::check(*self, at).map_err(|err| stack_graphs::CancellationError(err.0))
    }
}

impl GraphCancellationFlag for &dyn CancellationFlag {
    fn check(&self, at: &'static str) -> Result<(), GraphCancellationError> {
        CancellationFlag::check(*self, at).map_err(|err| GraphCancellationError(err.0))
    }
}

impl<'a> BitOr for &'a dyn CancellationFlag {
    type Output = OrCancellationFlag<'a>;
    fn bitor(self, rhs: Self) -> Self::Output {
        OrCancellationFlag(self, rhs)
    }
}

pub struct OrCancellationFlag<'a>(&'a dyn CancellationFlag, &'a dyn CancellationFlag);

impl CancellationFlag for OrCancellationFlag<'_> {
    fn check(&self, at: &'static str) -> Result<(), CancellationError> {
        self.0.check(at)?;
        self.1.check(at)?;
        Ok(())
    }
}

pub struct NoCancellation;

impl CancellationFlag for NoCancellation {
    fn check(&self, _at: &'static str) -> Result<(), CancellationError> {
        Ok(())
    }
}

pub struct CancelAfterDuration {
    start: Instant,
    limit: Duration,
}

impl CancelAfterDuration {
    #[allow(dead_code)]
    pub fn new(limit: Duration) -> Self {
        Self {
            start: Instant::now(),
            limit,
        }
    }

    #[allow(dead_code)]
    pub fn from_option(limit: Option<Duration>) -> Box<dyn CancellationFlag> {
        match limit {
            Some(limit) => Box::new(Self::new(limit)),
            None => Box::new(NoCancellation),
        }
    }
}

impl CancellationFlag for CancelAfterDuration {
    fn check(&self, at: &'static str) -> Result<(), CancellationError> {
        if self.start.elapsed().ge(&self.limit) {
            return Err(CancellationError(at));
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct AtomicCancellationFlag {
    flag: Arc<AtomicBool>,
}

impl AtomicCancellationFlag {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
        }
    }

    #[allow(dead_code)]
    pub fn cancel(&self) {
        self.flag.store(true, Ordering::Relaxed);
    }
}

impl CancellationFlag for AtomicCancellationFlag {
    fn check(&self, at: &'static str) -> Result<(), CancellationError> {
        if self.flag.load(Ordering::Relaxed) {
            return Err(CancellationError(at));
        }
        Ok(())
    }
}
