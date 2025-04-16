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

use self::model_choice::BuiltinModel;
use crate::cli::model_choice::UserModel;
use crate::compressor::Compressor;
use crate::models::{Model, ModelCfiError};
use crate::sim::DefaultSIM;
use clap::{Args, Parser, Subcommand};
use log::{debug, error, info};
use std::fs::File;
use std::io::{BufReader, IsTerminal, Read, Write};
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
    /// Compresses a file/piped data using arithmetic coding
    Compress(CodecArgs),
    /// Decompresses a file/piped data which was compressed using the `compress` command
    Decompress(CodecArgs),
}

/// CLI arguments for compression/decompression
#[derive(Args)]
pub struct CodecArgs {
    /// Path to the file that will be read. If not specified, the input data must be piped directly
    file: Option<PathBuf>,

    /// If set, the CLI will compress input **bit-by-bit**, which in some cases will result in
    /// better compression ratios.
    /// By default, this option is false, and the input will be read **byte-by-byte**.
    #[arg(short, long, default_value_t = false)]
    bit_mode: bool,

    /// Builtin probability models
    #[arg(long, group = "models", default_value_t = BuiltinModel::Uniform)]
    model: BuiltinModel,

    /// Custom probability models defined by the user, cannot be used with the --model option
    /// (which provides builtin models)
    #[arg(long, group = "models")]
    custom_model: Option<String>,
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
    file: Option<&PathBuf>,
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
    P: crate::parser::Parser,
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
    if let Err(e) = handle.flush() {
        error!("Failed to flush output");
        debug!("Error: {}", e);
    }
}

/// Converts codec args to input bytes, parser and probability model.<br>
fn parse_codec_args(
    CodecArgs { file, bit_mode, .. }: &CodecArgs,
) -> anyhow::Result<(
    impl Iterator<Item = Result<u8, std::io::Error>>,
    Box<dyn crate::parser::Parser>,
)> {
    let bytes = get_bytes_iterator(file.as_ref())?;
    let parser: Box<dyn crate::parser::Parser> = if *bit_mode {
        Box::new(crate::parser::BitParser)
    } else {
        Box::new(crate::parser::ByteParser)
    };
    Ok((bytes, parser))
}

/// Runs the CLI
pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Compress(args) => {
            let (bytes, parser) = parse_codec_args(&args)?;
            // Compress according to the model:
            match args.custom_model {
                None => {
                    let mut model = args.model.get_model();
                    let compressor = Compressor::new(&mut model);
                    compress(bytes, compressor, parser);
                }
                Some(model_name) => {
                    let mut user_model: UserModel<DefaultSIM> = UserModel::from_name(&model_name)?;
                    let compressor = Compressor::new(user_model.get_model());
                    compress(bytes, compressor, parser);
                }
            }
        }
        Commands::Decompress(CodecArgs { .. }) => {}
    }
    Ok(())
}
