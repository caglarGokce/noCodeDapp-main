use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum Errors {
  /// Invalid Instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,

  #[error("arithmetic error")]
  ArithmeticError,
}

impl From<Errors> for ProgramError {
  fn from(e: Errors) -> Self {
    ProgramError::Custom(e as u32)
  }
}
