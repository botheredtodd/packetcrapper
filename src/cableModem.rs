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

}