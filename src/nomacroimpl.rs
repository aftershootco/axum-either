#![allow(unused_parens)]
use crate::*;
use axum::http::Request;
use axum::response::IntoResponse;
impl<H1, H2, A1, B1, S, B, M> axum::handler::Handler<(M, (A1,), (B1,)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, S, B, M> axum::handler::Handler<(M, (A1,), (B1, B2)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, S, B, M> axum::handler::Handler<(M, (A1,), (B1, B2, B3)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, S, B, M> axum::handler::Handler<(M, (A1,), (B1, B2, B3, B4)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<(M, (A1,), (B1, B2, B3, B4, B5)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<(M, (A1,), (B1, B2, B3, B4, B5, B6)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<(M, (A1,), (B1, B2, B3, B4, B5, B6, B7)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<(M, (A1,), (B1, B2, B3, B4, B5, B6, B7, B8)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, B7, B8, B9, S, B, M>
    axum::handler::Handler<(M, (A1,), (B1, B2, B3, B4, B5, B6, B7, B8, B9)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, S, B, M>
    axum::handler::Handler<(M, (A1,), (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, S, B, M>
    axum::handler::Handler<(M, (A1,), (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1,),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1,),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1,),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1,),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1,),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, S, B, M> axum::handler::Handler<(M, (A1, A2), (B1,)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, S, B, M> axum::handler::Handler<(M, (A1, A2), (B1, B2)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, S, B, M> axum::handler::Handler<(M, (A1, A2), (B1, B2, B3)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<(M, (A1, A2), (B1, B2, B3, B4)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<(M, (A1, A2), (B1, B2, B3, B4, B5)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<(M, (A1, A2), (B1, B2, B3, B4, B5, B6)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<(M, (A1, A2), (B1, B2, B3, B4, B5, B6, B7)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<(M, (A1, A2), (B1, B2, B3, B4, B5, B6, B7, B8)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, B7, B8, B9, S, B, M>
    axum::handler::Handler<(M, (A1, A2), (B1, B2, B3, B4, B5, B6, B7, B8, B9)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, S, B, M>
    axum::handler::Handler<(M, (A1, A2), (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, S, B, M>
    axum::handler::Handler<(M, (A1, A2), (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, S, B, M> axum::handler::Handler<(M, (A1, A2, A3), (B1,)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, S, B, M> axum::handler::Handler<(M, (A1, A2, A3), (B1, B2)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3), (B1, B2, B3)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3), (B1, B2, B3, B4)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3), (B1, B2, B3, B4, B5)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3), (B1, B2, B3, B4, B5, B6)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3), (B1, B2, B3, B4, B5, B6, B7)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3), (B1, B2, B3, B4, B5, B6, B7, B8)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, B6, B7, B8, B9, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3), (B1, B2, B3, B4, B5, B6, B7, B8, B9)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3), (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, S, B, M> axum::handler::Handler<(M, (A1, A2, A3, A4), (B1,)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4), (B1, B2)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4), (B1, B2, B3)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4), (B1, B2, B3, B4)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4), (B1, B2, B3, B4, B5)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4), (B1, B2, B3, B4, B5, B6)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4), (B1, B2, B3, B4, B5, B6, B7)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4), (B1, B2, B3, B4, B5, B6, B7, B8)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, B5, B6, B7, B8, B9, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4), (B1, B2, B3, B4, B5, B6, B7, B8, B9)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5), (B1,)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5), (B1, B2)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5), (B1, B2, B3)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5), (B1, B2, B3, B4)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5), (B1, B2, B3, B4, B5)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5), (B1, B2, B3, B4, B5, B6)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5), (B1, B2, B3, B4, B5, B6, B7)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5), (B1, B2, B3, B4, B5, B6, B7, B8)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, B4, B5, B6, B7, B8, B9, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6), (B1,)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6), (B1, B2)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, B3, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6), (B1, B2, B3)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6), (B1, B2, B3, B4)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6), (B1, B2, B3, B4, B5)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6), (B1, B2, B3, B4, B5, B6)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6), (B1, B2, B3, B4, B5, B6, B7)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, B7, B8, B9, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state,
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7), (B1,)), S, B> for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7), (B1, B2)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, B3, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7), (B1, B2, B3)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7), (B1, B2, B3, B4)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7), (B1, B2, B3, B4, B5)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7), (B1, B2, B3, B4, B5, B6)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, B3, B4, B5, B6, B7, B8, B9, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8), (B1,)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8), (B1, B2)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8), (B1, B2, B3)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8), (B1, B2, B3, B4)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8), (B1, B2, B3, B4, B5)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (B1, B2, B3, B4, B5, B6),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, B9, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, B1, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8, A9), (B1,)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, B1, B2, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8, A9), (B1, B2)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, B1, B2, B3, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8, A9), (B1, B2, B3)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8, A9), (B1, B2, B3, B4)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5, B6),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, B1, B2, B3, B4, B5, B6, B7, B8, B9, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, B1, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (B1,)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, B1, B2, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (B1, B2)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, B1, B2, B3, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (B1, B2, B3)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5, B6),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, B1, B2, B3, B4, B5, B6, B7, B8, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, B1, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), (B1,)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, B1, B2, S, B, M>
    axum::handler::Handler<(M, (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), (B1, B2)), S, B>
    for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, B1, B2, B3, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, B1, B2, B3, B4, B5, B6, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5, B6),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, B1, B2, B3, B4, B5, B6, B7, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, B1, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1,),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, B1, B2, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, B1, B2, B3, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, B1, B2, B3, B4, B5, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5, B6),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, B1, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1,),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, B1, B2, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, B1, B2, B3, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, B1, B2, B3, B4, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5, B6),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<(M, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), S, B>,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, B1, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1,),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, B1, B2, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, B1, B2, B3, S, B, M>
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5, B6),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
            ) as axum::extract::FromRequestParts<
                S,
            >>::from_request_parts(&mut parts, &state)
                .await
                .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, B1, S, B, M>
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1,),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<H1, H2, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, B1, B2, S, B, M>
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5, B6),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1,),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5, B6),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5, B6, B7),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<(M, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13), S, B>,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
impl<
        H1,
        H2,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        A13,
        A14,
        A15,
        A16,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
        B13,
        B14,
        B15,
        B16,
        S,
        B,
        M,
    >
    axum::handler::Handler<
        (
            M,
            (
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ),
            (
                B1,
                B2,
                B3,
                B4,
                B5,
                B6,
                B7,
                B8,
                B9,
                B10,
                B11,
                B12,
                B13,
                B14,
                B15,
                B16,
            ),
        ),
        S,
        B,
    > for EitherHandler<H1, H2>
where
    H1: axum::handler::Handler<
        (
            M,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
            A16,
        ),
        S,
        B,
    >,
    H2: axum::handler::Handler<
        (
            M,
            B1,
            B2,
            B3,
            B4,
            B5,
            B6,
            B7,
            B8,
            B9,
            B10,
            B11,
            B12,
            B13,
            B14,
            B15,
            B16,
        ),
        S,
        B,
    >,
    B: Send + 'static,
    S: Send + Sync + 'static,
    A1: axum::extract::FromRequestParts<S> + Send,
    <A1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A2: axum::extract::FromRequestParts<S> + Send,
    <A2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A3: axum::extract::FromRequestParts<S> + Send,
    <A3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A4: axum::extract::FromRequestParts<S> + Send,
    <A4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A5: axum::extract::FromRequestParts<S> + Send,
    <A5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A6: axum::extract::FromRequestParts<S> + Send,
    <A6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A7: axum::extract::FromRequestParts<S> + Send,
    <A7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A8: axum::extract::FromRequestParts<S> + Send,
    <A8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A9: axum::extract::FromRequestParts<S> + Send,
    <A9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A10: axum::extract::FromRequestParts<S> + Send,
    <A10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A11: axum::extract::FromRequestParts<S> + Send,
    <A11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A12: axum::extract::FromRequestParts<S> + Send,
    <A12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A13: axum::extract::FromRequestParts<S> + Send,
    <A13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A14: axum::extract::FromRequestParts<S> + Send,
    <A14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A15: axum::extract::FromRequestParts<S> + Send,
    <A15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    A16: axum::extract::FromRequestParts<S> + Send,
    <A16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B1: axum::extract::FromRequestParts<S> + Send,
    <B1 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B2: axum::extract::FromRequestParts<S> + Send,
    <B2 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B3: axum::extract::FromRequestParts<S> + Send,
    <B3 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B4: axum::extract::FromRequestParts<S> + Send,
    <B4 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B5: axum::extract::FromRequestParts<S> + Send,
    <B5 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B6: axum::extract::FromRequestParts<S> + Send,
    <B6 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B7: axum::extract::FromRequestParts<S> + Send,
    <B7 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B8: axum::extract::FromRequestParts<S> + Send,
    <B8 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B9: axum::extract::FromRequestParts<S> + Send,
    <B9 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B10: axum::extract::FromRequestParts<S> + Send,
    <B10 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B11: axum::extract::FromRequestParts<S> + Send,
    <B11 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B12: axum::extract::FromRequestParts<S> + Send,
    <B12 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B13: axum::extract::FromRequestParts<S> + Send,
    <B13 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B14: axum::extract::FromRequestParts<S> + Send,
    <B14 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B15: axum::extract::FromRequestParts<S> + Send,
    <B15 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
    B16: axum::extract::FromRequestParts<S> + Send,
    <B16 as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send,
{
    type Future =
        core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
    fn call(self, req: Request<B>, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            if <(
                A1,
                A2,
                A3,
                A4,
                A5,
                A6,
                A7,
                A8,
                A9,
                A10,
                A11,
                A12,
                A13,
                A14,
                A15,
                A16,
            ) as axum::extract::FromRequestParts<S>>::from_request_parts(
                &mut parts, &state
            )
            .await
            .is_err()
            {
                let req = Request::from_parts(parts, body);
                self.either.call(req, state).await.into_response()
            } else {
                let req = Request::from_parts(parts, body);
                self.or.call(req, state).await.into_response()
            }
        })
    }
}
