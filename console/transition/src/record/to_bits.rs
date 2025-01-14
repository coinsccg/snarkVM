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

use super::*;

impl<N: Network> ToBits for Record<N> {
    /// Returns this data as a list of **little-endian** bits.
    fn to_bits_le(&self) -> Vec<bool> {
        [self.owner, self.balance, self.data, self.nonce.to_x_coordinate(), self.mac, self.bcm.to_x_coordinate()]
            .to_bits_le()
    }

    /// Returns this data as a list of **big-endian** bits.
    fn to_bits_be(&self) -> Vec<bool> {
        [self.owner, self.balance, self.data, self.nonce.to_x_coordinate(), self.mac, self.bcm.to_x_coordinate()]
            .to_bits_be()
    }
}
