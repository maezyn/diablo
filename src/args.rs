use clap::{Args, Parser, Subcommand};
use std::net::{IpAddr};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct DiabloArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Generate unique cyclic patterns
    Pattern(PatternCommand),

    /// Perform network scans 
    Scan(ScanCommand),
}

#[derive(Debug, Args)]
pub struct PatternCommand {
    #[clap(subcommand)]
    pub command: PatternSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum PatternSubcommand {
    /// Generate a cyclic pattern
    Generate(GeneratePattern),
    Offset(FindOffset),
}

#[derive(Debug, Args)]
pub struct GeneratePattern {
    /// The length of the pattern
    pub length: usize,
}

#[derive(Debug, Args)]
pub struct FindOffset {
    /// The cyclic string to find offset of
    pub pattern: String,
}

#[derive(Debug, Args)]
pub struct ScanCommand {
    #[clap(subcommand)]
    pub command: ScanSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ScanSubcommand {
    /// Generate a cyclic pattern
    Scan(RunScan),
}

#[derive(Debug, Args)]
pub struct RunScan {
    /// Target IP Address
    pub target: IpAddr,
}

