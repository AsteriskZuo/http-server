use anyhow::{Context, Result};
use http::response::Builder as HttpResponseBuilder;
use http::{StatusCode, Uri};
use hyper::{Body, Response};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct ApiServer {
  root_dir: PathBuf,
}

impl<'a> ApiServer {
  /// Creates a new instance of the `FileExplorer` with the provided `root_dir`
  pub fn new(root_dir: PathBuf) -> Self {
    ApiServer { root_dir }
  }

  /// Resolves a HTTP Request to a api.
  pub async fn resolve(&self, req_path: String) -> Result<Response<Body>> {
    Ok(
      HttpResponseBuilder::new()
        .header(http::header::CONTENT_TYPE, "text/html")
        .status(StatusCode::OK)
        .body(Body::from("sdsdf"))
        .expect("Failed to build response"),
    )
  }
}
