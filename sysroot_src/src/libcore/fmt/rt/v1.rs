//! This is an internal module used by the ifmt! runtime. These structures are
//! emitted to static arrays to precompile format strings ahead of time.
//!
//! These definitions are similar to their `ct` equivalents, but differ in that
//! these can be statically allocated and are slightly optimized for the runtime
#![allow(missing_debug_implementations)]

pub struct Argument {
    pub position: Position,
    pub format: FormatSpec,
}

pub struct FormatSpec {
    pub fill: char,
    pub align: Alignment,
    pub flags: u32,
    pub precision: Count,
    pub width: Count,
}

/// Possible alignments that can be requested as part of a formatting directive.
pub enum Alignment {
    /// Indication that contents should be left-aligned.
    Left,
    /// Indication that contents should be right-aligned.
    Right,
    /// Indication that contents should be center-aligned.
    Center,
    /// No alignment was requested.
    Unknown,
}

pub enum Count {
    Is(usize),
    Param(usize),
    NextParam,
    Implied,
}

pub enum Position {
    Next,
    At(usize),
}
