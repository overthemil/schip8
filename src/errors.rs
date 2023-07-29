use thiserror::Error;

/// The error types used by the interpreter
#[derive(Error, Debug)]
pub enum ChipError {
    /// This error occurs when attempting to access out of bounds memory
    #[error("The address {address:?} is out of bounds. The limits are: (0, {limit:?})")]
    AddressOutOfBounds { address: usize, limit: usize },

    /// Thrown by the CPU when attempting to pop() from an empty stack
    #[error("Attempted to remove item from an empty stack")]
    StackUnderflow(),

    /// Thrown by the CPU attempting to push() to a full stack
    #[error("Stack size limit exceeded. Maximum size: {0}")]
    StackOverflow(usize),

    /// Thrown by the CPU when attempting to execute an unknown opcode
    #[error("The opcode {:#06x} is not implemented", .opcode)]
    OpcodeNotImplemented { opcode: u16 },
}
