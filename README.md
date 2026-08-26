# plex-exporter

A Prometheus exporter for Plex, written in Rust. It exposes the same
metrics as [prometheus-plex-exporter](https://github.com/grafana/plexporter):

- `server_info` — gauge, always `1`. Labeled with server type/name/id, version, platform, platform version.
- `host_cpu_util` / `host_mem_util` — gauges. Host resource utilization (requires Plex Pass).
- `transmit_bytes_total` — counter. Bytes transmitted per Plex's own bandwidth statistics (requires Plex Pass).
- `library_duration_total` / `library_storage_total` — gauges per library.
- `plays_total` / `play_seconds_total` — counters per playback session.
- `estimated_transmit_bytes_total` — counter, estimated bytes transmitted based on active session bitrates.

## Configuration

Configured via environment variables:

- `PLEX_SERVER`: full URL of your Plex server, e.g. `http://192.168.0.10:32400`.
- `PLEX_TOKEN`: a [Plex token](https://support.plex.tv/articles/204059436-finding-an-authentication-token-x-plex-token/) belonging to the server administrator.
- `RUST_LOG`: log level filter (e.g. `info`, `debug`). Defaults to no output.

## Running

```bash
PLEX_SERVER=http://192.168.0.10:32400 PLEX_TOKEN=... cargo run --release
```

Metrics are served on `:9000/metrics`.

## Design notes

Library and session metrics are recomputed from current state on every
scrape via a custom `prometheus::core::Collector`, so stale libraries and
finished sessions drop out of `/metrics` instead of lingering. Sessions are
also pruned from memory a minute after they stop.

Unlike the upstream Go exporter, the websocket connection to Plex's
notification stream is retried with a fixed backoff on error instead of
exiting the process.
