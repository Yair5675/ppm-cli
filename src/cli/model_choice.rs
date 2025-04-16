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

use crate::models::distributions::{
    custom::CustomDistributionModel, uniform::UniformDistributionModel,
};
use crate::models::Model;
use crate::parser::{ByteParser, Parser};
use crate::sim::{DefaultSIM, SymbolIndexMapping};
use anyhow::Result;
use clap::ValueEnum;
use std::fmt::{Display, Formatter};

/// Builtin models the user can use for compression/decompression
#[derive(Debug, Clone, ValueEnum)]
pub enum BuiltinModel {
    Uniform,
}

impl BuiltinModel {
    pub fn get_model(&self) -> impl Model {
        match self {
            BuiltinModel::Uniform => UniformDistributionModel::new(DefaultSIM),
        }
    }

    pub fn get_parser(&self) -> impl Parser {
        match self {
            BuiltinModel::Uniform => ByteParser,
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

/// Custom models made by the user
pub struct UserModel<SIM: SymbolIndexMapping> {
    /// The model's name
    name: String,
    /// If it's a bit-model or byte-model
    is_bit_model: bool,
    /// The actual custom distribution
    custom_distribution_model: CustomDistributionModel<SIM>,
}

impl<SIM: SymbolIndexMapping> UserModel<SIM> {
    pub fn get_model(&mut self) -> &mut CustomDistributionModel<SIM> {
        &mut self.custom_distribution_model
    }

    pub fn from_name(_name: &str) -> Result<Self> {
        todo!("Implement according to todo-features.txt")
    }
}
