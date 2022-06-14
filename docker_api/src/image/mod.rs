//! Image API: creating, manipulating and pushing docker images
use std::pin::Pin;
use std::boxed::Box;
use futures_util::Stream;
use futures_util::{stream, stream::StreamExt};
use std::hash::Hash;
use futures::TryStreamExt;
use serde::Serialize;
use ntex::http::body::Body;

use crate::client::HttpClient;
use crate::api::DockerApiError;

pub mod models;

use models::{BuildInfo, BuildImageOptions};

#[derive(Default)]
pub struct Image {
  client: HttpClient,
}

impl Image {
  pub(crate) fn new(client: HttpClient) -> Self {
    Image { client }
  }

  /// ---
  ///
  /// # Build Image
  ///
  /// Build an image from a tar archive with a `Dockerfile` in it.
  ///
  /// The `Dockerfile` specifies how the image is built from the tar archive. It is typically in
  /// the archive's root, but can be at a different path or have a different name by specifying
  /// the `dockerfile` parameter.
  ///
  /// # Arguments
  ///
  ///  - [Build Image Options](BuildImageOptions) struct.
  ///  - Optional [Docker Credentials](DockerCredentials) struct.
  ///  - Tar archive compressed with one of the following algorithms: identity (no compression),
  ///    gzip, bzip2, xz. Optional [Hyper Body](hyper::body::Body).
  ///
  /// # Returns
  ///
  ///  - [Create Image Info](CreateImageInfo), wrapped in an asynchronous
  ///  Stream.
  ///
  /// # Examples
  ///
  /// ```rust
  /// # let docker = docker_api::Api::new();
  /// use docker_api::image::models::BuildImageOptions;
  /// use bollard::container::Config;
  ///
  /// use std::default::Default;
  /// use std::fs::File;
  /// use std::io::Read;
  ///
  /// let options = BuildImageOptions {
  ///     dockerfile: "Dockerfile",
  ///     t: "my-image",
  ///     rm: true,
  ///     ..Default::default()
  /// };
  ///
  /// let mut file = File::open("tarball.tar.gz").unwrap();
  /// let mut contents = Vec::new();
  /// file.read_to_end(&mut contents).unwrap();
  ///
  /// docker.build_image(options, None, Some(contents.into()));
  /// ```
  pub async fn build<T>(
    &self,
    options: BuildImageOptions<T>,
    tar: Option<Body>,
  ) -> impl Stream<Item = Result<BuildInfo, DockerApiError>>
  where
    T: Into<String> + Eq + Hash + Serialize,
  {
    let req = match self.client.post("/build").query(&options) {
      Err(err) => {
        return stream::once(
          async move { Err(DockerApiError::Errorurlencode(err)) },
        )
        .boxed()
      }
      Ok(req) => req.send_body(tar.unwrap_or(Body::Empty)).await,
    };

    let stream = match req {
      Err(err) => {
        return stream::once(async move {
          Err(DockerApiError::Errorsendrequest(err))
        })
        .boxed()
      }
      Ok(req) => req,
    };

    let res = stream
      .map(|chunk| match chunk {
        Err(err) => Err(DockerApiError::Errorpayload(err)),
        Ok(byte) => Ok(serde_json::from_slice(&byte).unwrap()),
      })
      .into_stream()
      .collect();

    Box::pin(res)
  }
}
