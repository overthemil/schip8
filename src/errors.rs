use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChipError {
    #[error("The address {address:?} is out of bounds. The limits are: (0, {limit:?})")]
    AddressOutOfBounds{ address: usize, limit: usize },

    #[error("Attempted to remove item from an empty stack")]
    StackUnderflow(),

    #[error("Stack size limit exceeded. Maximum size: {0}")]
    StackOverflow(usize),

    #[error("The opcode {:#06x} is not implemented", .opcode)]
    OpcodeNotImplemented{ opcode: u16 },
}

