<!--
 Copyright (c) 2026 Tylium.
 SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-TELF
 See LICENSE.md for details.
-->

# Contributing to Cutoff Fusion

Thanks for your interest. This project is **incubating**, so the surface is small and moving fast.

## Licensing and the CLA

Cutoff Fusion is **dual-licensed**:

- **AGPL-3.0** for open-source use.
- **Tylium Evolutive License Framework (TELF)** — a commercial license for proprietary use, with
  professional support. Commercial licenses: <https://cutoff.dev> · `legal@tylium.io`.

Because the project is dual-licensed, **all contributors must sign the Contributor License Agreement
(CLA)** in [`license-telf/CLA.md`](license-telf/CLA.md) before their code can be merged. The CLA grants
the project the rights it needs to distribute your contribution under both tiers. It is signed once,
covers all future contributions, and does not transfer ownership.

By submitting a pull request you confirm that:

1. You have the right to contribute the code (it is your own work, or you are authorized).
2. You agree to license it under the project's dual-license terms via the CLA.
3. New source files carry the standard SPDX header:
   `SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-TELF`.

## Development

- `cargo build` · `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all`

Keep the `cutoff` umbrella crate minimal; component crates land in this workspace as `cutoff-*`.
