use axum::{
    Extension, Json,
    body::Body,
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use tokio::fs;
use tokio_util::io::ReaderStream;

use crate::{AppState, repos::file::ChatFile};
use chat_core::{error::AppError, models::user::User, utils::ApiResponse};

pub async fn file_upload(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut files = Vec::new();
    let file_dir = &state.config.server.file_dir;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::InvalidRequest(format!("multipart error: {:?}", e)))?
    {
        let filename = match field.file_name() {
            Some(name) => name.to_string(),
            None => continue,
        };
        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::InvalidFile(format!("read field failed: {:?}", e)))?;

        let chat_file = ChatFile::try_new(user.workspace_id, &filename, &data)?;
        let path = chat_file.path(file_dir);

        if !path.exists() {
            fs::create_dir_all(path.parent().expect("file path parents should exists")).await?;
            fs::write(&path, &data).await?;
        }

        files.push(chat_file.url());
    }

    if files.is_empty() {
        let body = ApiResponse::error(StatusCode::BAD_REQUEST, "no file uploaded");
        Ok(Json(body))
    } else {
        Ok(Json(ApiResponse::success(files)))
    }
}

pub async fn file_download(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path((workspace_id, filepath)): Path<(i64, String)>,
) -> Result<Response, AppError> {
    if workspace_id != user.workspace_id {
        let body = ApiResponse::<()>::error(StatusCode::FORBIDDEN, "forbidden");
        return Ok(Json(body).into_response());
    }

    // 1、构建文件路径
    let file_dir = &state.config.server.file_dir.join(workspace_id.to_string());
    let path = file_dir.join(filepath);

    // 2、检查文件是否存在
    if !path.exists() {
        let body = ApiResponse::<()>::error(StatusCode::NOT_FOUND, "file not found");
        return Ok(Json(body).into_response());
    }

    // 3、转为流式响应
    let file_stream = fs::File::open(&path).await?;
    let stream = ReaderStream::new(file_stream);
    let body = Body::from_stream(stream);

    // 4、设置响应头
    let value = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string()
        .parse()
        .map_err(|e| AppError::InvalidFile(format!("parse mime error: {:?}", e)))?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, value);

    Ok((headers, body).into_response())
}
