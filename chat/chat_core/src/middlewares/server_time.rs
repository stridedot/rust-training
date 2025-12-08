use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::{extract::Request, response::Response};
use tower::{Layer, Service};

use crate::middlewares::{HEADER_REQUEST_ID, HEADER_SERVER_TIME};

#[derive(Clone)]
pub struct ServerTimeLayer;

impl<S> Layer<S> for ServerTimeLayer {
    type Service = ServerTimeService<S>;

    fn layer(&self, service: S) -> Self::Service {
        ServerTimeService { inner: service }
    }
}

#[derive(Clone)]
pub struct ServerTimeService<S> {
    inner: S,
}

impl<S> Service<Request> for ServerTimeService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let start = std::time::Instant::now();
        let future = self.inner.call(request);
        Box::pin(async move {
            let mut res: Response = future.await?;
            let elapsed = format!("{:?}us", start.elapsed().as_micros());
            match elapsed.parse() {
                Ok(v) => {
                    res.headers_mut().insert(HEADER_SERVER_TIME, v);
                }
                Err(e) => {
                    let id = res.headers().get(HEADER_REQUEST_ID);
                    tracing::warn!("Failed to parse server time for {:?}: {:?}", id, e);
                }
            }

            Ok(res)
        })
    }
}
