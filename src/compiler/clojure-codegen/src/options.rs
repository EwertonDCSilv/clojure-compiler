//! Backend option types: Cranelift optimization level, the optional
//! compiler-owned IR pipeline selection, and the not-yet-admitted IR
//! experiment bundle. [`CodegenOptions`] groups them into the single
//! configuration [`crate::compile_object_with_options`] accepts.

use std::str::FromStr;

/// Cranelift optimization level for generated functions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptimizationLevel {
    /// Disable optimization passes beyond required legalization.
    None,
    /// Optimize for execution speed.
    Speed,
    /// Balance execution speed and generated code size.
    SpeedAndSize,
}

impl OptimizationLevel {
    pub(crate) fn cranelift_value(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Speed => "speed",
            Self::SpeedAndSize => "speed_and_size",
        }
    }
}

impl FromStr for OptimizationLevel {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "none" => Ok(Self::None),
            "speed" => Ok(Self::Speed),
            "speed-and-size" | "speed_and_size" => Ok(Self::SpeedAndSize),
            _ => Err("esperado: none, speed ou speed-and-size"),
        }
    }
}

/// Optional compiler-owned IR pipeline selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IrOptimizationMode {
    /// Preserve the direct Analyzer AST to Cranelift lowering.
    None,
    /// Run admitted IR passes and representation specializations before lowering.
    Safe,
}

impl FromStr for IrOptimizationMode {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "none" => Ok(Self::None),
            "safe" => Ok(Self::Safe),
            _ => Err("esperado: none ou safe"),
        }
    }
}

/// Diagnostic optimization bundles that have not yet entered `safe`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IrExperiment {
    /// Use only the currently admitted `safe` passes.
    None,
    /// Evaluate ADR-0015 root, representation, and call-boundary specialization.
    Adr15,
}

impl FromStr for IrExperiment {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "none" => Ok(Self::None),
            "adr15" => Ok(Self::Adr15),
            _ => Err("esperado: none ou adr15"),
        }
    }
}

/// Backend options independent of the C runtime compilation profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodegenOptions {
    /// Cranelift optimization level.
    pub optimization_level: OptimizationLevel,
    /// Optional compiler-owned optimization IR mode.
    pub ir_optimization: IrOptimizationMode,
    /// Candidate bundle isolated from the admitted `safe` profile.
    pub ir_experiment: IrExperiment,
}

impl CodegenOptions {
    /// Creates the conservative unoptimized configuration.
    pub const fn unoptimized() -> Self {
        Self {
            optimization_level: OptimizationLevel::None,
            ir_optimization: IrOptimizationMode::None,
            ir_experiment: IrExperiment::None,
        }
    }

    /// Creates a configuration optimized for execution speed.
    pub const fn optimized_for_speed() -> Self {
        Self {
            optimization_level: OptimizationLevel::Speed,
            ir_optimization: IrOptimizationMode::None,
            ir_experiment: IrExperiment::None,
        }
    }
}

impl Default for CodegenOptions {
    fn default() -> Self {
        // Speed remains opt-in: the 2026-07-26 Cormen baseline regressed in
        // 25/30 cases due to additional spills and larger frames.
        Self::unoptimized()
    }
}
