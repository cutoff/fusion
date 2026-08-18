// Copyright (c) 2026 Tylium.
// SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-TELF
// See LICENSE.md for details.

//! # Cutoff
//!
//! Umbrella / root crate for the **Cutoff** audio-software framework ecosystem.
//!
//! > **Incubating — early stage. The API surface is minimal and will change.**
//!
//! ## The ecosystem
//!
//! - [`cutoff`](crate) (this crate) — the namespace anchor. As component crates
//!   publish, it will re-export them behind feature flags (`cutoff::midi`, …).
//! - `cutoff-*` — Tylium-stewarded component crates (e.g. `cutoff-common`, `cutoff-midi`).
//!
//! ## Licensing
//!
//! Dual-licensed under the **Tylium Evolutive License Framework (TELF)**: an
//! **AGPL-3.0** open-source arm plus a **commercial** option (contact
//! `legal@tylium.io`). See `LICENSE.md` and the `license-telf/` directory.
//!
//! A Tylium product — <https://cutoff.dev>.

#![forbid(unsafe_code)]

/// This crate's version string, from `CARGO_PKG_VERSION`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::VERSION;

    #[test]
    fn version_is_populated() {
        assert!(!VERSION.is_empty());
    }
}
