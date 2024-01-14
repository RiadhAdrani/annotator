use std::io::Write;

use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm,
    },
    Multipart,
};
use actix_web::{middleware, post, web, App, Error, HttpResponse, HttpServer, Responder};
use futures_util::TryStreamExt as _;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    file: TempFile,
}

#[post("/upload")]
pub async fn upload_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let path = format!("./tmp/{}", form.file.file_name.unwrap());
    log::info!("saving to {path}");
    form.file.file.persist(path).unwrap();

    Ok(HttpResponse::Ok())
}
