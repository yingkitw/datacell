//! xls-rs - A CLI tool and MCP server for reading, writing, and converting spreadsheet files
//!
//! Supports CSV, Excel (xlsx/xls), ODS, Parquet, and Avro formats with formula evaluation.

#![allow(dead_code)] // Modules expose APIs for library use

use anyhow::Result;
use clap::Parser;
use xls_rs::cli::{Cli, CommandHandler, DefaultCommandHandler};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let handler = DefaultCommandHandler::new();

    handler.handle(cli.command)
}
