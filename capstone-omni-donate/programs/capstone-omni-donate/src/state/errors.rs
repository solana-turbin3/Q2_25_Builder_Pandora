use anchor_lang::prelude::*;

#[error_code]
pub enum Cases {
    #[msg("Environment is Not in Distress")]
    WEATHERCONDITIONNOTMET,
    #[msg("Campaign has Insufficient Funds to Claim")]
    INSUFFICIENTFUNDS,
    #[msg("Weather Update too Frequent")]
    TIMENOTPASSED,
    #[msg("Campaing Is not Completed")]
    CAMPAIGNCOMPLETED,
    #[msg("Invalid Fee BPS")]
    INVALIDFEEBPS,
    #[msg("You're Not the Admin")]
    NOTADMIN,
    #[msg("You are Not the Oracle")]
    NOTORACLE,
    #[msg("Campaign Cannot Exceed Max Conditions")]
    REACHEDMAXCONDITIONS
}
