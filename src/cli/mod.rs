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
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufReader, IsTerminal, Read};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Parser)]
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
