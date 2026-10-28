# Dependency security review

Supply Weave runs `cargo deny check` as a required local and CI gate. The dated
candidate has four narrowly scoped advisory exceptions inherited from pinned
executed upstreams; none is hidden or treated as fixed.

| Advisory | Chain | Decision and runtime boundary |
|---|---|---|
| `RUSTSEC-2025-0141` | `ruvector-core 2.4.0 -> bincode 2.0.1` | Unmaintained-only advisory with no safe upgrade. RuVector is required for executed vector recall; only deterministic fixed-size embeddings and generated identifiers cross this boundary. |
| `RUSTSEC-2026-0391` | `datafusion 55.2.0 -> arrow-ipc 59.3.2 -> zstd 0.13.3` | The fixed `zstd >=0.14.3` is incompatible with Arrow's `^0.13` requirement. Supply Weave registers an in-memory Arrow `MemTable`; it neither reads nor writes compressed Arrow IPC or reused Zstandard streams. |
| `RUSTSEC-2026-0398` | `arrow-ipc 59.3.2 -> zstd-safe 7.3.0` | The fixed `zstd-safe >=8.2.0` is incompatible with the pinned chain. No seekable Zstandard decompression occurs in the application workload. |
| `RUSTSEC-2026-0403` | `arrow-ipc 59.3.2 -> zstd-safe 7.3.0` | Same incompatible upstream constraint. The service does not accept compressed or legacy Zstandard frames and does not call streaming decompression. |

The two additionally allowed licenses are OSI/free-software compatible and are
required by the exact upstream graph: `CC0-1.0` for `tiny-keccak` through Arrow's
hashing stack, and `BSL-1.0` for `xxhash-rust` through RVF. All unknown registries,
unknown git sources and yanked packages remain denied.

These exceptions are appropriate only for this synthetic, local prototype. A
production promotion must either upgrade DataFusion/Arrow and RuVector to fixed
compatible releases or remove the affected paths before accepting untrusted data.
