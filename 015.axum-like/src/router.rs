pub mod empty_router;
pub mod future;
pub mod method_filter;
pub mod route;

use std::{
    borrow::Cow,
    convert::Infallible,
    fmt,
    future::ready,
    marker::PhantomData,
    sync::Arc,
    task::{Context, Poll},
};

use crate::body::{box_body, BoxBody};

use http::{Request, Response, StatusCode, Uri};
use tower::{
    util::{BoxService, ServiceExt},
    ServiceBuilder,
};

use tower_http::map_response_body::MapResponseBodyLayer;
use tower_layer::Layer;
use tower_service::Service;

use crate::BoxError;

use self::{
    empty_router::{EmptyRouter, FromEmptyRouter},
    route::{PathPattern, Route},
    future::{EmptyRouterFuture, RouteFuture},
};

pub use self::method_filter::MethodFilter;

pub struct Router<S> {
    // 代表 Service
    svc: S,
}

impl<E> Router<EmptyRouter<E>> {
    // 创建一个新的路由，默认是 Not Found
    pub fn new() -> Self {
        Self {
            svc: EmptyRouter::not_found(),
        }
    }
}

impl<E> Default for Router<EmptyRouter<E>> {
    fn default() -> Self {
        Self::new()
    }
}

// 为 Router 实现 Service
impl<S, R> Service<R> for Router<S>
    where S: Service<R>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.svc.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: R) -> Self::Future {
        self.svc.call(req)
    }
}


impl<S> Router<S> {
    pub fn route<T>(self, path: &str, svc: T) -> Router<Route<T, S>> {
        self.map(|fallback| Route {
            pattern: PathPattern::new(path),
            svc,
            fallback,
        })
    }

    fn map<F, S2>(self, f: F) -> Router<S2> where F: FnOnce(S) -> S2 {
        Router { svc: f(self.svc) }
    }
}