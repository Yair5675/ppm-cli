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

use clap::{Parser, Subcommand, ValueEnum};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

/// Builtin models the user can use for compression/decompression
#[derive(Debug, Clone, ValueEnum)]
pub enum BuiltinModel {
    Uniform,
}

impl FromStr for BuiltinModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "uniform" => Ok(Self::Uniform),
            _ => Err(format!("{} does not match any builtin model", s)),
        }
    }
}

impl Display for BuiltinModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinModel::Uniform => write!(f, "uniform"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuiltinOrCustomModel {
    Builtin(BuiltinModel),
    Custom(String),
}

impl FromStr for BuiltinOrCustomModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<BuiltinModel>() {
            Ok(builtin) => Ok(Self::Builtin(builtin)),
            // TODO: Later validate against an SQL table whether this custom model exists
            Err(custom) => Ok(Self::Custom(custom)),
        }
    }
}

impl Display for BuiltinOrCustomModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinOrCustomModel::Builtin(builtin) => write!(f, "{}", builtin),
            BuiltinOrCustomModel::Custom(custom) => write!(f, "{}", custom),
        }
    }
}

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
