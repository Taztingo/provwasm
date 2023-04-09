use cosmwasm_std::{Env, MessageInfo};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    msg::InstantiateMsg,
};

use super::default;

pub fn route(
    deps: ProvDepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ProvTxResponse {
    match msg {
        InstantiateMsg::Default { owner, fee } => default::handle(deps, owner, fee),
    }
}
