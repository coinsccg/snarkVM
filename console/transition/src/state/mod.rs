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

mod randomizer;
pub use randomizer::Randomizer;

mod decrypt;
mod encrypt;

use crate::Record;
use snarkvm_console_account::{Address, ViewKey};
use snarkvm_console_network::Network;
use snarkvm_console_types::prelude::*;

/// A program's state is a set of **plaintext** variables used by a program.
/// Note: `State` is the **decrypted** form of `Record`.
pub struct State<N: Network> {
    /// The Aleo address this state belongs to.
    owner: Address<N>,
    /// The account balance in this program state.
    balance: U64<N>,
    /// The data for this program state.
    data: Field<N>,
    /// The nonce for this program state (i.e. `G^r`).
    nonce: Group<N>,
}

impl<N: Network> State<N> {
    /// Initializes a new instance of `State`.
    pub fn new(owner: Address<N>, balance: u64, data: Field<N>, nonce: Group<N>) -> Self {
        // Return the new program state.
        Self { owner, balance: U64::new(balance), data, nonce }
    }

    /// Returns the account owner.
    pub const fn owner(&self) -> Address<N> {
        self.owner
    }

    /// Returns the account balance.
    pub const fn balance(&self) -> U64<N> {
        self.balance
    }

    /// Returns the data ID.
    pub const fn data(&self) -> Field<N> {
        self.data
    }

    /// Returns the nonce for this program state.
    pub const fn nonce(&self) -> Group<N> {
        self.nonce
    }
}
