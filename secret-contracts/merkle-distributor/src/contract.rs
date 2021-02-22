use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, Uint128,
};
use web3::signing::keccak256;

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        token_addr: msg.token_addr,
        token_hash: msg.token_hash,
        merkle_root: msg.merkle_root,
        claimed_bitmap: vec![],
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Claim {
            index,
            hex_address,
            amount,
            proof,
        } => claim(deps, env, index, hex_address, amount, proof),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsClaimed { index } => to_binary(&is_claimed(deps, index)),
    }
}

pub fn claim<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    index: u128,
    hex_address: String,
    amount: u128,
    proof: String,
) -> StdResult<HandleResponse> {
    unimplemented!()
}

fn is_claimed<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    index: u128,
) -> StdResult<bool> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};
}
