use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tracing::log::{error, info};

use crate::context::api_version::ApiVersion;
use crate::context::response_helper::JsonErrorResponse;
use crate::context::validate::ValidatedRequest;
use crate::model::paginate::PaginateQuery;
use crate::model::user::{JsonCreateUser, JsonUpdateUser, JsonUser, JsonUserId, JsonUserList};
use crate::module::{Modules, ModulesExt};

pub async fn create_user(
    _: ApiVersion,
    modules: State<Arc<Modules>>,
    ValidatedRequest(source): ValidatedRequest<JsonCreateUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let resp = modules.user_use_case().create_user(source.into()).await;
    resp.map(|view| {
        info!("Created user: {}", view.id);
        let json: JsonUserId = view.into();
        (StatusCode::CREATED, Json(json))
    })
        .map_err(|err| {
            error!("Unexpected error: {:?}", err);
            if err.to_string() == *"User exists" {
                let json = JsonErrorResponse::new(
                    "invalid_request".to_string(),
                    vec![err.to_string()],
                );
                (StatusCode::BAD_REQUEST, Json(json))
            } else {
                let json = JsonErrorResponse::new(
                    "server_error".to_string(),
                    vec!["INTERNAL SERVER ERROR".to_string()],
                );
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json))
            }
        })
}

pub async fn update_user(
    _: ApiVersion,
    Path((_v, id)): Path<(ApiVersion, String)>,
    modules: State<Arc<Modules>>,
    ValidatedRequest(source): ValidatedRequest<JsonUpdateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules
        .user_use_case()
        .update_user(source.to_view(id))
        .await;
    resp.map(|view| {
        info!("Update user: {}", view.id);
        let json: JsonUserId = view.into();
        (StatusCode::OK, Json(json))
    })
        .map_err(|err| {
            error!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn get_user(
    _: ApiVersion,
    Path((_v, id)): Path<(ApiVersion, String)>,
    modules: State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules.user_use_case().get_user(id).await;

    match resp {
        Ok(val) => val
            .map(|val| {
                info!("Get user `{}`", val.id);
                let json: JsonUser = val.into();
                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| {
                error!("User is not found");
                StatusCode::NOT_FOUND
            }),
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_users(
    _: ApiVersion,
    Query(query): Query<PaginateQuery>,
    modules: State<Arc<Modules>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let limit = query.limit;
    let offset = query.offset;
    let resp = modules.user_use_case().get_users(query.into()).await;

    match resp {
        Ok(val) => match val {
            Some(values) => {
                let users = values
                    .into_iter()
                    .map(|user| user.into())
                    .collect();
                let json = JsonUserList::new(limit, offset, users);
                Ok((StatusCode::OK, Json(json)))
            },
            None => {
                let json = JsonUserList::new(limit, offset, vec![]);
                Ok((StatusCode::OK, Json(json)))
            }
        },
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub async fn delete_user(
    _: ApiVersion,
    Path((_v, id)): Path<(ApiVersion, String)>,
    modules: State<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let resp = modules.user_use_case().delete_user(id).await;

    match resp {
        Ok(val) => val
            .map(|val| {
                info!("Delete user `{}`", val.id);
                StatusCode::NO_CONTENT
            })
            .ok_or_else(|| {
                error!("User is not found");
                StatusCode::NOT_FOUND
            }),
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_me() {
    todo!()
}

pub async fn update_me() {
    todo!()
}

pub async fn delete_me() {
    todo!()
}
