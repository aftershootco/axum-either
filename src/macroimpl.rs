#![allow(unused_parens)]

use axum::http::Request;
use axum::response::IntoResponse;

use crate::*;

macro_rules! impl_handler_either {
    (($($either:ident),* $(,)?),($($or:ident),* $(,)?)) => {
        impl<H1, H2, $($either),* , $($or),*, S, B, M> axum::handler::Handler<(M, ($($either),*,), ($($or),*,)), S, B>
            for EitherHandler<H1, H2>
        where
            H1: axum::handler::Handler<(M, $($either),*), S, B>,
            H2: axum::handler::Handler<(M, $($or),*), S, B>,
            B: Send + 'static,
            S: Send + Sync + 'static,
            $($either: axum::extract::FromRequestParts<S> + Send,
            <$either as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send),*
            ,
            $($or: axum::extract::FromRequestParts<S> + Send,
            <$or as axum::extract::FromRequestParts<S>>::Rejection: std::marker::Send),*
            // ,
            // $(Either<$either, $or>: FromRequestParts<S> + Send,
            // <Either<$either, $or> as FromRequestParts<S>>::Rejection: std::marker::Send),*
        {
            type Future = core::pin::Pin<Box<dyn core::future::Future<Output = axum::response::Response> + Send>>;
            fn call(self, req: Request<B>, state: S) -> Self::Future {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    if <($($either),*) as axum::extract::FromRequestParts<S>>::from_request_parts(&mut parts, &state).await.is_err() {
                            let req = Request::from_parts(parts, body);
                            self.either.call(req, state).await.into_response()
                    } else {
                            let req = Request::from_parts(parts, body);
                            self.or.call(req, state).await.into_response()
                    }
                })
            }
        }

    };
}

#[rustfmt::skip]
macro_rules! all_the_A_tuples {
    ($name:ident, $func: ident) => {
        $name!((A1), $func);
        $name!((A1, A2), $func);
        $name!((A1, A2, A3), $func);
        $name!((A1, A2, A3, A4), $func);
        $name!((A1, A2, A3, A4, A5), $func);
        $name!((A1, A2, A3, A4, A5, A6), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7, A8), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7, A8, A9), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15), $func);
        $name!((A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16), $func);
    };
}

#[rustfmt::skip]
macro_rules! all_the_AB_tuples {
    ($As: tt, $name:ident) => {
        $name!($As, (B1));
        $name!($As, (B1, B2));
        $name!($As, (B1, B2, B3));
        $name!($As, (B1, B2, B3, B4));
        $name!($As, (B1, B2, B3, B4, B5));
        $name!($As, (B1, B2, B3, B4, B5, B6));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7, B8));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7, B8, B9));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15));
        $name!($As, (B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16));
    };
}

all_the_A_tuples!(all_the_AB_tuples, impl_handler_either);
