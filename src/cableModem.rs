use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use dhcproto::{v4, Encodable, Encoder};
use dhcproto::v4::{relay::{RelayInfo, RelayAgentInformation}};
use v4::OptionCode::MessageType;
use crate::{docsis};
use crate::docsis::{DocsisVersions, L2VpnCapabilities, OptionalFilteringSupportLevels, PrivacySupportLevels, RangingHoldOffSupport, UpstreamFrequencyRanges, UpstreamSymbols};


pub(crate) struct CableModem {
    pub(crate) mac_address :Vec<u8>,
    pub(crate) giaddr :Ipv4Addr,
    pub(crate) docsis_version :docsis::DocsisVersions,
    pub(crate) features :docsis::Features
}

impl CableModem {
    /// It creates a new CableModem struct.
    ///
    /// Returns:
    ///
    /// A CableModem struct with all zeros for the MAC
    fn new() -> CableModem {
        CableModem{
            mac_address: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            giaddr: (Ipv4Addr::from_str("0.0.0.0").unwrap()),
            docsis_version: DocsisVersions::Docsis31,
            features: docsis::Features {
                concatenation_support: false,
                fragmentation_support: false,
                payload_header_suppression_support: false,
                igmp_support: false,
                privacy_support: PrivacySupportLevels::BPIPlus,
                downstream_said_support: 0,
                upstream_service_slow_support: 0,
                optional_filtering_support: OptionalFilteringSupportLevels::Reserved,
                transmit_equalizer_taps_per_modulation_interval: 0,
                number_of_transmit_equalizer_taps: 0,
                dcc_support: false,
                ip_filters_support: 0,
                llc_filters_support: 0,
                expanded_unicast_sid_space: false,
                ranging_hold_off_support: RangingHoldOffSupport {
                    cm: false,
                    erouter: false,
                    emta: false,
                    stb: false
                },
                l2vpn_capability: L2VpnCapabilities::NotCompliant,
                upstream_frequency_range_support: UpstreamFrequencyRanges::Standard,
                upstream_symbol_rate_support: UpstreamSymbols {
                    kbps160: false,
                    kbps320: false,
                    kbps640: false,
                    kbps1280: false,
                    kbps2560: false,
                    kbps5120: true
                },
                selectable_active_code_mode_2_support: false,
                code_hopping_mode_2_support: false
            }
        }
    }
}