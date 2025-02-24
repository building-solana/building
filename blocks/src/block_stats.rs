use log::info;

#[derive(Default)]
pub struct EngineStats {
    heartbeat_count: u64,
    heartbeat_elapsed_us: u64,

    aoi_update_count: u64,
    aoi_update_elapsed_us: u64,
    aoi_accounts_received: u64,

    poi_update_count: u64,
    poi_update_elapsed_us: u64,
    poi_accounts_received: u64,

    num_packets_received: u64,

    packet_filter_elapsed_us: u64,
    packet_forward_elapsed_us: u64,

    engine_packet_sender_len: u64,
    auth_refresh_count: u64,
    refresh_auth_elapsed_us: u64,
    packet_forward_count: u64,
    metrics_delay_us: u64,

    accounts_of_interest_len: u64,
    programs_of_interest_len: u64,
    flush_elapsed_us: u64,
}

impl EngineStats {
    pub fn increment(&mut self, field: &str, num: u64) {
        match field {
            "heartbeat_count" => self.heartbeat_count = self.heartbeat_count.saturating_add(num),
            "heartbeat_elapsed_us" => {
                self.heartbeat_elapsed_us = self.heartbeat_elapsed_us.saturating_add(num)
            }
            "aoi_update_count" => self.aoi_update_count = self.aoi_update_count.saturating_add(num),
            "aoi_update_elapsed_us" => {
                self.aoi_update_elapsed_us = self.aoi_update_elapsed_us.saturating_add(num)
            }
            "aoi_accounts_received" => {
                self.aoi_accounts_received = self.aoi_accounts_received.saturating_add(num)
            }
            "poi_update_count" => self.poi_update_count = self.poi_update_count.saturating_add(num),
            "poi_update_elapsed_us" => {
                self.poi_update_elapsed_us = self.poi_update_elapsed_us.saturating_add(num)
            }
            "poi_accounts_received" => {
                self.poi_accounts_received = self.poi_accounts_received.saturating_add(num)
            }
            "num_packets_received" => {
                self.num_packets_received = self.num_packets_received.saturating_add(num)
            }
            "packet_filter_elapsed_us" => {
                self.packet_filter_elapsed_us = self.packet_filter_elapsed_us.saturating_add(num)
            }
            "packet_forward_elapsed_us" => {
                self.packet_forward_elapsed_us = self.packet_forward_elapsed_us.saturating_add(num)
            }
            "engine_packet_sender_len" => {
                self.engine_packet_sender_len = std::cmp::max(self.engine_packet_sender_len, num)
            }
            "auth_refresh_count" => {
                self.auth_refresh_count = self.auth_refresh_count.saturating_add(num)
            }
            "refresh_auth_elapsed_us" => {
                self.refresh_auth_elapsed_us = self.refresh_auth_elapsed_us.saturating_add(num)
            }
            "packet_forward_count" => {
                self.packet_forward_count = self.packet_forward_count.saturating_add(num)
            }
            "metrics_delay_us" => self.metrics_delay_us = self.metrics_delay_us.saturating_add(num),
            "accounts_of_interest_len" => {
                self.accounts_of_interest_len = self.accounts_of_interest_len.saturating_add(num)
            }
            "programs_of_interest_len" => {
                self.programs_of_interest_len = self.programs_of_interest_len.saturating_add(num)
            }
            "flush_elapsed_us" => self.flush_elapsed_us = self.flush_elapsed_us.saturating_add(num),
            _ => {}
        }
    }

    pub fn report(&self) {
        info!(
            "Engine Stats - Heartbeat: {}, AOI Updates: {}, POI Updates: {}, Packets Received: {}, Packet Filter Time: {}, Packet Forward Time: {}, Auth Refresh: {}, Metrics Delay: {}, Flush Time: {}",
            self.heartbeat_count,
            self.aoi_update_count,
            self.poi_update_count,
            self.num_packets_received,
            self.packet_filter_elapsed_us,
            self.packet_forward_elapsed_us,
            self.auth_refresh_count,
            self.metrics_delay_us,
            self.flush_elapsed_us,
        );
    }
}
