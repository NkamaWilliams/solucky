use anchor_lang::prelude::*;

declare_id!("ByCkjxvutPEQXk1FNB1yXCKWvEUFoT5HpSsxLf957pc");

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

#[program]
pub mod solucky {
    use super::*;
}
