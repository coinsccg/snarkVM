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

impl<N: Network> Request<N> {
    /// Returns `true` if the request is valid, and `false` otherwise.
    ///
    /// Verifies (challenge == challenge') && (address == address') && (serial_numbers == serial_numbers') where:
    ///     challenge' := HashToScalar(r * G, pk_sig, pr_sig, caller, \[tvk, input IDs\])
    pub fn verify(&self) -> bool {
        // Compute the function ID as `Hash(network_id, program_id, function_name)`.
        let function_id = match N::hash_bhp1024(
            &[
                U16::<N>::new(N::ID).to_bits_le(),
                self.program_id.name().to_bits_le(),
                self.program_id.network().to_bits_le(),
                self.function_name.to_bits_le(),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        ) {
            Ok(function_id) => function_id,
            Err(error) => {
                eprintln!("Failed to construct the function ID: {error}");
                return false;
            }
        };

        // Construct the signature message as `[tvk, function ID, input IDs]`.
        let mut message = Vec::with_capacity(1 + self.input_ids.len());
        message.push(self.tvk);
        message.push(function_id);

        // Retrieve the challenge from the signature.
        let challenge = self.signature.challenge();
        // Retrieve the response from the signature.
        let response = self.signature.response();

        if let Err(error) =
            self.input_ids.iter().zip_eq(&self.inputs).enumerate().try_for_each(|(index, (input_id, input))| {
                match input_id {
                    // A constant input is hashed to a field element.
                    InputID::Constant(input_hash) => {
                        // Ensure the input is a plaintext.
                        ensure!(matches!(input, Value::Plaintext(..)), "Expected a plaintext input");
                        // Hash the input to a field element.
                        let candidate_input_hash = N::hash_bhp1024(&input.to_bits_le())?;
                        // Ensure the input hash matches.
                        ensure!(*input_hash == candidate_input_hash, "Expected a constant input with the same hash");
                        // Add the input hash to the message.
                        message.push(candidate_input_hash);
                    }
                    // A public input is hashed to a field element.
                    InputID::Public(input_hash) => {
                        // Ensure the input is a plaintext.
                        ensure!(matches!(input, Value::Plaintext(..)), "Expected a plaintext input");
                        // Hash the input to a field element.
                        let candidate_input_hash = N::hash_bhp1024(&input.to_bits_le())?;
                        // Ensure the input hash matches.
                        ensure!(*input_hash == candidate_input_hash, "Expected a public input with the same hash");
                        // Add the input hash to the message.
                        message.push(candidate_input_hash);
                    }
                    // A private input is encrypted (using `tvk`) and hashed to a field element.
                    InputID::Private(input_hash) => {
                        // Ensure the input is a plaintext.
                        ensure!(matches!(input, Value::Plaintext(..)), "Expected a plaintext input");
                        // Prepare the index as a constant field element.
                        let index = Field::from_u16(index as u16);
                        // Compute the input view key as `Hash(tvk || index)`.
                        let input_view_key = N::hash_psd2(&[self.tvk, index])?;
                        // Compute the ciphertext.
                        let ciphertext = match &input {
                            Value::Plaintext(plaintext) => plaintext.encrypt_symmetric(input_view_key)?,
                            // Ensure the input is a plaintext.
                            Value::Record(..) => bail!("Expected a plaintext input, found a record input"),
                        };
                        // Hash the ciphertext to a field element.
                        let candidate_input_hash = N::hash_bhp1024(&ciphertext.to_bits_le())?;
                        // Ensure the input hash matches.
                        ensure!(
                            *input_hash == candidate_input_hash,
                            "Expected a private input with the same commitment"
                        );
                        // Add the input hash to the message.
                        message.push(candidate_input_hash);
                    }
                    // An input record is computed to its serial number.
                    InputID::Record(gamma, serial_number) => {
                        // Prepare the index as a constant field element.
                        let index = Field::from_u16(index as u16);
                        // Compute the commitment randomizer as `HashToScalar(tvk || index)`.
                        let randomizer = N::hash_to_scalar_psd2(&[self.tvk, index])?;
                        // Retrieve the record.
                        let record = match &input {
                            Value::Record(record) => record,
                            // Ensure the input is a record.
                            Value::Plaintext(..) => bail!("Expected a record input, found a plaintext input"),
                        };
                        // Compute the record commitment.
                        let commitment = record.to_commitment(&randomizer)?;
                        // Ensure the record belongs to the caller.
                        ensure!(**record.owner() == self.caller, "Input record does not belong to the caller");
                        // Ensure the record balance is less than or equal to 2^52.
                        if !(**record.balance()).to_bits_le()[52..].iter().all(|bit| !bit) {
                            bail!("Input record contains an invalid balance: {}", record.balance());
                        }

                        // Compute the generator `H` as `HashToGroup(commitment)`.
                        let h = N::hash_to_group_psd2(&[N::serial_number_domain(), commitment])?;
                        // Compute `h_r` as `(challenge * gamma) + (response * H)`, equivalent to `r * H`.
                        let h_r = (*gamma * challenge) + (h * response);
                        // Add `H`, `r * H`, and `gamma` to the message.
                        message.extend([h, h_r, *gamma].iter().map(|point| point.to_x_coordinate()));

                        // Compute `sn_nonce` as `Hash(COFACTOR * gamma)`.
                        let sn_nonce = N::hash_to_scalar_psd2(&[
                            N::serial_number_domain(),
                            gamma.mul_by_cofactor().to_x_coordinate(),
                        ])?;
                        // Compute `serial_number` as `Commit(commitment, sn_nonce)`.
                        let candidate_sn =
                            N::commit_bhp512(&(N::serial_number_domain(), commitment).to_bits_le(), &sn_nonce)?;
                        // Ensure the serial number matches.
                        ensure!(*serial_number == candidate_sn, "Expected a record input with the same serial number");
                    }
                }
                Ok(())
            })
        {
            eprintln!("Request verification failed on input checks: {error}");
            return false;
        }

        // Verify the signature.
        self.signature.verify(&self.caller, &message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Plaintext, Record};
    use snarkvm_console_account::PrivateKey;
    use snarkvm_console_network::Testnet3;

    type CurrentNetwork = Testnet3;

    pub(crate) const ITERATIONS: usize = 1000;

    #[test]
    fn test_sign_and_verify() {
        let rng = &mut test_crypto_rng();

        for _ in 0..ITERATIONS {
            // Sample a random private key and address.
            let private_key = PrivateKey::<CurrentNetwork>::new(rng).unwrap();
            let address = Address::try_from(&private_key).unwrap();

            // Construct a program ID and function name.
            let program_id = ProgramID::from_str("token.aleo").unwrap();
            let function_name = Identifier::from_str("transfer").unwrap();

            // Prepare a record belonging to the address.
            let record_string =
                format!("{{ owner: {address}.private, balance: 5u64.private, token_amount: 100u64.private }}");

            // Construct four inputs.
            let input_constant = Value::Plaintext(Plaintext::from_str("{ token_amount: 9876543210u128 }").unwrap());
            let input_public = Value::Plaintext(Plaintext::from_str("{ token_amount: 9876543210u128 }").unwrap());
            let input_private = Value::Plaintext(Plaintext::from_str("{ token_amount: 9876543210u128 }").unwrap());
            let input_record = Value::Record(Record::from_str(&record_string).unwrap());
            let inputs = vec![input_constant, input_public, input_private, input_record];

            // Construct the input types.
            let input_types = vec![
                ValueType::from_str("amount.constant").unwrap(),
                ValueType::from_str("amount.public").unwrap(),
                ValueType::from_str("amount.private").unwrap(),
                ValueType::from_str("token.record").unwrap(),
            ];

            // Compute the signed request.
            let request = Request::sign(&private_key, program_id, function_name, inputs, &input_types, rng).unwrap();
            assert!(request.verify());
        }
    }
}
