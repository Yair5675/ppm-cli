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

use clap::ValueEnum;
use std::fmt::{Display, Formatter};

/// Builtin models the user can use for compression/decompression
#[derive(Debug, Clone, ValueEnum)]
pub enum BuiltinModel {
    Uniform,
}

impl Display for BuiltinModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinModel::Uniform => write!(f, "uniform"),
        }
    }
}
