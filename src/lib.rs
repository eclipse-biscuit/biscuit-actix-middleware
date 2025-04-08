/*
 * SPDX-FileCopyrightText: 2023 Tristan Germain <ge.tristan@gmail.com>
 *
 * SPDX-License-Identifier: Apache-2.0
 */

pub mod error;
mod middleware;

extern crate biscuit_auth as biscuit;

pub use middleware::BiscuitMiddleware;
