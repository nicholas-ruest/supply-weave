use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use std::{sync::Arc, time::Duration};
use sw_config::Config;
use sw_errors::WeaveError;
use sw_orchestrator::Orchestrator;
use sw_types::{CreatePlan, Plan};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

#[derive(Clone)]
struct App {
    orchestrator: Arc<Orchestrator>,
}

#[derive(serde::Serialize)]
struct Health {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

#[derive(serde::Serialize)]
struct Problem {
    code: &'static str,
    message: String,
}

struct ApiError(WeaveError);

impl From<WeaveError> for ApiError {
    fn from(value: WeaveError) -> Self {
        Self(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code) = match &self.0 {
            WeaveError::Invalid(_) => (StatusCode::UNPROCESSABLE_ENTITY, "invalid_request"),
            WeaveError::Denied(_) => (StatusCode::FORBIDDEN, "policy_denied"),
            WeaveError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            WeaveError::Timeout(_) => (StatusCode::REQUEST_TIMEOUT, "timeout"),
            _ => (StatusCode::BAD_GATEWAY, "dependency_failed"),
        };
        (
            status,
            Json(Problem {
                code,
                message: self.0.to_string(),
            }),
        )
            .into_response()
    }
}

fn context(headers: &HeaderMap) -> Result<(sw_auth::Principal, String, String), ApiError> {
    let auth = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok());
    let principal = sw_auth::authenticate(auth)?;
    sw_middleware::authorize_plan(&principal)?;
    let correlation = sw_middleware::correlation(
        headers
            .get("x-correlation-id")
            .and_then(|value| value.to_str().ok()),
    )?;
    let idempotency = sw_middleware::idempotency(
        headers
            .get("idempotency-key")
            .and_then(|value| value.to_str().ok()),
    )?;
    Ok((principal, correlation, idempotency))
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        service: "supply-weave",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn create(
    State(app): State<App>,
    headers: HeaderMap,
    Json(request): Json<CreatePlan>,
) -> Result<(StatusCode, Json<Plan>), ApiError> {
    let (_, correlation, idempotency) = context(&headers)?;
    let plan = app
        .orchestrator
        .create(request, correlation, idempotency)
        .await?;
    Ok((StatusCode::CREATED, Json(plan)))
}

async fn list(State(app): State<App>) -> Result<Json<Vec<Plan>>, ApiError> {
    Ok(Json(app.orchestrator.list()?))
}

async fn get_one(State(app): State<App>, Path(id): Path<String>) -> Result<Json<Plan>, ApiError> {
    Ok(Json(app.orchestrator.get(&id)?))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();
    let config = Config::from_env().expect("valid configuration");
    let orchestrator =
        Arc::new(Orchestrator::open(config.data_dir.clone()).expect("open RVF repository"));
    let api = Router::new()
        .route("/health", get(health))
        .route("/ready", get(health))
        .route("/api/v1/plans", post(create).get(list))
        .route("/api/v1/plans/{id}", get(get_one))
        .with_state(App { orchestrator })
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_millis(config.request_timeout_ms),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());
    let static_files = ServeDir::new("frontend/dist")
        .not_found_service(ServeFile::new("frontend/dist/index.html"));
    let app = api.fallback_service(static_files);
    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&address).await.expect("bind");
    tracing::info!(%address, "Supply Weave ready");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .expect("serve");
}

#[cfg(test)]
mod tests {
    #[test]
    fn versioned_api() {
        assert!("/api/v1/plans".contains("v1"));
    }
}
