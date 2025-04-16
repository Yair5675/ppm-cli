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

#![allow(dead_code)]

mod bit_buffer;
mod cli;
mod compressor;
mod decompressor;
mod frequencies;
mod interval;
mod models;
mod number_types;
mod parser;
mod sim;

use log::error;
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();
    if let Err(e) = cli::run() {
        error!("{}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
