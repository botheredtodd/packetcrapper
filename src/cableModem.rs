use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use dhcproto::{v4, Encodable, Encoder};
use dhcproto::v4::{relay::{RelayInfo, RelayAgentInformation}};
use v4::OptionCode::MessageType;
use crate::{docsis};


pub(crate) struct CableModem {
    pub(crate) mac_address :Vec<u8>,
    pub(crate) giaddr :Ipv4Addr,
    pub(crate) docsis_version :docsis::DocsisVersions,
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
}

pub(crate) struct RangingHoldOffSupport {
    pub(crate) cm :bool,
    pub(crate) erouter :bool,
    pub(crate) emta :bool,
    pub(crate) stb :bool,



}
pub(crate) enum PrivacySupportLevels {
    BPIPlus,
}

pub(crate) enum OptionalFilteringSupportLevels {
    Reserved, //03
}