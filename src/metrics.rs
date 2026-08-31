use prometheus::{CounterVec, GaugeVec, Opts};

pub const SERVER_LABELS: &[&str] = &["server_type", "server", "server_id"];

pub const LIBRARY_LABELS: &[&str] = &[
    "server_type",
    "server",
    "server_id",
    "library_type",
    "library",
    "library_id",
];

/// `PLAY_LABELS` plus a `state` label, used for metrics that describe the
/// current playback state of a session rather than an accumulated total.
pub fn active_session_labels() -> Vec<&'static str> {
    let mut labels = PLAY_LABELS.to_vec();
    labels.push("state");
    labels
}

pub const PLAY_LABELS: &[&str] = &[
    "server_type",
    "server",
    "server_id",
    "library_type",
    "library",
    "library_id",
    "media_type",
    "title",
    "child_title",
    "grandchild_title",
    "stream_type",
    "stream_resolution",
    "stream_file_resolution",
    "stream_bitrate",
    "device",
    "device_type",
    "user",
    "session",
];

/// Global, always-on metrics updated directly from server refresh polling.
/// These mirror the promauto-registered vecs in the Go exporter: they are
/// never reset, matching the upstream behavior of leaving stale series in
/// place once set.
pub struct GlobalMetrics {
    pub server_info: GaugeVec,
    pub host_cpu_util: GaugeVec,
    pub host_mem_util: GaugeVec,
    pub transmit_bytes_total: CounterVec,
    pub websocket_connected: GaugeVec,
    pub websocket_reconnects_total: CounterVec,
}

impl GlobalMetrics {
    pub fn new() -> prometheus::Result<Self> {
        let mut server_info_labels: Vec<&str> = SERVER_LABELS.to_vec();
        server_info_labels.extend(["version", "platform", "platform_version"]);

        Ok(Self {
            server_info: GaugeVec::new(Opts::new("plex_server_info", "server_info"), &server_info_labels)?,
            host_cpu_util: GaugeVec::new(Opts::new("plex_host_cpu_util", "host_cpu_util"), SERVER_LABELS)?,
            host_mem_util: GaugeVec::new(Opts::new("plex_host_mem_util", "host_mem_util"), SERVER_LABELS)?,
            transmit_bytes_total: CounterVec::new(
                Opts::new("plex_transmit_bytes_total", "transmit_bytes_total"),
                SERVER_LABELS,
            )?,
            websocket_connected: GaugeVec::new(
                Opts::new(
                    "plex_websocket_connected",
                    "Whether the Plex notification websocket is currently connected",
                ),
                SERVER_LABELS,
            )?,
            websocket_reconnects_total: CounterVec::new(
                Opts::new(
                    "plex_websocket_reconnects_total",
                    "Total number of times the Plex notification websocket had to be (re)connected",
                ),
                SERVER_LABELS,
            )?,
        })
    }

    pub fn register(&self, registry: &prometheus::Registry) -> prometheus::Result<()> {
        registry.register(Box::new(self.server_info.clone()))?;
        registry.register(Box::new(self.host_cpu_util.clone()))?;
        registry.register(Box::new(self.host_mem_util.clone()))?;
        registry.register(Box::new(self.transmit_bytes_total.clone()))?;
        registry.register(Box::new(self.websocket_connected.clone()))?;
        registry.register(Box::new(self.websocket_reconnects_total.clone()))?;
        Ok(())
    }
}
