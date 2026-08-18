# cutoff

**Umbrella / root crate for the Cutoff audio-software framework ecosystem.**

> ⚠️ **Incubating** — early stage. The API surface is minimal and will change.

`cutoff` is the namespace anchor for the Cutoff ecosystem. As component crates publish, this crate will
re-export them behind feature flags (`cutoff::midi`, …). Today it carries the ecosystem's documentation
and reserves the name.

Component crates are published as `cutoff-*` (e.g. `cutoff-common`, `cutoff-midi`), stewarded by Tylium.

## Licensing

Dual-licensed under the **Tylium Evolutive License Framework (TELF)**:

- **Open source:** GNU **AGPL-3.0** — see [`LICENSE.md`](LICENSE.md).
- **Commercial:** a commercial license lifts the AGPL obligations. Contact `legal@tylium.io`. The
  framework, CLA, and SLA are in the repository's
  [`license-telf/`](https://github.com/cutoff/fusion/tree/main/license-telf).

A **Tylium** product — <https://cutoff.dev>.
