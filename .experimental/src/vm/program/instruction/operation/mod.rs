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

mod call;
pub use call::*;

mod cast;
pub use cast::*;

mod literals;
pub use literals::*;

mod macros;

use crate::vm::Opcode;
use console::network::prelude::*;

pub trait Operation<N: Network, Value: Parser + ToBits, CircuitValue, ValueType: Parser, const NUM_OPERANDS: usize> {
    /// The opcode of the operation.
    const OPCODE: Opcode;

    /// Returns the result of evaluating the operation on the given inputs.
    fn evaluate(inputs: &[Value; NUM_OPERANDS]) -> Result<Value>;

    /// Returns the result of executing the operation on the given circuit inputs.
    fn execute(inputs: &[CircuitValue; NUM_OPERANDS]) -> Result<CircuitValue>;

    /// Returns the output type from the given input types.
    fn output_type(inputs: &[ValueType; NUM_OPERANDS]) -> Result<ValueType>;
}

/// Compute the absolute value of `first`, checking for overflow/underflow, and storing the outcome in `destination`.
pub type Abs<N, A> = UnaryLiteral<N, A, AbsOperation<N, A>>;

crate::operation!(
    pub struct AbsOperation<console::prelude::AbsChecked, circuit::prelude::AbsChecked, abs_checked, "abs"> {
        I8 => I8 ("ensure overflows halt"),
        I16 => I16 ("ensure overflows halt"),
        I32 => I32 ("ensure overflows halt"),
        I64 => I64 ("ensure overflows halt"),
        I128 => I128 ("ensure overflows halt"),
    }
);

/// Compute the absolute value of `first`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
pub type AbsWrapped<N, A> = UnaryLiteral<N, A, AbsWrappedOperation<N, A>>;

crate::operation!(
    pub struct AbsWrappedOperation<console::prelude::AbsWrapped, circuit::prelude::AbsWrapped, abs_wrapped, "abs.w"> {
        I8 => I8,
        I16 => I16,
        I32 => I32,
        I64 => I64,
        I128 => I128,
    }
);

/// Adds `first` with `second`, storing the outcome in `destination`.
pub type Add<N, A> = BinaryLiteral<N, A, AddOperation<N, A>>;

crate::operation!(
    pub struct AddOperation<core::ops::Add, core::ops::Add, add, "add"> {
        (Field, Field) => Field,
        (Group, Group) => Group,
        (I8, I8) => I8 ("ensure overflows halt"),
        (I16, I16) => I16 ("ensure overflows halt"),
        (I32, I32) => I32 ("ensure overflows halt"),
        (I64, I64) => I64 ("ensure overflows halt"),
        (I128, I128) => I128 ("ensure overflows halt"),
        (U8, U8) => U8 ("ensure overflows halt"),
        (U16, U16) => U16 ("ensure overflows halt"),
        (U32, U32) => U32 ("ensure overflows halt"),
        (U64, U64) => U64 ("ensure overflows halt"),
        (U128, U128) => U128 ("ensure overflows halt"),
        (Scalar, Scalar) => Scalar,
    }
);

/// Adds `first` with `second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
pub type AddWrapped<N, A> = BinaryLiteral<N, A, AddWrappedOperation<N, A>>;

crate::operation!(
    pub struct AddWrappedOperation<console::prelude::AddWrapped, circuit::prelude::AddWrapped, add_wrapped, "add.w"> {
        (I8, I8) => I8,
        (I16, I16) => I16,
        (I32, I32) => I32,
        (I64, I64) => I64,
        (I128, I128) => I128,
        (U8, U8) => U8,
        (U16, U16) => U16,
        (U32, U32) => U32,
        (U64, U64) => U64,
        (U128, U128) => U128,
    }
);

/// Performs a bitwise `and` on `first` and `second`, storing the outcome in `destination`.
pub type And<N, A> = BinaryLiteral<N, A, AndOperation<N, A>>;

crate::operation!(
    pub struct AndOperation<core::ops::BitAnd, core::ops::BitAnd, bitand, "and"> {
        (Boolean, Boolean) => Boolean,
        (I8, I8) => I8,
        (I16, I16) => I16,
        (I32, I32) => I32,
        (I64, I64) => I64,
        (I128, I128) => I128,
        (U8, U8) => U8,
        (U16, U16) => U16,
        (U32, U32) => U32,
        (U64, U64) => U64,
        (U128, U128) => U128,
    }
);

// /// Divides `first` by `second`, storing the outcome in `destination`.
// pub type Div<N, A> = BinaryLiteral<N, A, DivOperation<N, A>>;
//
// crate::operation!(
//     pub struct DivOperation<core::ops::Div, core::ops::Div, div, "div"> {
//         (Field, Field) => Field,
//         (I8, I8) => I8 ("ensure overflows halt", "ensure divide by zero halts"),
//         (I16, I16) => I16 ("ensure overflows halt", "ensure divide by zero halts"),
//         (I32, I32) => I32 ("ensure overflows halt", "ensure divide by zero halts"),
//         (I64, I64) => I64 ("ensure overflows halt", "ensure divide by zero halts"),
//         (I128, I128) => I128 ("ensure overflows halt", "ensure divide by zero halts"),
//         (U8, U8) => U8 ("ensure overflows halt", "ensure divide by zero halts"),
//         (U16, U16) => U16 ("ensure overflows halt", "ensure divide by zero halts"),
//         (U32, U32) => U32 ("ensure overflows halt", "ensure divide by zero halts"),
//         (U64, U64) => U64 ("ensure overflows halt", "ensure divide by zero halts"),
//         (U128, U128) => U128 ("ensure overflows halt", "ensure divide by zero halts"),
//         (Scalar, Scalar) => Scalar,
//     }
// );

// /// Divides `first` by `second`, wrapping around at the boundary of the type, storing the outcome in `destination`.
// pub type DivWrapped<N, A> = BinaryLiteral<N, A, DivWrappedOperation<N, A>>;
//
// crate::operation!(
//     pub struct DivWrappedOperation<console::prelude::DivWrapped, circuit::prelude::DivWrapped, div_wrapped, "div.w"> {
//         (I8, I8) => I8 ("ensure divide by zero halts"),
//         (I16, I16) => I16 ("ensure divide by zero halts"),
//         (I32, I32) => I32 ("ensure divide by zero halts"),
//         (I64, I64) => I64 ("ensure divide by zero halts"),
//         (I128, I128) => I128 ("ensure divide by zero halts"),
//         (U8, U8) => U8 ("ensure divide by zero halts"),
//         (U16, U16) => U16 ("ensure divide by zero halts"),
//         (U32, U32) => U32 ("ensure divide by zero halts"),
//         (U64, U64) => U64 ("ensure divide by zero halts"),
//         (U128, U128) => U128 ("ensure divide by zero halts"),
//     }
// );

/// Doubles `first`, storing the outcome in `destination`.
pub type Double<N, A> = UnaryLiteral<N, A, DoubleOperation<N, A>>;

crate::operation!(
    pub struct DoubleOperation<console::prelude::Double, circuit::prelude::Double, double, "double"> {
        Field => Field,
        Group => Group,
    }
);

/// Computes whether `first` is greater than `second` as a boolean, storing the outcome in `destination`.
pub type GreaterThan<N, A> = BinaryLiteral<N, A, GreaterThanOperation<N, A>>;

crate::operation!(
    pub struct GreaterThanOperation<console::prelude::Compare, circuit::prelude::Compare, is_greater_than, "gt"> {
        (Address, Address) => Boolean,
        (Field, Field) => Boolean,
        (I8, I8) => Boolean,
        (I16, I16) => Boolean,
        (I32, I32) => Boolean,
        (I64, I64) => Boolean,
        (I128, I128) => Boolean,
        (U8, U8) => Boolean,
        (U16, U16) => Boolean,
        (U32, U32) => Boolean,
        (U64, U64) => Boolean,
        (U128, U128) => Boolean,
        (Scalar, Scalar) => Boolean,
    }
);

/// Computes whether `first` is greater than or equal to `second` as a boolean, storing the outcome in `destination`.
pub type GreaterThanOrEqual<N, A> = BinaryLiteral<N, A, GreaterThanOrEqualOperation<N, A>>;

crate::operation!(
    pub struct GreaterThanOrEqualOperation<console::prelude::Compare, circuit::prelude::Compare, is_greater_than_or_equal, "gte"> {
        (Address, Address) => Boolean,
        (Field, Field) => Boolean,
        (I8, I8) => Boolean,
        (I16, I16) => Boolean,
        (I32, I32) => Boolean,
        (I64, I64) => Boolean,
        (I128, I128) => Boolean,
        (U8, U8) => Boolean,
        (U16, U16) => Boolean,
        (U32, U32) => Boolean,
        (U64, U64) => Boolean,
        (U128, U128) => Boolean,
        (Scalar, Scalar) => Boolean,
    }
);

/// Computes whether `first` equals `second` as a boolean, storing the outcome in `destination`.
pub type IsEqual<N, A> = BinaryLiteral<N, A, IsEqualOperation<N, A>>;

crate::operation!(
    pub struct IsEqualOperation<console::prelude::Equal, circuit::prelude::Equal, is_equal, "is.eq"> {
        (Address, Address) => Boolean,
        (Boolean, Boolean) => Boolean,
        (Field, Field) => Boolean,
        (Group, Group) => Boolean,
        (I8, I8) => Boolean,
        (I16, I16) => Boolean,
        (I32, I32) => Boolean,
        (I64, I64) => Boolean,
        (I128, I128) => Boolean,
        (U8, U8) => Boolean,
        (U16, U16) => Boolean,
        (U32, U32) => Boolean,
        (U64, U64) => Boolean,
        (U128, U128) => Boolean,
        (Scalar, Scalar) => Boolean,
        // (StringType, StringType) => Boolean,
    }
);

/// Computes whether `first` does **not** equals `second` as a boolean, storing the outcome in `destination`.
pub type IsNotEqual<N, A> = BinaryLiteral<N, A, IsNotEqualOperation<N, A>>;

crate::operation!(
    pub struct IsNotEqualOperation<console::prelude::Equal, circuit::prelude::Equal, is_not_equal, "is.neq"> {
        (Address, Address) => Boolean,
        (Boolean, Boolean) => Boolean,
        (Field, Field) => Boolean,
        (Group, Group) => Boolean,
        (I8, I8) => Boolean,
        (I16, I16) => Boolean,
        (I32, I32) => Boolean,
        (I64, I64) => Boolean,
        (I128, I128) => Boolean,
        (U8, U8) => Boolean,
        (U16, U16) => Boolean,
        (U32, U32) => Boolean,
        (U64, U64) => Boolean,
        (U128, U128) => Boolean,
        (Scalar, Scalar) => Boolean,
        // (StringType, StringType) => Boolean,
    }
);

/// Computes the multiplicative inverse of `first`, storing the outcome in `destination`.
pub type Inv<N, A> = UnaryLiteral<N, A, InvOperation<N, A>>;

crate::operation!(
    pub struct InvOperation<console::prelude::Inverse, circuit::prelude::Inverse, inverse?, "inv"> {
        Field => Field ("ensure inverse of zero halts"),
    }
);

/// Computes whether `first` is less than `second` as a boolean, storing the outcome in `destination`.
pub type LessThan<N, A> = BinaryLiteral<N, A, LessThanOperation<N, A>>;

crate::operation!(
    pub struct LessThanOperation<console::prelude::Compare, circuit::prelude::Compare, is_less_than, "lt"> {
        (Address, Address) => Boolean,
        (Field, Field) => Boolean,
        (I8, I8) => Boolean,
        (I16, I16) => Boolean,
        (I32, I32) => Boolean,
        (I64, I64) => Boolean,
        (I128, I128) => Boolean,
        (U8, U8) => Boolean,
        (U16, U16) => Boolean,
        (U32, U32) => Boolean,
        (U64, U64) => Boolean,
        (U128, U128) => Boolean,
        (Scalar, Scalar) => Boolean,
    }
);

/// Computes whether `first` is less than or equal to `second` as a boolean, storing the outcome in `destination`.
pub type LessThanOrEqual<N, A> = BinaryLiteral<N, A, LessThanOrEqualOperation<N, A>>;

crate::operation!(
    pub struct LessThanOrEqualOperation<console::prelude::Compare, circuit::prelude::Compare, is_less_than_or_equal, "lte"> {
        (Address, Address) => Boolean,
        (Field, Field) => Boolean,
        (I8, I8) => Boolean,
        (I16, I16) => Boolean,
        (I32, I32) => Boolean,
        (I64, I64) => Boolean,
        (I128, I128) => Boolean,
        (U8, U8) => Boolean,
        (U16, U16) => Boolean,
        (U32, U32) => Boolean,
        (U64, U64) => Boolean,
        (U128, U128) => Boolean,
        (Scalar, Scalar) => Boolean,
    }
);

/// Multiplies `first` and `second`, storing the outcome in `destination`.
pub type Mul<N, A> = BinaryLiteral<N, A, MulOperation<N, A>>;

crate::operation!(
    pub struct MulOperation<core::ops::Mul, core::ops::Mul, mul, "mul"> {
        (Field, Field) => Field,
        (Group, Scalar) => Group,
        (Scalar, Group) => Group,
        (I8, I8) => I8 ("ensure overflows halt"),
        (I16, I16) => I16 ("ensure overflows halt"),
        (I32, I32) => I32 ("ensure overflows halt"),
        (I64, I64) => I64 ("ensure overflows halt"),
        (I128, I128) => I128 ("ensure overflows halt"),
        (U8, U8) => U8 ("ensure overflows halt"),
        (U16, U16) => U16 ("ensure overflows halt"),
        (U32, U32) => U32 ("ensure overflows halt"),
        (U64, U64) => U64 ("ensure overflows halt"),
        (U128, U128) => U128 ("ensure overflows halt"),
        // (Scalar, Scalar) => Scalar,
    }
);

/// Multiplies `first` and `second`, wrapping around at the boundary of the type, storing the outcome in `destination`.
pub type MulWrapped<N, A> = BinaryLiteral<N, A, MulWrappedOperation<N, A>>;

crate::operation!(
    pub struct MulWrappedOperation<console::prelude::MulWrapped, circuit::prelude::MulWrapped, mul_wrapped, "mul.w"> {
        (I8, I8) => I8,
        (I16, I16) => I16,
        (I32, I32) => I32,
        (I64, I64) => I64,
        (I128, I128) => I128,
        (U8, U8) => U8,
        (U16, U16) => U16,
        (U32, U32) => U32,
        (U64, U64) => U64,
        (U128, U128) => U128,
    }
);

/// Returns `false` if `first` and `second` are `true`, storing the outcome in `destination`.
pub type Nand<N, A> = BinaryLiteral<N, A, NandOperation<N, A>>;

crate::operation!(
    pub struct NandOperation<console::prelude::Nand, circuit::prelude::Nand, nand, "nand"> {
        (Boolean, Boolean) => Boolean,
    }
);

/// Negates `first`, storing the outcome in `destination`.
pub type Neg<N, A> = UnaryLiteral<N, A, NegOperation<N, A>>;

crate::operation!(
    pub struct NegOperation<core::ops::Neg, core::ops::Neg, neg, "neg"> {
        Field => Field,
        Group => Group,
        I8 => I8 ("ensure overflows halt"),
        I16 => I16 ("ensure overflows halt"),
        I32 => I32 ("ensure overflows halt"),
        I64 => I64 ("ensure overflows halt"),
        I128 => I128 ("ensure overflows halt"),
    }
);

/// Returns `true` if neither `first` nor `second` is `true`, storing the outcome in `destination`.
pub type Nor<N, A> = BinaryLiteral<N, A, NorOperation<N, A>>;

crate::operation!(
    pub struct NorOperation<console::prelude::Nor, circuit::prelude::Nor, nor, "nor"> {
        (Boolean, Boolean) => Boolean,
    }
);

/// Flips each bit in the representation of `first`, storing the outcome in `destination`.
pub type Not<N, A> = UnaryLiteral<N, A, NotOperation<N, A>>;

crate::operation!(
    pub struct NotOperation<core::ops::Not, core::ops::Not, not, "not"> {
        Boolean => Boolean,
        I8 => I8,
        I16 => I16,
        I32 => I32,
        I64 => I64,
        I128 => I128,
        U8 => U8,
        U16 => U16,
        U32 => U32,
        U64 => U64,
        U128 => U128,
    }
);

/// Performs a bitwise `or` on `first` and `second`, storing the outcome in `destination`.
pub type Or<N, A> = BinaryLiteral<N, A, OrOperation<N, A>>;

crate::operation!(
    pub struct OrOperation<core::ops::BitOr, core::ops::BitOr, bitor, "or"> {
        (Boolean, Boolean) => Boolean,
        (I8, I8) => I8,
        (I16, I16) => I16,
        (I32, I32) => I32,
        (I64, I64) => I64,
        (I128, I128) => I128,
        (U8, U8) => U8,
        (U16, U16) => U16,
        (U32, U32) => U32,
        (U64, U64) => U64,
        (U128, U128) => U128,
    }
);

/// Raises `first` to the power of `second`, storing the outcome in `destination`.
pub type Pow<N, A> = BinaryLiteral<N, A, PowOperation<N, A>>;

crate::operation!(
    pub struct PowOperation<num_traits::Pow, num_traits::Pow, pow, "pow"> {
        (Field, Field) => Field,
        (I8, U8) => I8 ("ensure exponentiation overflows halt"),
        (I8, U16) => I8 ("ensure exponentiation overflows halt"),
        (I8, U32) => I8 ("ensure exponentiation overflows halt"),
        (I16, U8) => I16 ("ensure exponentiation overflows halt"),
        (I16, U16) => I16 ("ensure exponentiation overflows halt"),
        (I16, U32) => I16 ("ensure exponentiation overflows halt"),
        (I32, U8) => I32 ("ensure exponentiation overflows halt"),
        (I32, U16) => I32 ("ensure exponentiation overflows halt"),
        (I32, U32) => I32 ("ensure exponentiation overflows halt"),
        (I64, U8) => I64 ("ensure exponentiation overflows halt"),
        (I64, U16) => I64 ("ensure exponentiation overflows halt"),
        (I64, U32) => I64 ("ensure exponentiation overflows halt"),
        (I128, U8) => I128 ("ensure exponentiation overflows halt"),
        (I128, U16) => I128 ("ensure exponentiation overflows halt"),
        (I128, U32) => I128 ("ensure exponentiation overflows halt"),
        (U8, U8) => U8 ("ensure exponentiation overflows halt"),
        (U8, U16) => U8 ("ensure exponentiation overflows halt"),
        (U8, U32) => U8 ("ensure exponentiation overflows halt"),
        (U16, U8) => U16 ("ensure exponentiation overflows halt"),
        (U16, U16) => U16 ("ensure exponentiation overflows halt"),
        (U16, U32) => U16 ("ensure exponentiation overflows halt"),
        (U32, U8) => U32 ("ensure exponentiation overflows halt"),
        (U32, U16) => U32 ("ensure exponentiation overflows halt"),
        (U32, U32) => U32 ("ensure exponentiation overflows halt"),
        (U64, U8) => U64 ("ensure exponentiation overflows halt"),
        (U64, U16) => U64 ("ensure exponentiation overflows halt"),
        (U64, U32) => U64 ("ensure exponentiation overflows halt"),
        (U128, U8) => U128 ("ensure exponentiation overflows halt"),
        (U128, U16) => U128 ("ensure exponentiation overflows halt"),
        (U128, U32) => U128 ("ensure exponentiation overflows halt"),
    }
);

/// Raises `first` to the power of `second`, wrapping around at the boundary of the type, storing the outcome in `destination`.
pub type PowWrapped<N, A> = BinaryLiteral<N, A, PowWrappedOperation<N, A>>;

crate::operation!(
    pub struct PowWrappedOperation<console::prelude::PowWrapped, circuit::prelude::PowWrapped, pow_wrapped, "pow.w"> {
        (I8, U8) => I8,
        (I8, U16) => I8,
        (I8, U32) => I8,
        (I16, U8) => I16,
        (I16, U16) => I16,
        (I16, U32) => I16,
        (I32, U8) => I32,
        (I32, U16) => I32,
        (I32, U32) => I32,
        (I64, U8) => I64,
        (I64, U16) => I64,
        (I64, U32) => I64,
        (I128, U8) => I128,
        (I128, U16) => I128,
        (I128, U32) => I128,
        (U8, U8) => U8,
        (U8, U16) => U8,
        (U8, U32) => U8,
        (U16, U8) => U16,
        (U16, U16) => U16,
        (U16, U32) => U16,
        (U32, U8) => U32,
        (U32, U16) => U32,
        (U32, U32) => U32,
        (U64, U8) => U64,
        (U64, U16) => U64,
        (U64, U32) => U64,
        (U128, U8) => U128,
        (U128, U16) => U128,
        (U128, U32) => U128,
    }
);

/// Shifts `first` left by `second` bits, storing the outcome in `destination`.
pub type Shl<N, A> = BinaryLiteral<N, A, ShlOperation<N, A>>;

crate::operation!(
    pub struct ShlOperation<console::prelude::ShlChecked, circuit::prelude::ShlChecked, shl_checked, "shl"> {
        (I8, U8) => I8 ("ensure shifting past boundary halts"),
        (I8, U16) => I8 ("ensure shifting past boundary halts"),
        (I8, U32) => I8 ("ensure shifting past boundary halts"),
        (I16, U8) => I16 ("ensure shifting past boundary halts"),
        (I16, U16) => I16 ("ensure shifting past boundary halts"),
        (I16, U32) => I16 ("ensure shifting past boundary halts"),
        (I32, U8) => I32 ("ensure shifting past boundary halts"),
        (I32, U16) => I32 ("ensure shifting past boundary halts"),
        (I32, U32) => I32 ("ensure shifting past boundary halts"),
        (I64, U8) => I64 ("ensure shifting past boundary halts"),
        (I64, U16) => I64 ("ensure shifting past boundary halts"),
        (I64, U32) => I64 ("ensure shifting past boundary halts"),
        (I128, U8) => I128 ("ensure shifting past boundary halts"),
        (I128, U16) => I128 ("ensure shifting past boundary halts"),
        (I128, U32) => I128 ("ensure shifting past boundary halts"),
        (U8, U8) => U8 ("ensure shifting past boundary halts"),
        (U8, U16) => U8 ("ensure shifting past boundary halts"),
        (U8, U32) => U8 ("ensure shifting past boundary halts"),
        (U16, U8) => U16 ("ensure shifting past boundary halts"),
        (U16, U16) => U16 ("ensure shifting past boundary halts"),
        (U16, U32) => U16 ("ensure shifting past boundary halts"),
        (U32, U8) => U32 ("ensure shifting past boundary halts"),
        (U32, U16) => U32 ("ensure shifting past boundary halts"),
        (U32, U32) => U32 ("ensure shifting past boundary halts"),
        (U64, U8) => U64 ("ensure shifting past boundary halts"),
        (U64, U16) => U64 ("ensure shifting past boundary halts"),
        (U64, U32) => U64 ("ensure shifting past boundary halts"),
        (U128, U8) => U128 ("ensure shifting past boundary halts"),
        (U128, U16) => U128 ("ensure shifting past boundary halts"),
        (U128, U32) => U128 ("ensure shifting past boundary halts"),
    }
);

/// Shifts `first` left by `second` bits, continuing past the boundary of the type, storing the outcome in `destination`.
pub type ShlWrapped<N, A> = BinaryLiteral<N, A, ShlWrappedOperation<N, A>>;

crate::operation!(
    pub struct ShlWrappedOperation<console::prelude::ShlWrapped, circuit::prelude::ShlWrapped, shl_wrapped, "shl.w"> {
        (I8, U8) => I8,
        (I8, U16) => I8,
        (I8, U32) => I8,
        (I16, U8) => I16,
        (I16, U16) => I16,
        (I16, U32) => I16,
        (I32, U8) => I32,
        (I32, U16) => I32,
        (I32, U32) => I32,
        (I64, U8) => I64,
        (I64, U16) => I64,
        (I64, U32) => I64,
        (I128, U8) => I128,
        (I128, U16) => I128,
        (I128, U32) => I128,
        (U8, U8) => U8,
        (U8, U16) => U8,
        (U8, U32) => U8,
        (U16, U8) => U16,
        (U16, U16) => U16,
        (U16, U32) => U16,
        (U32, U8) => U32,
        (U32, U16) => U32,
        (U32, U32) => U32,
        (U64, U8) => U64,
        (U64, U16) => U64,
        (U64, U32) => U64,
        (U128, U8) => U128,
        (U128, U16) => U128,
        (U128, U32) => U128,
    }
);

/// Shifts `first` right by `second` bits, storing the outcome in `destination`.
pub type Shr<N, A> = BinaryLiteral<N, A, ShrOperation<N, A>>;

crate::operation!(
    pub struct ShrOperation<console::prelude::ShrChecked, circuit::prelude::ShrChecked, shr_checked, "shr"> {
        (I8, U8) => I8 ("ensure shifting past boundary halts"),
        (I8, U16) => I8 ("ensure shifting past boundary halts"),
        (I8, U32) => I8 ("ensure shifting past boundary halts"),
        (I16, U8) => I16 ("ensure shifting past boundary halts"),
        (I16, U16) => I16 ("ensure shifting past boundary halts"),
        (I16, U32) => I16 ("ensure shifting past boundary halts"),
        (I32, U8) => I32 ("ensure shifting past boundary halts"),
        (I32, U16) => I32 ("ensure shifting past boundary halts"),
        (I32, U32) => I32 ("ensure shifting past boundary halts"),
        (I64, U8) => I64 ("ensure shifting past boundary halts"),
        (I64, U16) => I64 ("ensure shifting past boundary halts"),
        (I64, U32) => I64 ("ensure shifting past boundary halts"),
        (I128, U8) => I128 ("ensure shifting past boundary halts"),
        (I128, U16) => I128 ("ensure shifting past boundary halts"),
        (I128, U32) => I128 ("ensure shifting past boundary halts"),
        (U8, U8) => U8 ("ensure shifting past boundary halts"),
        (U8, U16) => U8 ("ensure shifting past boundary halts"),
        (U8, U32) => U8 ("ensure shifting past boundary halts"),
        (U16, U8) => U16 ("ensure shifting past boundary halts"),
        (U16, U16) => U16 ("ensure shifting past boundary halts"),
        (U16, U32) => U16 ("ensure shifting past boundary halts"),
        (U32, U8) => U32 ("ensure shifting past boundary halts"),
        (U32, U16) => U32 ("ensure shifting past boundary halts"),
        (U32, U32) => U32 ("ensure shifting past boundary halts"),
        (U64, U8) => U64 ("ensure shifting past boundary halts"),
        (U64, U16) => U64 ("ensure shifting past boundary halts"),
        (U64, U32) => U64 ("ensure shifting past boundary halts"),
        (U128, U8) => U128 ("ensure shifting past boundary halts"),
        (U128, U16) => U128 ("ensure shifting past boundary halts"),
        (U128, U32) => U128 ("ensure shifting past boundary halts"),
    }
);

/// Shifts `first` right by `second` bits, continuing past the boundary of the type, storing the outcome in `destination`.
pub type ShrWrapped<N, A> = BinaryLiteral<N, A, ShrWrappedOperation<N, A>>;

crate::operation!(
    pub struct ShrWrappedOperation<console::prelude::ShrWrapped, circuit::prelude::ShrWrapped, shr_wrapped, "shr.w"> {
        (I8, U8) => I8,
        (I8, U16) => I8,
        (I8, U32) => I8,
        (I16, U8) => I16,
        (I16, U16) => I16,
        (I16, U32) => I16,
        (I32, U8) => I32,
        (I32, U16) => I32,
        (I32, U32) => I32,
        (I64, U8) => I64,
        (I64, U16) => I64,
        (I64, U32) => I64,
        (I128, U8) => I128,
        (I128, U16) => I128,
        (I128, U32) => I128,
        (U8, U8) => U8,
        (U8, U16) => U8,
        (U8, U32) => U8,
        (U16, U8) => U16,
        (U16, U16) => U16,
        (U16, U32) => U16,
        (U32, U8) => U32,
        (U32, U16) => U32,
        (U32, U32) => U32,
        (U64, U8) => U64,
        (U64, U16) => U64,
        (U64, U32) => U64,
        (U128, U8) => U128,
        (U128, U16) => U128,
        (U128, U32) => U128,
    }
);

/// Squares `first`, storing the outcome in `destination`.
pub type Square<N, A> = UnaryLiteral<N, A, SquareOperation<N, A>>;

crate::operation!(
    pub struct SquareOperation<console::prelude::Square, circuit::prelude::Square, square, "square"> {
        Field => Field,
    }
);

/// Computes the square root of `first`, storing the outcome in `destination`.
pub type SquareRoot<N, A> = UnaryLiteral<N, A, SquareRootOperation<N, A>>;

crate::operation!(
    pub struct SquareRootOperation<console::prelude::SquareRoot, circuit::prelude::SquareRoot, square_root?, "sqrt"> {
        Field => Field ("ensure quadratic nonresidues halt"),
    }
);

/// Computes `first - second`, storing the outcome in `destination`.
pub type Sub<N, A> = BinaryLiteral<N, A, SubOperation<N, A>>;

crate::operation!(
    pub struct SubOperation<core::ops::Sub, core::ops::Sub, sub, "sub"> {
        (Field, Field) => Field,
        (Group, Group) => Group,
        (I8, I8) => I8 ("ensure overflows halt"),
        (I16, I16) => I16 ("ensure overflows halt"),
        (I32, I32) => I32 ("ensure overflows halt"),
        (I64, I64) => I64 ("ensure overflows halt"),
        (I128, I128) => I128 ("ensure overflows halt"),
        (U8, U8) => U8 ("ensure overflows halt"),
        (U16, U16) => U16 ("ensure overflows halt"),
        (U32, U32) => U32 ("ensure overflows halt"),
        (U64, U64) => U64 ("ensure overflows halt"),
        (U128, U128) => U128 ("ensure overflows halt"),
        // (Scalar, Scalar) => Scalar,
    }
);

/// Computes `first - second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
pub type SubWrapped<N, A> = BinaryLiteral<N, A, SubWrappedOperation<N, A>>;

crate::operation!(
    pub struct SubWrappedOperation<console::prelude::SubWrapped, circuit::prelude::SubWrapped, sub_wrapped, "sub.w"> {
        (I8, I8) => I8,
        (I16, I16) => I16,
        (I32, I32) => I32,
        (I64, I64) => I64,
        (I128, I128) => I128,
        (U8, U8) => U8,
        (U16, U16) => U16,
        (U32, U32) => U32,
        (U64, U64) => U64,
        (U128, U128) => U128,
    }
);

/// Performs a bitwise `xor` on `first` and `second`, storing the outcome in `destination`.
pub type Xor<N, A> = BinaryLiteral<N, A, XorOperation<N, A>>;

crate::operation!(
    pub struct XorOperation<core::ops::BitXor, core::ops::BitXor, bitxor, "xor"> {
        (Boolean, Boolean) => Boolean,
        (I8, I8) => I8,
        (I16, I16) => I16,
        (I32, I32) => I32,
        (I64, I64) => I64,
        (I128, I128) => I128,
        (U8, U8) => U8,
        (U16, U16) => U16,
        (U32, U32) => U32,
        (U64, U64) => U64,
        (U128, U128) => U128,
    }
);
