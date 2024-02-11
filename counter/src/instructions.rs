use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct AddArgs {
    pub value: u32,
}
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct DedArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(AddArgs), 
    Decrement(DedArgs), 
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let args = AddArgs::try_from_slice(&rest)?;
                Self::Increment(args)
            }
            1 => {
                let args = DedArgs::try_from_slice(&rest)?;
                Self::Decrement(args)
            }
            2 => {
                let args = UpdateArgs::try_from_slice(&rest)?;
                Self::Update(args)
            }
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
