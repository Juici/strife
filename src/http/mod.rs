//! The HTTP module provides low-level functions for interacting with the
//! Discord API.
//!
//! Discord API endpoints are ratelimited to prevent spam. The library takes
//! preventative measures to ensure that requests are not ratelimited.

mod routing;
