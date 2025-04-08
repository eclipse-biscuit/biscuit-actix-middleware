/*
 * SPDX-FileCopyrightText: 2023 Tristan Germain <ge.tristan@gmail.com>
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

pub(crate) type MiddlewareResult<R> = Result<R, HttpResponse>;

#[derive(Debug, Display)]
pub enum MiddlewareError {
    InvalidHeader,
    InvalidToken,
}

impl ResponseError for MiddlewareError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MiddlewareError::InvalidHeader => HttpResponse::Unauthorized().finish(),
            MiddlewareError::InvalidToken => HttpResponse::Forbidden().finish(),
        }
    }
}
