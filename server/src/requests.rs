
use crate::Data;
use crate::error::Error;
use crate::api::{
	CurrentStateReq, CurrentState,
	ChangeAiReq,
	RunReq, RunResp, RunStat,
	ViewIterationsReq, ViewIterations
};

use fire::FireBuilder;
use fire_api::request_handler;

request_handler! {
	async fn current_state(
		_req: CurrentStateReq,
		runner
	) -> Result<CurrentState, Error> {
		let runner = runner.lock().await;

		Ok(CurrentState {
			ai: runner.current_ai(),
			stats: runner.iter().enumerate()
				.map(|(num, iter)| RunStat::from_iteration(num, iter))
				.collect()
		})
	}
}

pub fn add_routes(server: &mut FireBuilder<Data>) {
	server.add_route(current_state);
}