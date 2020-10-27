//! # Core Constants
//!
//! This module contains a few constants that are common enough that they
//! can be shared here. Do not use this to share RPC data types

pub const OP_PERFORM_LIVE_UPDATE: &str = "PerformLiveUpdate";
pub const OP_IDENTIFY_CAPABILITY: &str = "IdentifyCapability";
pub const OP_HEALTH_REQUEST: &str = "HealthRequest";
pub const OP_INITIALIZE: &str = "Initialize";
pub const OP_BIND_ACTOR: &str = "BindActor";
pub const OP_REMOVE_ACTOR: &str = "RemoveActor";

// Keys used for providing actor claim data to a capability provider during binding

pub const CONFIG_WASCC_CLAIMS_ISSUER: &str = "__wascc_issuer";
pub const CONFIG_WASCC_CLAIMS_CAPABILITIES: &str = "__wascc_capabilities";
pub const CONFIG_WASCC_CLAIMS_NAME: &str = "__wascc_name";
pub const CONFIG_WASCC_CLAIMS_EXPIRES: &str = "__wascc_expires";
pub const CONFIG_WASCC_CLAIMS_TAGS: &str = "__wascc_tags";
