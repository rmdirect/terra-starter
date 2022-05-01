#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
// We're adding to_binary, Binary, Deps and StdResult
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
// We're adding InstantiateMsg and QueryMsg and ExecuteMsg
use crate::msg::{DateResponse, InstantiateMsg, ExecuteMsg, QueryMsg, ScoreResponse, MigrateMsg};
// I updated the import since we renamed STATE to STORAGE
use crate::state::{State, STORAGE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:clicker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
  Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {

  // We're storing stuff in a variable called "state" of type "State"
  let state = State {
    date: msg.date,
    owner: info.sender.clone(),
    // Since our state NEEDS to have a scores value, I'm just putting in an empty one
    scores: vec![],
  };

  // We're setting the contract version using a helper function we imported
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  // We're storing state in a special variable called "STATE"
  STORAGE.save(deps.storage, &state)?;

  // Sending a response back to the caller
  Ok(Response::new()
    .add_attribute("method", "instantiate")
    .add_attribute("owner", info.sender)
    .add_attribute("date", msg.date.to_string())
    // Scores is empty so we just send back an empty string as the value
    .add_attribute("scores", "".to_string()))
}

// Here's our execute message handler, we need 'info' as a parameter too
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    // 'score' is being passed in, we'll pass that forward
    ExecuteMsg::UpsertScore { score } => try_upsert_score(deps, info, score),
  }
}

// Here's our main upsert function - it adds a score if the address doesn't exist, or updates it if it does
fn try_upsert_score(
  deps: DepsMut,
  info: MessageInfo,
  score: u16,
) -> Result<Response, ContractError> {
  // Declare all the variables we'll need
  let mut state = STORAGE.load(deps.storage)?;
  let sender = info.sender.clone();
  let scores = &mut state.scores;

  // Iterate (loop) through all scores values and check if the address is the sender
  let index = scores.iter().position(|(s, _)| s == &sender);
  match index {
    Some(i) => {
      scores[i].1 = score;
    },
    None => {
      scores.push((sender.clone(), score));
    }
  }

  STORAGE.save(deps.storage, &state)?;
  Ok(Response::new()
    .add_attribute("method", "upsert")
    .add_attribute("player", info.sender)
    .add_attribute("score", score.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
      QueryMsg::GetDate {} => to_binary(&query_date(deps)?),
      // Match case for the newly added GetScores query
      QueryMsg::GetScores {} => to_binary(&query_scores(deps)?),
  }
}

fn query_date(deps: Deps) -> StdResult<DateResponse> {
  let state = STORAGE.load(deps.storage)?;
  Ok(DateResponse { date: state.date })
}

// Load from storage, return as a vector of (address, score) tuples
fn query_scores(deps: Deps) -> StdResult<ScoreResponse> {
  let state = STORAGE.load(deps.storage)?;
  Ok(ScoreResponse { scores: state.scores })
}