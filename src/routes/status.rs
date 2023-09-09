use axum::{routing::get, Json, Router};
use bson::doc;
use serde::{Deserialize, Serialize};
use tracing::debug;
use utoipa::ToSchema;

use crate::errors::Error;

pub fn create_route() -> Router {
  Router::new().route("/status", get(get_status))
}

#[utoipa::path(
  get,
  path = "/status",
  responses(
    (status = 200, description = "Get server status", body = Status)
  )
)]
async fn get_status() -> Result<Json<Status>, Error> {
  debug!("Returning status");
  Ok(Json(Status {
    status: "ok".to_owned(),
  }))
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Status {
  status: String,
}
