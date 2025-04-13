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
use std::path::PathBuf;

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
