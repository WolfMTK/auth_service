use std::sync::Arc;
use tracing::log::{error, info};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::context::api_version::ApiVersion;
use crate::context::validate::ValidatedRequest;
use crate::model::user::{JsonCreateUser, JsonUser};

use crate::module::{Modules, ModulesExt};

pub async fn create_user(
    _: ApiVersion,
    modules: State<Arc<Modules>>,
    ValidatedRequest(source): ValidatedRequest<JsonCreateUser>,
) -> Result<impl IntoResponse, StatusCode>{
    let resp = modules.user_use_case().create_user(source.into()).await;

    resp.map(|view| {
        info!("Created user: {}", view.id);
        let json: JsonUser = view.into();
        (StatusCode::CREATED, Json(json))
    })
        .map_err(|err| {
            error!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
