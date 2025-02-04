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

mod closure;
pub use closure::*;

mod function;
pub use function::*;

mod import;
pub use import::*;

mod instruction;
pub use instruction::*;

mod bytes;
mod matches;
mod parse;
mod sample;

use console::{
    account::PrivateKey,
    network::prelude::*,
    program::{
        Balance,
        Entry,
        EntryType,
        Identifier,
        Interface,
        Literal,
        Owner,
        Plaintext,
        PlaintextType,
        ProgramID,
        Record,
        RecordType,
        RegisterType,
        Request,
        Value,
        ValueType,
    },
    types::{Address, U64},
};

use indexmap::IndexMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum ProgramDefinition {
    /// A program interface.
    Interface,
    /// A program record.
    Record,
    /// A program closure.
    Closure,
    /// A program function.
    Function,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Program<N: Network> {
    /// The ID of the program.
    id: ProgramID<N>,
    /// A map of the declared imports for the program.
    imports: IndexMap<ProgramID<N>, Import<N>>,
    /// A map of identifiers to their program declaration.
    identifiers: IndexMap<Identifier<N>, ProgramDefinition>,
    /// A map of the declared interfaces for the program.
    interfaces: IndexMap<Identifier<N>, Interface<N>>,
    /// A map of the declared record types for the program.
    records: IndexMap<Identifier<N>, RecordType<N>>,
    /// A map of the declared closures for the program.
    closures: IndexMap<Identifier<N>, Closure<N>>,
    /// A map of the declared functions for the program.
    functions: IndexMap<Identifier<N>, Function<N>>,
}

impl<N: Network> Program<N> {
    /// Initializes an empty program.
    #[inline]
    pub fn new(id: ProgramID<N>) -> Self {
        Self {
            id,
            imports: IndexMap::new(),
            identifiers: IndexMap::new(),
            interfaces: IndexMap::new(),
            records: IndexMap::new(),
            closures: IndexMap::new(),
            functions: IndexMap::new(),
        }
    }

    /// Signs a request to execute a program function.
    #[inline]
    pub fn sign<R: Rng + CryptoRng>(
        &self,
        private_key: &PrivateKey<N>,
        function_name: Identifier<N>,
        inputs: Vec<Value<N>>,
        rng: &mut R,
    ) -> Result<Request<N>> {
        // Retrieve the function from the program.
        let function = self.get_function(&function_name)?;
        // Compute the signed request.
        Request::sign(private_key, self.id, function_name, inputs, &function.input_types(), rng)
    }

    /// Returns the ID of the program.
    pub const fn id(&self) -> &ProgramID<N> {
        &self.id
    }

    /// Returns the closures in the program.
    pub const fn closures(&self) -> &IndexMap<Identifier<N>, Closure<N>> {
        &self.closures
    }

    /// Returns the functions in the program.
    pub const fn functions(&self) -> &IndexMap<Identifier<N>, Function<N>> {
        &self.functions
    }

    /// Returns `true` if the program contains a interface with the given name.
    pub fn contains_interface(&self, name: &Identifier<N>) -> bool {
        self.interfaces.contains_key(name)
    }

    /// Returns `true` if the program contains a record with the given name.
    pub fn contains_record(&self, name: &Identifier<N>) -> bool {
        self.records.contains_key(name)
    }

    /// Returns `true` if the program contains a closure with the given name.
    pub fn contains_closure(&self, name: &Identifier<N>) -> bool {
        self.closures.contains_key(name)
    }

    /// Returns `true` if the program contains a function with the given name.
    pub fn contains_function(&self, name: &Identifier<N>) -> bool {
        self.functions.contains_key(name)
    }

    /// Returns the interface with the given name.
    pub fn get_interface(&self, name: &Identifier<N>) -> Result<Interface<N>> {
        // Attempt to retrieve the interface.
        let interface =
            self.interfaces.get(name).cloned().ok_or_else(|| anyhow!("Interface '{name}' is not defined."))?;
        // Ensure the interface contains members.
        ensure!(!interface.members().is_empty(), "Interface '{name}' is missing members.");
        // Return the interface.
        Ok(interface)
    }

    /// Returns the record with the given name.
    pub fn get_record(&self, name: &Identifier<N>) -> Result<RecordType<N>> {
        self.records.get(name).cloned().ok_or_else(|| anyhow!("Record '{name}' is not defined."))
    }

    /// Returns the closure with the given name.
    pub fn get_closure(&self, name: &Identifier<N>) -> Result<Closure<N>> {
        // Attempt to retrieve the closure.
        let closure = self.closures.get(name).cloned().ok_or_else(|| anyhow!("Closure '{name}' is not defined."))?;
        // Ensure there are input statements in the closure.
        ensure!(!closure.inputs().is_empty(), "Cannot evaluate a closure without input statements");
        // Ensure the number of inputs is within the allowed range.
        ensure!(closure.inputs().len() <= N::MAX_INPUTS, "Closure exceeds maximum number of inputs");
        // Ensure there are instructions in the closure.
        ensure!(!closure.instructions().is_empty(), "Cannot evaluate a closure without instructions");
        // Ensure the number of outputs is within the allowed range.
        ensure!(closure.outputs().len() <= N::MAX_OUTPUTS, "Closure exceeds maximum number of outputs");
        // Return the closure.
        Ok(closure)
    }

    /// Returns the function with the given name.
    pub fn get_function(&self, name: &Identifier<N>) -> Result<Function<N>> {
        // Attempt to retrieve the function.
        let function = self.functions.get(name).cloned().ok_or_else(|| anyhow!("Function '{name}' is not defined."))?;
        // Ensure there are input statements in the function.
        ensure!(!function.inputs().is_empty(), "Cannot evaluate a function without input statements");
        // Ensure the number of inputs is within the allowed range.
        ensure!(function.inputs().len() <= N::MAX_INPUTS, "Function exceeds maximum number of inputs");
        // Ensure there are instructions in the function.
        ensure!(!function.instructions().is_empty(), "Cannot evaluate a function without instructions");
        // Ensure the number of outputs is within the allowed range.
        ensure!(function.outputs().len() <= N::MAX_OUTPUTS, "Function exceeds maximum number of outputs");
        // Return the function.
        Ok(function)
    }
}

impl<N: Network> Program<N> {
    /// Adds a new import statement to the program.
    ///
    /// # Errors
    /// This method will halt if the imported program was previously added.
    #[inline]
    fn add_import(&mut self, import: Import<N>) -> Result<()> {
        // Retrieve the imported program name.
        let import_name = *import.name();

        // Ensure the import name is new.
        ensure!(self.is_unique_name(&import_name), "'{import_name}' is already in use.");
        // Ensure the import name is not a reserved opcode.
        ensure!(!self.is_reserved_opcode(&import_name), "'{import_name}' is a reserved opcode.");
        // Ensure the import name is not a reserved keyword.
        ensure!(!self.is_reserved_keyword(&import_name), "'{import_name}' is a reserved keyword.");

        // Ensure the import is new.
        ensure!(!self.imports.contains_key(import.id()), "Import '{}' is already defined.", import.id());

        // Add the import statement to the program.
        if self.imports.insert(*import.id(), import.clone()).is_some() {
            bail!("'{}' already exists in the program.", import.id())
        }
        Ok(())
    }

    /// Adds a new interface to the program.
    ///
    /// # Errors
    /// This method will halt if the interface was previously added.
    /// This method will halt if the interface name is already in use in the program.
    /// This method will halt if the interface name is a reserved opcode or keyword.
    /// This method will halt if any interfaces in the interface's members are not already defined.
    #[inline]
    fn add_interface(&mut self, interface: Interface<N>) -> Result<()> {
        // Retrieve the interface name.
        let interface_name = *interface.name();

        // Ensure the interface name is new.
        ensure!(self.is_unique_name(&interface_name), "'{interface_name}' is already in use.");
        // Ensure the interface name is not a reserved opcode.
        ensure!(!self.is_reserved_opcode(&interface_name), "'{interface_name}' is a reserved opcode.");
        // Ensure the interface name is not a reserved keyword.
        ensure!(!self.is_reserved_keyword(&interface_name), "'{interface_name}' is a reserved keyword.");

        // Ensure the interface contains members.
        ensure!(!interface.members().is_empty(), "Interface '{interface_name}' is missing members.");

        // Ensure all interface members are well-formed.
        // Note: This design ensures cyclic references are not possible.
        for (identifier, plaintext_type) in interface.members() {
            // Ensure the member name is not a reserved keyword.
            ensure!(!self.is_reserved_keyword(identifier), "'{identifier}' is a reserved keyword.");
            // Ensure the member type is already defined in the program.
            match plaintext_type {
                PlaintextType::Literal(..) => continue,
                PlaintextType::Interface(member_identifier) => {
                    // Ensure the member interface name exists in the program.
                    if !self.interfaces.contains_key(member_identifier) {
                        bail!("'{member_identifier}' in interface '{}' is not defined.", interface_name)
                    }
                }
            }
        }

        // Add the interface name to the identifiers.
        if self.identifiers.insert(interface_name, ProgramDefinition::Interface).is_some() {
            bail!("'{}' already exists in the program.", interface_name)
        }
        // Add the interface to the program.
        if self.interfaces.insert(interface_name, interface).is_some() {
            bail!("'{}' already exists in the program.", interface_name)
        }
        Ok(())
    }

    /// Adds a new record to the program.
    ///
    /// # Errors
    /// This method will halt if the record was previously added.
    /// This method will halt if the record name is already in use in the program.
    /// This method will halt if the record name is a reserved opcode or keyword.
    /// This method will halt if any records in the record's members are not already defined.
    #[inline]
    fn add_record(&mut self, record: RecordType<N>) -> Result<()> {
        // For now, ensure only one record type exists in the program.
        ensure!(self.records.len() <= 1, "Only one record type is allowed in the program (for now).");

        // Retrieve the record name.
        let record_name = *record.name();

        // Ensure the record name is new.
        ensure!(self.is_unique_name(&record_name), "'{record_name}' is already in use.");
        // Ensure the record name is not a reserved opcode.
        ensure!(!self.is_reserved_opcode(&record_name), "'{record_name}' is a reserved opcode.");
        // Ensure the record name is not a reserved keyword.
        ensure!(!self.is_reserved_keyword(&record_name), "'{record_name}' is a reserved keyword.");

        // Ensure all record entries are well-formed.
        // Note: This design ensures cyclic references are not possible.
        for (identifier, entry_type) in record.entries() {
            // Ensure the member name is not a reserved keyword.
            ensure!(!self.is_reserved_keyword(identifier), "'{identifier}' is a reserved keyword.");
            // Ensure the member type is already defined in the program.
            match entry_type {
                // Ensure the plaintext type is already defined.
                EntryType::Constant(plaintext_type)
                | EntryType::Public(plaintext_type)
                | EntryType::Private(plaintext_type) => match plaintext_type {
                    PlaintextType::Literal(..) => continue,
                    PlaintextType::Interface(identifier) => {
                        if !self.interfaces.contains_key(identifier) {
                            bail!("Interface '{identifier}' in record '{record_name}' is not defined.")
                        }
                    }
                },
            }
        }

        // Add the record name to the identifiers.
        if self.identifiers.insert(record_name, ProgramDefinition::Record).is_some() {
            bail!("'{record_name}' already exists in the program.")
        }
        // Add the record to the program.
        if self.records.insert(record_name, record).is_some() {
            bail!("'{record_name}' already exists in the program.")
        }
        Ok(())
    }

    /// Adds a new closure to the program.
    ///
    /// # Errors
    /// This method will halt if the closure was previously added.
    /// This method will halt if the closure name is already in use in the program.
    /// This method will halt if the closure name is a reserved opcode or keyword.
    /// This method will halt if any registers are assigned more than once.
    /// This method will halt if the registers are not incrementing monotonically.
    /// This method will halt if an input type references a non-existent definition.
    /// This method will halt if an operand register does not already exist in memory.
    /// This method will halt if a destination register already exists in memory.
    /// This method will halt if an output register does not already exist.
    /// This method will halt if an output type references a non-existent definition.
    #[inline]
    fn add_closure(&mut self, closure: Closure<N>) -> Result<()> {
        // Retrieve the closure name.
        let closure_name = *closure.name();

        // Ensure the closure name is new.
        ensure!(self.is_unique_name(&closure_name), "'{closure_name}' is already in use.");
        // Ensure the closure name is not a reserved opcode.
        ensure!(!self.is_reserved_opcode(&closure_name), "'{closure_name}' is a reserved opcode.");
        // Ensure the closure name is not a reserved keyword.
        ensure!(!self.is_reserved_keyword(&closure_name), "'{closure_name}' is a reserved keyword.");

        // Ensure there are input statements in the closure.
        ensure!(!closure.inputs().is_empty(), "Cannot evaluate a closure without input statements");
        // Ensure the number of inputs is within the allowed range.
        ensure!(closure.inputs().len() <= N::MAX_INPUTS, "Closure exceeds maximum number of inputs");
        // Ensure there are instructions in the closure.
        ensure!(!closure.instructions().is_empty(), "Cannot evaluate a closure without instructions");
        // Ensure the number of outputs is within the allowed range.
        ensure!(closure.outputs().len() <= N::MAX_OUTPUTS, "Closure exceeds maximum number of outputs");

        // Add the function name to the identifiers.
        if self.identifiers.insert(closure_name, ProgramDefinition::Closure).is_some() {
            bail!("'{closure_name}' already exists in the program.")
        }
        // Add the closure to the program.
        if self.closures.insert(closure_name, closure).is_some() {
            bail!("'{closure_name}' already exists in the program.")
        }
        Ok(())
    }

    /// Adds a new function to the program.
    ///
    /// # Errors
    /// This method will halt if the function was previously added.
    /// This method will halt if the function name is already in use in the program.
    /// This method will halt if the function name is a reserved opcode or keyword.
    /// This method will halt if any registers are assigned more than once.
    /// This method will halt if the registers are not incrementing monotonically.
    /// This method will halt if an input type references a non-existent definition.
    /// This method will halt if an operand register does not already exist in memory.
    /// This method will halt if a destination register already exists in memory.
    /// This method will halt if an output register does not already exist.
    /// This method will halt if an output type references a non-existent definition.
    #[inline]
    fn add_function(&mut self, function: Function<N>) -> Result<()> {
        // Retrieve the function name.
        let function_name = *function.name();

        // Ensure the function name is new.
        ensure!(self.is_unique_name(&function_name), "'{function_name}' is already in use.");
        // Ensure the function name is not a reserved opcode.
        ensure!(!self.is_reserved_opcode(&function_name), "'{function_name}' is a reserved opcode.");
        // Ensure the function name is not a reserved keyword.
        ensure!(!self.is_reserved_keyword(&function_name), "'{function_name}' is a reserved keyword.");

        // Ensure there are input statements in the function.
        ensure!(!function.inputs().is_empty(), "Cannot evaluate a function without input statements");
        // Ensure the number of inputs is within the allowed range.
        ensure!(function.inputs().len() <= N::MAX_INPUTS, "Function exceeds maximum number of inputs");
        // Ensure there are instructions in the function.
        ensure!(!function.instructions().is_empty(), "Cannot evaluate a function without instructions");
        // Ensure the number of outputs is within the allowed range.
        ensure!(function.outputs().len() <= N::MAX_OUTPUTS, "Function exceeds maximum number of outputs");

        // Add the function name to the identifiers.
        if self.identifiers.insert(function_name, ProgramDefinition::Function).is_some() {
            bail!("'{function_name}' already exists in the program.")
        }
        // Add the function to the program.
        if self.functions.insert(function_name, function).is_some() {
            bail!("'{function_name}' already exists in the program.")
        }
        Ok(())
    }
}

impl<N: Network> Program<N> {
    #[rustfmt::skip]
    const KEYWORDS: &'static [&'static str] = &[
        // Mode
        "const",
        "constant",
        "public",
        "private",
        // Literals
        "address",
        "boolean",
        "field",
        "group",
        "i8",
        "i16",
        "i32",
        "i64",
        "i128",
        "u8",
        "u16",
        "u32",
        "u64",
        "u128",
        "scalar",
        "string",
        // Boolean
        "true",
        "false",
        // Statements
        "input",
        "output",
        "as",
        "into",
        // Program
        "function",
        "interface",
        "record",
        "closure",
        "program",
        "global",
        // Reserved (catch all)
        "return",
        "break",
        "assert",
        "continue",
        "let",
        "if",
        "else",
        "while",
        "for",
        "switch",
        "case",
        "default",
        "match",
        "enum",
        "struct",
        "union",
        "trait",
        "impl",
        "type",
    ];

    /// Returns `true` if the given name does not already exist in the program.
    fn is_unique_name(&self, name: &Identifier<N>) -> bool {
        !self.identifiers.contains_key(name)
    }

    /// Returns `true` if the given name is a reserved opcode.
    fn is_reserved_opcode(&self, name: &Identifier<N>) -> bool {
        // Convert the given name to a string.
        let name = name.to_string();
        // Check if the given name matches the root of any opcode (the first part, up to the first '.').
        Instruction::<N>::OPCODES.iter().any(|opcode| (**opcode).split('.').next() == Some(&name))
    }

    /// Returns `true` if the given name uses a reserved keyword.
    fn is_reserved_keyword(&self, name: &Identifier<N>) -> bool {
        // Convert the given name to a string.
        let name = name.to_string();
        // Check if the name is a keyword.
        Self::KEYWORDS.iter().any(|keyword| *keyword == name)
    }
}

impl<N: Network> TypeName for Program<N> {
    /// Returns the type name as a string.
    #[inline]
    fn type_name() -> &'static str {
        "program"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Stack;
    use circuit::network::AleoV0;
    use console::network::Testnet3;

    type CurrentNetwork = Testnet3;
    type CurrentAleo = AleoV0;

    #[test]
    fn test_program_interface() -> Result<()> {
        // Create a new interface.
        let interface = Interface::<CurrentNetwork>::from_str(
            r"
interface message:
    first as field;
    second as field;",
        )?;

        // Initialize a new program.
        let mut program = Program::<CurrentNetwork>::new(ProgramID::from_str("unknown")?);

        // Add the interface to the program.
        program.add_interface(interface.clone())?;
        // Ensure the interface was added.
        assert!(program.contains_interface(&Identifier::from_str("message")?));
        // Ensure the retrieved interface matches.
        assert_eq!(interface, program.get_interface(&Identifier::from_str("message")?)?);

        Ok(())
    }

    #[test]
    fn test_program_record() -> Result<()> {
        // Create a new record.
        let record = RecordType::<CurrentNetwork>::from_str(
            r"
record foo:
    owner as address.private;
    balance as u64.private;
    first as field.private;
    second as field.public;",
        )?;

        // Initialize a new program.
        let mut program = Program::<CurrentNetwork>::new(ProgramID::from_str("unknown")?);

        // Add the record to the program.
        program.add_record(record.clone())?;
        // Ensure the record was added.
        assert!(program.contains_record(&Identifier::from_str("foo")?));
        // Ensure the retrieved record matches.
        assert_eq!(record, program.get_record(&Identifier::from_str("foo")?)?);

        Ok(())
    }

    #[test]
    fn test_program_function() -> Result<()> {
        // Create a new function.
        let function = Function::<CurrentNetwork>::from_str(
            r"
function compute:
    input r0 as field.public;
    input r1 as field.private;
    add r0 r1 into r2;
    output r2 as field.private;",
        )?;

        // Initialize a new program.
        let mut program = Program::<CurrentNetwork>::new(ProgramID::from_str("unknown")?);

        // Add the function to the program.
        program.add_function(function.clone())?;
        // Ensure the function was added.
        assert!(program.contains_function(&Identifier::from_str("compute")?));
        // Ensure the retrieved function matches.
        assert_eq!(function, program.get_function(&Identifier::from_str("compute")?)?);

        Ok(())
    }

    #[test]
    fn test_program_evaluate_function() {
        let program = Program::<CurrentNetwork>::from_str(
            r"
    program example;

    function foo:
        input r0 as field.public;
        input r1 as field.private;
        add r0 r1 into r2;
        output r2 as field.private;
    ",
        )
        .unwrap();

        // Declare the function name.
        let function_name = Identifier::from_str("foo").unwrap();
        // Declare the function inputs.
        let inputs = vec![
            Value::<CurrentNetwork>::Plaintext(Plaintext::from_str("2field").unwrap()),
            Value::Plaintext(Plaintext::from_str("3field").unwrap()),
        ];

        // Prepare the stack.
        let mut stack = Stack::<CurrentNetwork, CurrentAleo>::new(program).unwrap();

        // Run the function.
        let expected = Value::Plaintext(Plaintext::<CurrentNetwork>::from_str("5field").unwrap());
        let candidate = stack.test_evaluate(&function_name, &inputs).unwrap();
        assert_eq!(1, candidate.len());
        assert_eq!(expected, candidate[0]);

        // Re-run to ensure state continues to work.
        let candidate = stack.test_evaluate(&function_name, &inputs).unwrap();
        assert_eq!(1, candidate.len());
        assert_eq!(expected, candidate[0]);
    }

    #[test]
    fn test_program_evaluate_interface_and_function() {
        // Initialize a new program.
        let (string, program) = Program::<CurrentNetwork>::parse(
            r"
program example;

interface message:
    first as field;
    second as field;

function compute:
    input r0 as message.private;
    add r0.first r0.second into r1;
    output r1 as field.private;",
        )
        .unwrap();
        assert!(string.is_empty(), "Parser did not consume all of the string: '{string}'");

        // Declare the function name.
        let function_name = Identifier::from_str("compute").unwrap();
        // Declare the input value.
        let input =
            Value::<CurrentNetwork>::Plaintext(Plaintext::from_str("{ first: 2field, second: 3field }").unwrap());
        // Declare the expected output value.
        let expected = Value::Plaintext(Plaintext::from_str("5field").unwrap());

        // Prepare the stack.
        let mut stack = Stack::<CurrentNetwork, CurrentAleo>::new(program).unwrap();

        // Compute the output value.
        let candidate = stack.test_evaluate(&function_name, &[input.clone()]).unwrap();
        assert_eq!(1, candidate.len());
        assert_eq!(expected, candidate[0]);

        // Re-run to ensure state continues to work.
        let candidate = stack.test_evaluate(&function_name, &[input]).unwrap();
        assert_eq!(1, candidate.len());
        assert_eq!(expected, candidate[0]);
    }

    #[test]
    fn test_program_evaluate_record_and_function() {
        // Initialize a new program.
        let (string, program) = Program::<CurrentNetwork>::parse(
            r"
program token;

record token:
    owner as address.private;
    balance as u64.private;
    token_amount as u64.private;

function compute:
    input r0 as token.record;
    add r0.token_amount r0.token_amount into r1;
    output r1 as u64.private;",
        )
        .unwrap();
        assert!(string.is_empty(), "Parser did not consume all of the string: '{string}'");

        // Declare the function name.
        let function_name = Identifier::from_str("compute").unwrap();
        // Declare the input value.
        let input =
            Value::<CurrentNetwork>::Record(Record::from_str("{ owner: aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.private, balance: 5u64.private, token_amount: 100u64.private }").unwrap());
        // Declare the expected output value.
        let expected = Value::Plaintext(Plaintext::from_str("200u64").unwrap());

        // Prepare the stack.
        let mut stack = Stack::<CurrentNetwork, CurrentAleo>::new(program).unwrap();

        // Compute the output value.
        let candidate = stack.test_evaluate(&function_name, &[input.clone()]).unwrap();
        assert_eq!(1, candidate.len());
        assert_eq!(expected, candidate[0]);

        // Re-run to ensure state continues to work.
        let candidate = stack.test_evaluate(&function_name, &[input]).unwrap();
        assert_eq!(1, candidate.len());
        assert_eq!(expected, candidate[0]);
    }

    #[test]
    fn test_program_evaluate_call() {
        // Initialize a new program.
        let (string, program) = Program::<CurrentNetwork>::parse(
            r"
program example_call;

// (a + (a + b)) + (a + b) == (3a + 2b)
closure execute:
    input r0 as field;
    input r1 as field;
    add r0 r1 into r2;
    add r0 r2 into r3;
    add r2 r3 into r4;
    output r4 as field;
    output r3 as field;
    output r2 as field;

function compute:
    input r0 as field.private;
    input r1 as field.public;
    call execute r0 r1 into r2 r3 r4;
    output r2 as field.private;
    output r3 as field.private;
    output r4 as field.private;",
        )
        .unwrap();
        assert!(string.is_empty(), "Parser did not consume all of the string: '{string}'");

        // Declare the function name.
        let function_name = Identifier::from_str("compute").unwrap();

        // Declare the input value.
        let r0 = Value::<CurrentNetwork>::Plaintext(Plaintext::from_str("3field").unwrap());
        let r1 = Value::<CurrentNetwork>::Plaintext(Plaintext::from_str("5field").unwrap());

        // Declare the expected output value.
        let r2 = Value::Plaintext(Plaintext::from_str("19field").unwrap());
        let r3 = Value::Plaintext(Plaintext::from_str("11field").unwrap());
        let r4 = Value::Plaintext(Plaintext::from_str("8field").unwrap());

        // Prepare the stack.
        let mut stack = Stack::<CurrentNetwork, CurrentAleo>::new(program).unwrap();

        // Compute the output value.
        let candidate = stack.test_evaluate(&function_name, &[r0.clone(), r1.clone()]).unwrap();
        assert_eq!(3, candidate.len());
        assert_eq!(r2, candidate[0]);
        assert_eq!(r3, candidate[1]);
        assert_eq!(r4, candidate[2]);

        // Re-run to ensure state continues to work.
        let candidate = stack.test_evaluate(&function_name, &[r0.clone(), r1.clone()]).unwrap();
        assert_eq!(3, candidate.len());
        assert_eq!(r2, candidate[0]);
        assert_eq!(r3, candidate[1]);
        assert_eq!(r4, candidate[2]);

        use circuit::Eject;

        // Re-run to ensure state continues to work.
        let candidate = stack.test_execute(&function_name, &[r0, r1]).unwrap();
        assert_eq!(3, candidate.len());
        assert_eq!(r2, candidate[0].eject_value());
        assert_eq!(r3, candidate[1].eject_value());
        assert_eq!(r4, candidate[2].eject_value());
    }

    #[test]
    fn test_program_evaluate_cast() {
        // Initialize a new program.
        let (string, program) = Program::<CurrentNetwork>::parse(
            r"
program token_with_cast;

record token:
    owner as address.private;
    balance as u64.private;
    token_amount as u64.private;

function compute:
    input r0 as token.record;
    cast r0.owner r0.balance r0.token_amount into r1 as token.record;
    output r1 as token.record;",
        )
        .unwrap();
        assert!(string.is_empty(), "Parser did not consume all of the string: '{string}'");

        // Declare the function name.
        let function_name = Identifier::from_str("compute").unwrap();
        // Declare the input value.
        let input_record = Record::from_str("{ owner: aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.private, balance: 5u64.private, token_amount: 100u64.private }").unwrap();
        let input = Value::<CurrentNetwork>::Record(input_record.clone());
        // Declare the expected output value.
        let expected = Value::Record(input_record);

        // Prepare the stack.
        let mut stack = Stack::<CurrentNetwork, CurrentAleo>::new(program).unwrap();

        // Compute the output value.
        let candidate = stack.test_evaluate(&function_name, &[input.clone()]).unwrap();
        assert_eq!(1, candidate.len());
        assert_eq!(expected, candidate[0]);

        // Re-run to ensure state continues to work.
        let candidate = stack.test_evaluate(&function_name, &[input]).unwrap();
        assert_eq!(1, candidate.len());
        assert_eq!(expected, candidate[0]);
    }
}
