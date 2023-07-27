use thiserror::Error;
use crate::MEMORY_SIZE;

#[derive(Error, Debug)]
pub enum ChipError {
    #[error("The address {0} is out of bounds. The limits are: (0, {})", MEMORY_SIZE)]
    AddressOutOfBounds(usize)
}

