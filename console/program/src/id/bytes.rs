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

impl<N: Network> FromBytes for ProgramID<N> {
    /// Reads the program ID from a buffer.
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        let name = FromBytes::read_le(&mut reader)?;

        let variant = u8::read_le(&mut reader)?;
        match variant {
            0 => Ok(Self { name, network: None }),
            1 => {
                let network = FromBytes::read_le(&mut reader)?;
                Ok(Self { name, network: Some(network) })
            }
            _ => Err(error(format!("Failed to parse program ID. Invalid variant '{variant}'"))),
        }
    }
}

impl<N: Network> ToBytes for ProgramID<N> {
    /// Writes the program ID to a buffer.
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        self.name.write_le(&mut writer)?;
        match self.network {
            None => 0u8.write_le(&mut writer),
            Some(ref network) => {
                1u8.write_le(&mut writer)?;
                network.write_le(&mut writer)
            }
        }
    }
}
