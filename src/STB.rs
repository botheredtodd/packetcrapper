use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use dhcproto::{v4, Encodable, Encoder};
use dhcproto::v4::{relay::{RelayInfo, RelayAgentInformation}};
use v4::OptionCode::MessageType;


pub(crate) fn send_stb() {
let chaddr = vec![
    29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
];
let chaddr2 = vec![
    29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 94,
];
let giaddr = Ipv4Addr::from_str("192.168.2.145").unwrap();
let open_cable_2_1 = vec![
    0x4f, 0x70, 0x65, 0x6e, 0x43, 0x61, 0x62, 0x6c, 0x65, 0x32 , 0x2e , 0x31
];
let some_opts :Vec<u8> = vec![0x02, 0x04, 0x45, 0x53, 0x54, 0x42, 0x03, 0x08, 0x45, 0x43, 0x4d, 0x3a, 0x45, 0x53, 0x54, 0x42
                              , 0x04, 0x09, 0x53, 0x41, 0x43, 0x47, 0x44, 0x47, 0x4d, 0x54, 0x4d, 0x05, 0x03, 0x33, 0x2e, 0x36
                              , 0x06, 0x1c, 0x43, 0x53, 0x30, 0x31, 0x31, 0x41, 0x4e, 0x5f, 0x50, 0x52, 0x4f, 0x44, 0x5f, 0x42
                              , 0x44, 0x52, 0x49, 0x5f, 0x32, 0x30, 0x33, 0x2e, 0x32, 0x37, 0x2e, 0x32, 0x2e, 0x37, 0x07, 0x05
                              , 0x31, 0x2e, 0x30, 0x2e, 0x30, 0x08, 0x06, 0x35, 0x34, 0x44, 0x34, 0x36, 0x46, 0x09, 0x08, 0x43
                              , 0x53, 0x30, 0x31, 0x31, 0x41, 0x4e, 0x4d, 0x0a, 0x05, 0x43, 0x69, 0x73, 0x63, 0x6f
];
// construct a new Message
let mut msg = v4::Message::default();
msg.set_flags(v4::Flags::default().set_broadcast()) // set broadcast to true
.set_chaddr(&chaddr) // set chaddr
.set_giaddr(giaddr)
.set_htype(dhcproto::v4::HType::Eth)
.opts_mut()
.insert(v4::DhcpOption::MessageType(v4::MessageType::Discover)); // set msg type

// set some more options
//     msg.opts_mut()
//         .insert(v4::DhcpOption::ParameterRequestList(vec![ //opt 55
//             v4::OptionCode::SubnetMask, // 1
//             v4::OptionCode::Router, // 3
//             v4::OptionCode::DomainNameServer, // 6
//             v4::OptionCode::LogServer, // 7
//             v4::OptionCode::Hostname, // 12
//             v4::OptionCode::DomainName, //15
//         ]));
msg.opts_mut()
.insert(v4::DhcpOption::ClientIdentifier(chaddr)); //opt 63
msg.opts_mut()
.insert(v4::DhcpOption::MaxMessageSize(576)); //opt 57
msg.opts_mut()
.insert(v4::DhcpOption::ClassIdentifier(open_cable_2_1));


let mut info = RelayAgentInformation::default();
info.insert(RelayInfo::LinkSelection("1.2.3.4".parse().unwrap()));
info.insert(RelayInfo::AgentRemoteId(chaddr2));

// let mut msg = v4::Message::default();
msg.opts_mut()
.insert(v4::DhcpOption::RelayAgentInformation(info)); //82
msg.opts_mut()
.insert(v4::DhcpOption::MessageType(v4::MessageType::Discover));
msg.opts_mut()
.insert(v4::DhcpOption::VendorExtensions(some_opts));






// now encode to bytes
let mut buf = Vec::new();
let mut e = Encoder::new(&mut buf);
msg.encode(&mut e);

let socket = UdpSocket::bind("192.168.2.145:34343").expect("couldn't bind to address");
socket.connect("192.168.2.221:67").expect("TODO: panic message");
socket.send(&buf).expect("couldn't send data");
// buf now has the contents of the encoded DHCP message
}