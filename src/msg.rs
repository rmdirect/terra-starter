use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std:: Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub date: i32, // Changed from 'count' to 'date'
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // We won't need Increment and Reset from the template so we can remove them
    //Increment {},
    //Reset { date: i32 }, //Changed from 'count' to 'date'

    UpsertScore { score: u16 }, //This will add or update scores
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetDate returns the current 'date' as a json-encoded number
    GetDate {}, // Changed from 'GetCount' to 'GetDate'
    // Like with GetDate, we declare an enum for scores too
    GetScores {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DateResponse {
    pub date: i32, // Changed from 'pub count: i32' to 'pub date: i32; also above changed CountResponse to DateResponse'
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ScoreResponse{
    pub scores: Vec<(Addr, u16)>,
}

