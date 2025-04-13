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

use std::str::FromStr;
use clap::ValueEnum;

/// Builtin models the user can use for compression/decompression
#[derive(Debug, Clone, ValueEnum)]
pub enum BuiltinModel {
    Uniform
}

impl FromStr for BuiltinModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "uniform" => Ok(Self::Uniform),
            _ => Err(format!("{} does not match any builtin model", s))
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuiltinOrCustomModel {
    Builtin(BuiltinModel),
    Custom(String)
}

impl FromStr for BuiltinOrCustomModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<BuiltinModel>() {
            Ok(builtin) => Ok(Self::Builtin(builtin)),
            // TODO: Later validate against an SQL table whether this custom model exists
            Err(custom) => Ok(Self::Custom(custom))
        }
    }
}
