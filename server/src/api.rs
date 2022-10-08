
use crate::error::Error;

use fire_api::request::{Request, Method};

use backend::{Ai, Iteration};
// use backend::game::snapshots::Iteration;
// use backend::game::environment::EnvironmentStats;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentStateReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentState {
	pub ai: Ai,
	pub stats: Vec<RunStat>
}

impl Request for CurrentStateReq {
	type Response = CurrentState;
	type Error = Error;

	const PATH: &'static str = "/api/current-state";
	const METHOD: Method = Method::Get;
}