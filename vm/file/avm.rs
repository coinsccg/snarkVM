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

use snarkvm_compiler::Program;
use snarkvm_utilities::{FromBytes, ToBytes};

use anyhow::{anyhow, ensure, Result};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

// TODO (howardwu): Unify these higher up.
type A = snarkvm_circuit::AleoV0;
type N = <A as snarkvm_circuit::Environment>::Network;

static AVM_FILE_EXTENSION: &str = "avm";

pub struct AVMFile {
    /// The file name (without the extension).
    file_name: String,
    /// The program.
    program: Program<N>,
}

impl AVMFile {
    /// Reads the program from the given file path, if it exists.
    pub fn from_path(path: &Path) -> Result<Self> {
        // Ensure the path is well-formed.
        Self::check_path(path)?;

        // Retrieve the file name.
        let file_name = path
            .file_stem()
            .ok_or_else(|| anyhow!("File name not found."))?
            .to_str()
            .ok_or_else(|| anyhow!("File name not found."))?
            .to_string();

        // Read the program string.
        let program_bytes = fs::read(&path)?;
        // Parse the program string.
        let program = Program::from_bytes_le(&program_bytes)?;

        Ok(Self { file_name, program })
    }

    /// Returns the file name.
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Returns the program.
    pub const fn program(&self) -> &Program<N> {
        &self.program
    }

    /// Returns `true` if the file exists at the given path.
    pub fn exists_at(&self, path: &Path) -> bool {
        // Ensure the path is well-formed.
        Self::check_path(path).is_ok() && path.exists()
    }

    /// Writes the program string to the file.
    pub fn write_to(&self, path: &Path) -> Result<()> {
        // Ensure the path is well-formed.
        Self::check_path(path)?;

        // Retrieve the file name.
        let file_name = path
            .file_stem()
            .ok_or_else(|| anyhow!("File name not found."))?
            .to_str()
            .ok_or_else(|| anyhow!("File name not found."))?
            .to_string();
        // Ensure the file name matches the expected file name.
        ensure!(file_name == self.file_name, "File name does not match.");

        Ok(File::create(&path)?.write_all(&self.program.to_bytes_le()?)?)
    }

    /// Removes the file at the given path, if it exists.
    pub fn remove(&self, path: &Path) -> Result<()> {
        // If the path does not exist, do nothing.
        if !path.exists() {
            Ok(())
        } else {
            // Ensure the path is well-formed.
            Self::check_path(path)?;
            // Remove the file.
            Ok(fs::remove_file(&path)?)
        }
    }
}

impl AVMFile {
    /// Checks that the given path has the correct file extension.
    fn check_path(path: &Path) -> Result<()> {
        // Ensure the given path is a file.
        ensure!(path.is_file(), "The path is not a file.");

        // Ensure the given path has the correct file extension.
        let extension = path.extension().ok_or_else(|| anyhow!("File extension not found."))?;
        ensure!(extension == AVM_FILE_EXTENSION, "File extension is incorrect.");

        // Ensure the given path exists.
        ensure!(path.exists(), "File does not exist: {}", path.display());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_circuit::Parser;

    type CurrentNetwork = N;

    fn temp_dir() -> std::path::PathBuf {
        tempfile::tempdir().expect("Failed to open temporary directory").into_path()
    }

    #[test]
    fn test_from_path() {
        // Initialize a temporary directory.
        let directory = temp_dir();

        let program_string = r"
program token;

record token:
    owner as address.private;
    balance as u64.private;
    token_amount as u64.private;

function compute:
    input r0 as token.record;
    add r0.token_amount r0.token_amount into r1;
    output r1 as u64.private;";

        // Initialize a new program.
        let (string, program) = Program::<CurrentNetwork>::parse(program_string).unwrap();
        assert!(string.is_empty(), "Parser did not consume all of the string: '{string}'");

        // Write the program bytes to a file in the temporary directory.
        let path = directory.join("token.avm");
        let mut file = File::create(&path).unwrap();
        file.write_all(&program.to_bytes_le().unwrap()).unwrap();

        // Read the program from the path.
        let file = AVMFile::from_path(&path).unwrap();

        assert_eq!("token", file.file_name());
        assert_eq!(&program, file.program());
    }
}
