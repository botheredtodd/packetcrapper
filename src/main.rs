use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;
use dhcproto::{v4, Encodable, Encoder};
use dhcproto::v4::{relay::{RelayInfo, RelayAgentInformation}};
use v4::OptionCode::MessageType;


fn main() {

// hardware addr
    let chaddr = vec![
        29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
    ];
    let chaddr2 = vec![
        29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 94,
    ];
    let giaddr = Ipv4Addr::from_str("192.168.2.145").unwrap();
// construct a new Message
    let mut msg = v4::Message::default();
    msg.set_flags(v4::Flags::default().set_broadcast()) // set broadcast to true
        .set_chaddr(&chaddr) // set chaddr
        .set_giaddr(giaddr)
        .set_htype(dhcproto::v4::HType::Eth)
        .opts_mut()
        .insert(v4::DhcpOption::MessageType(v4::MessageType::Discover)); // set msg type

// set some more options
    msg.opts_mut()
        .insert(v4::DhcpOption::ParameterRequestList(vec![ //opt 55
            v4::OptionCode::SubnetMask, // 1
            v4::OptionCode::Router, // 3
            v4::OptionCode::DomainNameServer, // 6
            v4::OptionCode::LogServer, // 7
            v4::OptionCode::Hostname, // 12
            v4::OptionCode::DomainName, //15
        ]));
    msg.opts_mut()
        .insert(v4::DhcpOption::ClientIdentifier(chaddr)); //opt 63
    msg.opts_mut()
        .insert(v4::DhcpOption::MaxMessageSize(576)); //opt 57


    let mut info = RelayAgentInformation::default();
    info.insert(RelayInfo::LinkSelection("1.2.3.4".parse().unwrap()));
    info.insert(RelayInfo::AgentRemoteId(chaddr2));

   // let mut msg = v4::Message::default();
    msg.opts_mut()
        .insert(v4::DhcpOption::RelayAgentInformation(info)); //82
    msg.opts_mut()
        .insert(v4::DhcpOption::MessageType(v4::MessageType::Discover));


    

    

// now encode to bytes
    let mut buf = Vec::new();
    let mut e = Encoder::new(&mut buf);
    msg.encode(&mut e);

    let socket = UdpSocket::bind("192.168.2.145:68").expect("couldn't bind to address");
    socket.connect("192.168.2.221:67").expect("TODO: panic message");
    socket.send(&buf).expect("couldn't send data");
// buf now has the contents of the encoded DHCP message

}