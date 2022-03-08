use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("You cannot withdraw UST when you have not deposited UST")]
    CannotWithdrawWithoutDeposit {},

    #[error("You cannot withdraw UST when your balance is zero")]
    CannotWithdrawBalanceZero {},

    #[error("You cannot withdraw an amount of UST greater than your balance")]
    CannotWithdrawGreaterThanBalance {},

    #[error("You cannot claim rewards without depositing UST")]
    CannotClaimWithoutDeposit {},
}
