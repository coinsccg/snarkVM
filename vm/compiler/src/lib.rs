// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

#![forbid(unsafe_code)]
#![allow(clippy::module_inception)]
// TODO (howardwu): Remove me after tracing.
#![allow(clippy::print_in_format_impl)]
#![allow(dead_code)]

#[macro_use]
extern crate tracing;

#[allow(dead_code, unused_imports)]
mod ledger;
pub use ledger::*;

mod process;
pub use process::*;

mod program;
pub use program::*;

mod snark;
pub use snark::*;

pub mod transition;
