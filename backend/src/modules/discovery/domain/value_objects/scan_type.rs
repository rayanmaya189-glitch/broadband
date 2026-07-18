use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScanType {
    SnmpWalk,
    Lldp,
    Cdp,
    ArpScan,
    MacTable,
    PonScan,
    DhcpScan,
    IcmpSweep,
}

impl ScanType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "snmp_walk" | "snmpwalk" => Some(Self::SnmpWalk),
            "lldp" => Some(Self::Lldp),
            "cdp" => Some(Self::Cdp),
            "arp_scan" | "arpscan" => Some(Self::ArpScan),
            "mac_table" | "mactable" => Some(Self::MacTable),
            "pon_scan" | "ponscan" => Some(Self::PonScan),
            "dhcp_scan" | "dhcpscan" => Some(Self::DhcpScan),
            "icmp_sweep" | "icmpsweep" => Some(Self::IcmpSweep),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SnmpWalk => "snmp_walk",
            Self::Lldp => "lldp",
            Self::Cdp => "cdp",
            Self::ArpScan => "arp_scan",
            Self::MacTable => "mac_table",
            Self::PonScan => "pon_scan",
            Self::DhcpScan => "dhcp_scan",
            Self::IcmpSweep => "icmp_sweep",
        }
    }
}

impl fmt::Display for ScanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
