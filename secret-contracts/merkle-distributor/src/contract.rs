use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128,
};
use secret_toolkit::snip20;
use web3::signing::keccak256;

use crate::merkle_proof::{encode_as_merkle_leaf, verify_proof};
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};
use byteorder::{BigEndian, ByteOrder};
use hex::FromHex;
use secret_toolkit::storage::{TypedStore, TypedStoreMut};

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
            address,
            amount,
            proof,
        } => claim(deps, env, index, address, amount, proof),
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
    address: HumanAddr,
    amount: u128,
    proof: Vec<String>,
) -> StdResult<HandleResponse> {
    let is_claimed = is_claimed(deps, index)?;
    if is_claimed {
        return Err(StdError::generic_err("drop already claimed"));
    }

    let state = config_read(&mut deps.storage).load()?;

    let hex_address = deps.api.canonical_address(&address)?;
    let proof_bytes = proof
        .into_iter()
        .map(|p| <[u8; 32]>::from_hex(p).unwrap())
        .collect();
    let root_bytes = <[u8; 32]>::from_hex(state.merkle_root).unwrap();
    let leaf_bytes = keccak256(&encode_as_merkle_leaf(
        index,
        hex_address.as_slice(),
        amount,
    ));
    let valid_proof = verify_proof(proof_bytes, root_bytes, leaf_bytes);
    if !valid_proof {
        return Err(StdError::generic_err("invalid proof"));
    }

    set_claimed(deps, index);

    Ok(HandleResponse {
        messages: vec![snip20::transfer_msg(
            address,
            Uint128(amount),
            None,
            1,
            state.token_hash,
            state.token_addr,
        )?],
        log: vec![],
        data: None,
    })
}

fn set_claimed<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>, index: u128) {
    let mut claimed_bitmap = TypedStoreMut::attach(&mut deps.storage);

    let mut claimed_word_index = [0u8; 16];
    BigEndian::write_u128(&mut claimed_word_index, index / 128);

    let mut claimed_word: u128 = claimed_bitmap.load(&claimed_word_index).unwrap_or(0);
    let claimed_bit_index = index % 128;
    claimed_word = claimed_word | (1 << claimed_bit_index);

    claimed_bitmap
        .store(&claimed_word_index, &claimed_word)
        .unwrap();
}

pub fn is_claimed<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    index: u128,
) -> StdResult<bool> {
    let claimed_bitmap = TypedStore::attach(&deps.storage);

    let mut claimed_word_index = [0u8; 16];
    BigEndian::write_u128(&mut claimed_word_index, index / 128);

    let claimed_word: u128 = claimed_bitmap.load(&claimed_word_index).unwrap_or(0);
    let claimed_bit_index = index % 128;
    let mask = (1 << claimed_bit_index);

    Ok(claimed_word & mask == mask)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};
}
