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

mod input;
use input::*;

mod output;
use output::*;

use crate::{Proof, VerifyingKey};
use console::{
    network::prelude::*,
    program::{Identifier, InputID, OutputID, ProgramID, Request, Response, Value},
    types::{Field, Group},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Transition<N: Network> {
    /// The program ID.
    program_id: ProgramID<N>,
    /// The function name.
    function_name: Identifier<N>,
    /// The transition inputs.
    inputs: Vec<Input<N>>,
    /// The transition outputs.
    outputs: Vec<Output<N>>,
    /// The transition proof.
    proof: Proof<N>,
    /// The transition public key.
    tpk: Group<N>,
    /// The network fee.
    fee: u64,
}

impl<N: Network> Transition<N> {
    /// Initializes a new transition.
    pub fn new(
        program_id: ProgramID<N>,
        function_name: Identifier<N>,
        inputs: Vec<Input<N>>,
        outputs: Vec<Output<N>>,
        proof: Proof<N>,
        tpk: Group<N>,
        fee: u64,
    ) -> Self {
        Self { program_id, function_name, inputs, outputs, proof, tpk, fee }
    }

    /// Initializes a new transition from a request and response.
    pub fn from(request: &Request<N>, response: &Response<N>, proof: Proof<N>, fee: u64) -> Result<Self> {
        let program_id = *request.program_id();
        let function_name = *request.function_name();
        let num_inputs = request.inputs().len();

        let inputs = request
            .input_ids()
            .iter()
            .zip_eq(request.inputs())
            .enumerate()
            .map(|(index, (input_id, input))| {
                // Construct the transition input.
                match (input_id, input) {
                    (InputID::Constant(input_hash), Value::Plaintext(plaintext)) => {
                        // Compute the plaintext hash.
                        let plaintext_hash = N::hash_bhp1024(&plaintext.to_bits_le())?;
                        // Ensure the plaintext hash matches.
                        ensure!(*input_hash == plaintext_hash, "The constant input plaintext hash is incorrect");
                        // Return the constant input.
                        Ok(Input::Constant(*input_hash, Some(plaintext.clone())))
                    }
                    (InputID::Public(input_hash), Value::Plaintext(plaintext)) => {
                        // Compute the plaintext hash.
                        let plaintext_hash = N::hash_bhp1024(&plaintext.to_bits_le())?;
                        // Ensure the plaintext hash matches.
                        ensure!(*input_hash == plaintext_hash, "The public input plaintext hash is incorrect");
                        // Return the public input.
                        Ok(Input::Public(*input_hash, Some(plaintext.clone())))
                    }
                    (InputID::Private(input_hash), Value::Plaintext(plaintext)) => {
                        // Construct the (console) input index as a field element.
                        let index = Field::from_u16(index as u16);
                        // Compute the ciphertext, with the input view key as `Hash(tvk || index)`.
                        let ciphertext = plaintext.encrypt_symmetric(N::hash_psd2(&[*request.tvk(), index])?)?;
                        // Compute the ciphertext hash.
                        let ciphertext_hash = N::hash_bhp1024(&ciphertext.to_bits_le())?;
                        // Ensure the ciphertext hash matches.
                        ensure!(*input_hash == ciphertext_hash, "The input ciphertext hash is incorrect");
                        // Return the private input.
                        Ok(Input::Private(*input_hash, Some(ciphertext)))
                    }
                    (InputID::Record(_, serial_number), Value::Record(..)) => Ok(Input::Record(*serial_number)),
                    _ => bail!("Malformed request input: {:?}, {input}", input_id),
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let outputs = response
            .output_ids()
            .iter()
            .zip_eq(response.outputs())
            .enumerate()
            .map(|(index, (output_id, output))| {
                // Construct the transition output.
                match (output_id, output) {
                    (OutputID::Constant(output_hash), Value::Plaintext(plaintext)) => {
                        // Compute the plaintext hash.
                        let plaintext_hash = N::hash_bhp1024(&plaintext.to_bits_le())?;
                        // Ensure the plaintext hash matches.
                        ensure!(*output_hash == plaintext_hash, "The constant output plaintext hash is incorrect");
                        // Return the constant output.
                        Ok(Output::Constant(*output_hash, Some(plaintext.clone())))
                    }
                    (OutputID::Public(output_hash), Value::Plaintext(plaintext)) => {
                        // Compute the plaintext hash.
                        let plaintext_hash = N::hash_bhp1024(&plaintext.to_bits_le())?;
                        // Ensure the plaintext hash matches.
                        ensure!(*output_hash == plaintext_hash, "The public output plaintext hash is incorrect");
                        // Return the public output.
                        Ok(Output::Public(*output_hash, Some(plaintext.clone())))
                    }
                    (OutputID::Private(output_hash), Value::Plaintext(plaintext)) => {
                        // Construct the (console) output index as a field element.
                        let index = Field::from_u16((num_inputs + index) as u16);
                        // Compute the ciphertext, with the input view key as `Hash(tvk || index)`.
                        let ciphertext = plaintext.encrypt_symmetric(N::hash_psd2(&[*request.tvk(), index])?)?;
                        // Compute the ciphertext hash.
                        let ciphertext_hash = N::hash_bhp1024(&ciphertext.to_bits_le())?;
                        // Ensure the ciphertext hash matches.
                        ensure!(*output_hash == ciphertext_hash, "The output ciphertext hash is incorrect");
                        // Return the private output.
                        Ok(Output::Private(*output_hash, Some(ciphertext)))
                    }
                    (OutputID::Record(commitment, nonce, checksum), Value::Record(record)) => {
                        // Construct the (console) output index as a field element.
                        let index = Field::from_u16((num_inputs + index) as u16);
                        // Compute the encryption randomizer as `HashToScalar(tvk || index)`.
                        let randomizer = N::hash_to_scalar_psd2(&[*request.tvk(), index])?;
                        // Compute the record commitment.
                        let candidate_cm = record.to_commitment(&randomizer)?;
                        // Ensure the commitment matches.
                        ensure!(*commitment == candidate_cm, "The output record commitment is incorrect");

                        // Compute the record nonce.
                        let candidate_nonce = N::g_scalar_multiply(&randomizer).to_x_coordinate();
                        // Ensure the nonce matches.
                        ensure!(*nonce == candidate_nonce, "The output record nonce is incorrect");

                        // Encrypt the record, using the randomizer.
                        let record_ciphertext = record.encrypt(randomizer)?;
                        // Compute the record checksum, as the hash of the encrypted record.
                        let ciphertext_checksum = N::hash_bhp1024(&record_ciphertext.to_bits_le())?;
                        // Ensure the checksum matches.
                        ensure!(*checksum == ciphertext_checksum, "The output record ciphertext checksum is incorrect");

                        // Return the record output.
                        Ok(Output::Record(*commitment, *nonce, *checksum, Some(record_ciphertext)))
                    }
                    _ => bail!("Malformed response output: {:?}, {output}", output_id),
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let tpk = request.to_tpk();

        Ok(Self { program_id, function_name, inputs, outputs, proof, tpk, fee })
    }

    /// Returns `true` if the transition is valid.
    pub fn verify(&self, verifying_key: &VerifyingKey<N>) -> bool {
        // Ensure each input is valid.
        if self.inputs.iter().any(|input| !input.verify()) {
            eprintln!("Failed to verify a transition input");
            return false;
        }
        // Ensure each output is valid.
        if self.outputs.iter().any(|output| !output.verify()) {
            eprintln!("Failed to verify a transition output");
            return false;
        }

        // Compute the x- and y-coordinate of `tpk`.
        let (tpk_x, tpk_y) = self.tpk.to_xy_coordinate();
        // Construct the public inputs to verify the proof.
        let mut inputs = vec![N::Field::one(), *tpk_x, *tpk_y];
        inputs.extend(self.inputs.iter().map(|input| *input.id()));
        inputs.extend(self.outputs.iter().flat_map(Output::id).map(|id| *id));
        // Verify the proof.
        verifying_key.verify(&inputs, &self.proof)
    }
}
