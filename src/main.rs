#![allow(unused)]

mod args;
mod pattern;
mod scan;
mod rop;

use anyhow::{Result};
use args::{DiabloArgs, EntityType, PatternSubcommand, ScanSubcommand};
use clap::Parser;

fn main() -> Result<()> {
    let args: DiabloArgs = DiabloArgs::parse();

    match args.entity_type {
        EntityType::Pattern(pattern_args) => {
            match pattern_args.command {
                PatternSubcommand::Generate(pattern_args) => {
                    pattern::generate(pattern_args.length);
                }
                PatternSubcommand::Offset(offset_args) => {
                    pattern::offset(&offset_args.pattern);
                }
            }
        }
        EntityType::Scan(scan_args) => {
            match scan_args.command {
                ScanSubcommand::Scan(scan_args) => {
                    scan::scan(scan_args.target);
                }
            }
        }
    }

    Ok(())
}

