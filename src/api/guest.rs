use actix_web::{
  web::{Data, Json},
  Result,
};
use app_error::ErrorCode;
use chrono::Utc;
use shared_entity::{
  dto::guest_dto::{
    RevokeSharedViewAccessRequest, ShareViewWithGuestRequest, SharedViewDetails,
    SharedViewDetailsRequest, SharedViews,
  },
  dto::workspace_dto::{FolderView, ViewIcon, ViewLayout},
  response::{AppResponseError, JsonAppResponse},
};

use actix_web::{
  web::{self},
  Scope,
};
use uuid::Uuid;

use crate::biz::authentication::jwt::UserUuid;
use crate::state::AppState;

pub fn sharing_scope() -> Scope {
  web::scope("/api/sharing/workspace")
    .service(
      web::resource("{workspace_id}/view")
        .route(web::get().to(list_shared_views_handler))
        .route(web::put().to(put_shared_view_handler)),
    )
    .service(
      web::resource("{workspace_id}/view/{view_id}/access-details")
        .route(web::post().to(shared_view_access_details_handler)),
    )
    .service(
      web::resource("{workspace_id}/view/{view_id}/revoke-access")
        .route(web::post().to(revoke_shared_view_access_handler)),
    )
    .service(
      web::resource("{workspace_id}/folder")
        .route(web::get().to(get_share_with_me_folder_handler)),
    )
}

async fn list_shared_views_handler(
  _user_uuid: UserUuid,
  _state: Data<AppState>,
  _path: web::Path<Uuid>,
) -> Result<JsonAppResponse<SharedViews>> {
  Err(
    AppResponseError::new(
      ErrorCode::FeatureNotAvailable,
      "this version of appflowy cloud server does not support guest editors",
    )
    .into(),
  )
}

async fn put_shared_view_handler(
  _user_uuid: UserUuid,
  _state: Data<AppState>,
  _payload: web::Json<ShareViewWithGuestRequest>,
  _path: web::Path<Uuid>,
) -> Result<JsonAppResponse<()>> {
  Err(
    AppResponseError::new(
      ErrorCode::FeatureNotAvailable,
      "this version of appflowy cloud server does not support guest editors",
    )
    .into(),
  )
}

async fn shared_view_access_details_handler(
  _user_uuid: UserUuid,
  _state: Data<AppState>,
  _json: Json<SharedViewDetailsRequest>,
  _path: web::Path<(Uuid, Uuid)>,
) -> Result<JsonAppResponse<SharedViewDetails>> {
  Err(
    AppResponseError::new(
      ErrorCode::FeatureNotAvailable,
      "this version of appflowy cloud server does not support guest editors",
    )
    .into(),
  )
}

async fn revoke_shared_view_access_handler(
  _user_uuid: UserUuid,
  _state: Data<AppState>,
  _payload: web::Json<RevokeSharedViewAccessRequest>,
  _path: web::Path<(Uuid, Uuid)>,
) -> Result<JsonAppResponse<()>> {
  Err(
    AppResponseError::new(
      ErrorCode::FeatureNotAvailable,
      "this version of appflowy cloud server does not support guest editors",
    )
    .into(),
  )
}

/// Stub endpoint for "Shared with Me" folder
/// Returns an empty folder view since this open-source version doesn't support guest sharing
async fn get_share_with_me_folder_handler(
  _user_uuid: UserUuid,
  _state: Data<AppState>,
  _workspace_id: web::Path<Uuid>,
) -> Result<JsonAppResponse<FolderView>> {
  // Return an empty "Shared with Me" folder
  // The frontend checks if children.length > 0, so an empty folder is ignored gracefully
  // IMPORTANT: Use static timestamps to prevent triggering React re-renders on every request
  use chrono::TimeZone;
  let static_time = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
  
  let empty_folder = FolderView {
    view_id: Uuid::nil(),
    parent_view_id: None,
    prev_view_id: None,
    name: "Shared with Me".to_string(),
    icon: Some(ViewIcon {
      ty: shared_entity::dto::workspace_dto::IconType::Emoji,
      value: "📤".to_string(),
    }),
    is_space: false,
    is_private: false,
    is_published: false,
    is_favorite: false,
    layout: ViewLayout::Document,
    created_at: static_time,
    created_by: None,
    last_edited_by: None,
    last_edited_time: static_time,
    is_locked: None,
    extra: None,
    children: vec![], // Empty children array - frontend will ignore this
  };
  
  Ok(shared_entity::response::AppResponse::Ok()
    .with_data(empty_folder)
    .into())
}
