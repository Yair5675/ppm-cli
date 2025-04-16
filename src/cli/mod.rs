// PPM-CLI: A Command-Line Interface for compressing data using Arithmetic Coding + Prediction by
// Partial Matching
// Copyright (C) 2025  Yair Ziv
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod model_choice;

use self::model_choice::{BuiltinModel, BuiltinOrCustomModel};
use crate::compressor::Compressor;
use crate::models::{Model, ModelCfiError};
use crate::parser::Parser;
use clap::Subcommand;
use log::{debug, error, info};
use std::fs::File;
use std::io::{BufReader, IsTerminal, Read, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Compress {
        /// Path to the file to compress. If not specified, the input data must be piped directly
        file: Option<PathBuf>,

        /// The probability model used for the compression
        #[arg(short, long, default_value_t = BuiltinOrCustomModel::Builtin(BuiltinModel::Uniform))]
        model: BuiltinOrCustomModel,
    },

    Decompress {
        /// Path to the file to decompress. If not specified, the input data must be piped directly
        file: Option<PathBuf>,

        /// The probability model that was used when compressing the file
        #[arg(short, long, default_value_t = BuiltinOrCustomModel::Builtin(BuiltinModel::Uniform))]
        model: BuiltinOrCustomModel,
    },
}

/// When trying to read input to compress/decompress, the following errors may occur
#[derive(Debug, Error)]
pub enum InputFileError {
    #[error("No path to an input file was provided, nor was it piped into the command")]
    MissingInputFile,
    #[error("Failed to read the provided input file: {0}")]
    IoError(#[from] std::io::Error),
}

/// Forms a bytes iterator for compression/decompression, either from stdin or from a path to a
/// file.<br>
fn get_bytes_iterator(
    file: Option<PathBuf>,
) -> Result<Box<dyn Iterator<Item = Result<u8, std::io::Error>>>, InputFileError> {
    match file {
        None => {
            let stdin = std::io::stdin();
            // If we aren't reading from the terminal, the input is piped into the command:
            if !stdin.is_terminal() {
                let reader = BufReader::new(stdin.lock());
                Ok(Box::new(reader.bytes()))
            } else {
                Err(InputFileError::MissingInputFile)
            }
        }
        Some(path) => Ok(Box::new(BufReader::new(File::open(path)?).bytes())),
    }
}

/// Handles a case where compressing a symbol fails
fn handle_compression_error(compression_err: anyhow::Error) {
    if let Some(ModelCfiError::UnsupportedSymbol(symbol)) = compression_err.downcast_ref() {
        error!(
            "A symbol not supported by the model ({}) was found. Skipping it",
            symbol
        );
    } else {
        error!("Failed to compress symbol; skipping it");
        debug!("Compression error: {}", compression_err);
    }
}

fn compress<I, P, M>(bytes: I, mut compressor: Compressor<M>, parser: P)
where
    I: Iterator<Item = Result<u8, std::io::Error>>,
    P: Parser,
    M: Model,
{
    info!("Compressing input stream. Unsupported or invalid symbols will be skipped");
    // Since we'll perform many writes, get a handle to stdout in a buffer:
    let stdout = std::io::stdout();
    let mut handle = std::io::BufWriter::new(stdout);
    bytes
        // Filter bytes we can't read, parse those we can:
        .filter_map(|result_byte| match result_byte {
            Ok(b) => Some(parser.parse_byte(b)),
            Err(e) => {
                error!("Failed to read byte; skipping it");
                debug!("IO Error: {}", e);
                None
            }
        })
        .flatten()
        .flat_map(|symbol| match compressor.load_symbol(symbol) {
            Ok(compressed_bytes) => Box::new(compressed_bytes),
            Err(e) => {
                handle_compression_error(e);
                Box::new(std::iter::empty()) as Box<dyn Iterator<Item = u8>>
            }
        })
        .for_each(|compressed_byte| {
            // Output the data (log failures to write just in case):
            if let Err(e) = handle.write(&[compressed_byte]) {
                error!("Failed to output compressed byte");
                debug!("Error: {}", e);
            }
        });
}
