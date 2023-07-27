use thiserror::Error;
use crate::MEMORY_SIZE;
use crate::cpu::STACK_SIZE;

#[derive(Error, Debug)]
pub enum ChipError {
    #[error("The address {0} is out of bounds. The limits are: (0, {})", MEMORY_SIZE)]
    AddressOutOfBounds(usize),

    #[error("Attempted to remove item from an empty stack")]
    StackUnderflow(),

    #[error("Stack size limit exceeded. Maximum size: {}", STACK_SIZE)]
    StackOverflow(),
}

