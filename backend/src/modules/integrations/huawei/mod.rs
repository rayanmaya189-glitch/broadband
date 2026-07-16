//! Huawei OLT Integration Module
//!
//! Supports Huawei GPON/XG-PON OLT devices (MA5683T, MA5800 series) via:
//! - SSH CLI commands for configuration
//! - DBA profile management (upstream bandwidth)
//! - Traffic table management (downstream bandwidth)
//! - ONT authorization and provisioning

pub mod adapter;

pub use adapter::{
    HuaweiOltAdapter, HuaweiOltSshAdapter, HuaweiOltConfig, DbaProfile, DbaProfileType,
    TrafficTable, OntStatus, PonInterfaceStatus, CliResult,
};
