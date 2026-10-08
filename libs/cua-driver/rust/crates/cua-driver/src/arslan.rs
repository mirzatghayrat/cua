//! Arslan's build of Cua Driver (ARSLAN-FORK.md at the repository root).
//!
//! Arslan runs this driver as a private worker of its "Arslan Hands" helper, which
//! holds the user's Accessibility and Screen Recording grants. That build never
//! reaches the network: no telemetry, no update checks, no skill downloads, no
//! self-update. Each of those senders returns early on `OFFLINE`, so the rest of
//! upstream's code (and its Cargo.lock) stays as it is.

/// Always true in this fork. The senders check it before opening any connection.
pub const OFFLINE: bool = true;

/// What a sender reports instead of reaching the network.
pub const OFFLINE_REASON: &str = "network access is compiled out of Arslan's build of Cua Driver";
