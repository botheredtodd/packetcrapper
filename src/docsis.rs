pub(crate) enum DocsisVersions {
    Docsis20,
    Docsis30,
    Docsis31,
}

pub(crate) struct Features {
    pub(crate) concatenation_support :bool, // 01
    // option 02 is docsis version, which we already have in the parent struct
    pub(crate) fragmentation_support :bool, //03
    pub(crate) payload_header_suppression_support:bool, //04
    pub(crate) igmp_support :bool, //05
    pub(crate) privacy_support :PrivacySupportLevels, //06
    pub(crate) downstream_said_support :u8, //07
    pub(crate) upstream_service_slow_support :u8, //08
    pub(crate) optional_filtering_support :OptionalFilteringSupportLevels, //09
    pub(crate) transmit_equalizer_taps_per_modulation_interval :u8, //0a
    pub(crate) number_of_transmit_equalizer_taps :u8, //0b
    pub(crate) dcc_support :bool, //0c
    pub(crate) ip_filters_support :u8, //0d
    pub(crate) llc_filters_support :u8, //0e
    pub(crate) expanded_unicast_sid_space :bool, //0f
    pub(crate) ranging_hold_off_support :RangingHoldOffSupport, //10
    pub(crate) l2vpn_capability :L2VpnCapabilities, //11

    pub(crate) upstream_frequency_range_support :UpstreamFrequencyRanges, //14
    pub(crate) upstream_symbol_rate_support :UpstreamSymbols, //15
    pub(crate) selectable_active_code_mode_2_support :bool, //16
    pub(crate) code_hopping_mode_2_support :bool, //17
    pub(crate) multiple_transmit_channel_support :u8, //18
    pub(crate) msps_512_upstream_channel_support :u8, //19
    pub(crate) msps_256_upstream_channel_support :u8, //1a
    pub(crate) total_sid_cluster_support :u8, //1b
    pub(crate) sid_clusters_per_service_flow_support :u8, //1c
    pub(crate) multiple_receive_channel_support :u8, //1d
    pub(crate) total_downstream_service_id_support :u8, //1e
    pub(crate) resequencing_downstream_service_id_support :u8, //1f
    pub(crate) multicast_downstream_service_id_support :u8, //20
    pub(crate) multicast_downstream_service_id_forwarding :DsidForwarding, //21

}

pub(crate) enum DsidForwarding {
    GmacPromiscuousMulticast, //02
}
pub(crate) struct RangingHoldOffSupport {
    pub(crate) cm :bool,
    pub(crate) erouter :bool,
    pub(crate) emta :bool,
    pub(crate) stb :bool,



}

pub(crate) struct UpstreamSymbols {
    pub(crate) kbps160 :bool, //.... ...x
    pub(crate) kbps320 :bool, //.... ..x.
    pub(crate) kbps640 :bool, //.... .x..
    pub(crate) kbps1280 :bool,//.... x...
    pub(crate) kbps2560 :bool,//...x ....
    pub(crate) kbps5120 :bool,//..x. ....
}
pub(crate) enum PrivacySupportLevels {
    BPIPlus,
}

pub(crate) enum OptionalFilteringSupportLevels {
    Reserved, //03
}

pub(crate) enum L2VpnCapabilities {
    NotCompliant, //00 -- default
}

pub(crate) enum UpstreamFrequencyRanges {
    Standard, //00
}
