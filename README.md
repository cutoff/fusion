# Cutoff Fusion

**The public Rust stack for the Cutoff audio-software framework** — a cargo workspace of crates published
to crates.io.

> ⚠️ **Incubating** — early stage. APIs will change.

## Crates

| Crate | What | License |
|---|---|---|
| [`cutoff`](cutoff/) | Umbrella / root crate — the namespace anchor for the ecosystem. | AGPL-3.0 (TELF) |
| [`cutoff-common`](cutoff-common/) | Shared utilities used across the Cutoff crates. | MIT OR Apache-2.0 |

More component crates (`cutoff-*`) land here as they mature.

## Licensing

**Licensing is per-crate** (see the table above). The Fusion crates are dual-licensed under the **Tylium
Evolutive License Framework (TELF)**:

- **Open source:** GNU **AGPL-3.0** — see [`LICENSE.md`](LICENSE.md).
- **Commercial:** a commercial license lifts the AGPL obligations. Contact `legal@tylium.io`.

Foundational utilities such as **`cutoff-common`** are permissive (**MIT OR Apache-2.0**). The TELF framework,
CLA, and SLA are in [`license-telf/`](license-telf/). Contributions require the CLA — see
[`CONTRIBUTING.md`](CONTRIBUTING.md).

A **Tylium** product — <https://cutoff.dev>.
