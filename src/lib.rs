// Crate root: re-exports internal modules that implement parsing, model inference logic,
// the adapter trait for inference engines, application state, API models, and routes.
//
// Recent changes (commit):
// - Added unit tests and integration tests with a MockEngine to verify routing and SSE
// - Added helper test utilities under `tests/` for consistent request construction
pub mod parse;
pub mod model_inference;
pub mod engine;
pub mod state;
pub mod models;
pub mod routes;
pub mod engine_mock;

#[cfg(test)]
mod tests {
	use super::*;
	use axum::body::Body;
	use axum::http::Request;
	use tower::ServiceExt;

	#[tokio::test]
	async fn lib_parse_and_router_smoke() {
		// test parse::parse_args minimal behavior
	let json = r#"{"model-name":"mock-model","model-dir":"models/","prompt":"hi","repeat-penalty":1.0,"stop":[]}"#;
		let args = parse::parse_args(json).expect("parse args");
		assert_eq!(args.model_name, "mock-model");

		// test router responds to /models
		let state = state::AppState::new(std::sync::Arc::new(engine_mock::MockEngine::new()));
		let app = routes::router().with_state(state);
		let req = Request::builder().method("GET").uri("/models").body(Body::empty()).unwrap();
		let resp = app.oneshot(req).await.expect("request");
		assert!(resp.status().is_success());
	}
}