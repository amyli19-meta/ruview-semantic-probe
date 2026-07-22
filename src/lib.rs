//! RuView HA-MIND semantic-inference layer (ADR-115 §3.12).
//!
//! Consumes a stream of fused sensor `Reading`s (one per second) from the
//! sensing-server and runs each semantic primitive's finite-state machine,
//! emitting a `SemanticEvent` whenever a primitive's active state changes.
//!
//! This is the reference (oracle) implementation.

use std::collections::VecDeque;

/// A room zone. Some zones carry semantic tags (bed / bathroom).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Bedroom,
    Bathroom,
    LivingRoom,
    Hallway,
}

impl Zone {
    /// Zones tagged as sleeping areas (bed-tagged).
    pub fn is_bed(self) -> bool {
        matches!(self, Zone::Bedroom)
    }
}

/// One fused sensor sample. `t_s` is seconds since sensing-server start.
#[derive(Debug, Clone, Copy)]
pub struct Reading {
    pub t_s: u64,
    pub presence: bool,
    pub motion_pct: f64,
    pub breathing_bpm: f64,
    pub heart_rate_bpm: f64,
    pub hrv_ms: f64,
    pub person_count: u32,
    pub zone: Zone,
    pub recent_fall: bool,
}

/// The semantic primitives implemented in this probe (subset of ADR-115 v1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    SomeoneSleeping,
    PossibleDistress,
    RoomActive,
    NoMovementSafety,
    BathroomOccupied,
}

impl Primitive {
    fn index(self) -> usize {
        match self {
            Primitive::SomeoneSleeping => 0,
            Primitive::PossibleDistress => 1,
            Primitive::RoomActive => 2,
            Primitive::NoMovementSafety => 3,
            Primitive::BathroomOccupied => 4,
        }
    }
    fn from_index(i: usize) -> Self {
        [
            Primitive::SomeoneSleeping,
            Primitive::PossibleDistress,
            Primitive::RoomActive,
            Primitive::NoMovementSafety,
            Primitive::BathroomOccupied,
        ][i]
    }
}

/// Emitted when a primitive's active state changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticEvent {
    pub primitive: Primitive,
    pub active: bool,
    pub t_s: u64,
}

/// Tracks how long a boolean condition has held continuously (seconds).
#[derive(Default, Clone, Copy)]
struct Sustained {
    since: Option<u64>,
}
impl Sustained {
    /// Returns the number of seconds the condition has held continuously
    /// (0 on the tick it first becomes true; resets to 0 when false).
    fn update(&mut self, cond: bool, t: u64) -> u64 {
        if cond {
            let s = *self.since.get_or_insert(t);
            t.saturating_sub(s)
        } else {
            self.since = None;
            0
        }
    }
}

/// Warmup suppression window: no primitive fires in the first 60 s (ADR §3.12.4).
const WARMUP_S: u64 = 60;

/// The semantic-inference engine. Feed it readings in time order.
#[allow(dead_code)]
pub struct SemanticEngine {
    active: [bool; 5],
    sleep_cond: Sustained,
    sleep_exit: Sustained,
    room_move: Sustained,
    room_idle: Sustained,
    nomove: Sustained,
    distress_cond: Sustained,
    distress_latch_until: Option<u64>,
    hr_hist: VecDeque<(u64, f64)>,
}

impl Default for SemanticEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticEngine {
    pub fn new() -> Self {
        Self {
            active: [false; 5],
            sleep_cond: Sustained::default(),
            sleep_exit: Sustained::default(),
            room_move: Sustained::default(),
            room_idle: Sustained::default(),
            nomove: Sustained::default(),
            distress_cond: Sustained::default(),
            distress_latch_until: None,
            hr_hist: VecDeque::new(),
        }
    }

    pub fn is_active(&self, p: Primitive) -> bool {
        self.active[p.index()]
    }

    /// Push one reading; returns the state-change events emitted at this tick.
    pub fn push(&mut self, r: Reading) -> Vec<SemanticEvent> {
        // TODO: implement the ADR-115 semantic-inference FSMs.
        let _ = r;
        Vec::new()
    }
}
