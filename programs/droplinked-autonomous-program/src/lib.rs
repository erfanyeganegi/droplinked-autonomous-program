use anchor_lang::prelude::*;

declare_id!("Du3qfvSDkDq92p9dYV6XhRPW6TRRtHbGnH3tiNfJjYWp");

mod processor;
pub mod state;

use processor::*;
use state::*;

#[program]
pub mod droplinked_autonomous_program {
    use super::*;
}
