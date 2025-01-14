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

pub struct UniversalSRS<N: Network> {
    /// The universal SRS parameter.
    srs: marlin::UniversalSRS<N::PairingCurve>,
}

impl<N: Network> UniversalSRS<N> {
    /// Initializes the universal SRS.
    pub fn load(num_gates: usize) -> Result<Self> {
        let mut rng = rand::thread_rng();

        let timer = std::time::Instant::now();
        let max_degree =
            marlin::ahp::AHPForR1CS::<N::Field, marlin::MarlinHidingMode>::max_degree(num_gates, num_gates, num_gates)
                .unwrap();
        let universal_srs = Marlin::<N>::universal_setup(&max_degree, &mut rng)?;
        println!("Called universal setup: {} ms", timer.elapsed().as_millis());

        Ok(Self { srs: universal_srs })
    }

    /// Returns the circuit proving and verifying key.
    pub fn to_circuit_key(
        &self,
        assignment: &circuit::Assignment<N::Field>,
    ) -> Result<(ProvingKey<N>, VerifyingKey<N>)> {
        let timer = std::time::Instant::now();
        let (proving_key, verifying_key) = Marlin::<N>::circuit_setup(self, assignment)?;
        println!("Called setup: {} ms", timer.elapsed().as_millis());

        Ok((ProvingKey::new(proving_key), VerifyingKey::new(verifying_key)))
    }
}

impl<N: Network> Deref for UniversalSRS<N> {
    type Target = marlin::UniversalSRS<N::PairingCurve>;

    fn deref(&self) -> &Self::Target {
        &self.srs
    }
}
