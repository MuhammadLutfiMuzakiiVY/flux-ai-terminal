//! Shared bindings for Flux AI Terminal
//! Provides UniFFI for iOS and JNI for Android

uniffi::setup_scaffolding!();

use flux_core::bridge::{BridgeMessage, serialize_message, deserialize_message};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static FLUX_INITIALIZED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[uniffi::export]
pub fn init_flux(data_dir: String) -> bool {
    let mut initialized = FLUX_INITIALIZED.lock().unwrap();
    if *initialized {
        return true;
    }
    
    // In a real app, we would start the async runtime and initialize the engine here
    // tokio::runtime::Runtime::new().unwrap().block_on(async { ... })
    
    *initialized = true;
    true
}

#[uniffi::export]
pub fn send_message(json: String) -> String {
    match deserialize_message(&json) {
        Ok(msg) => {
            // Process message (mocked for now, in real app this goes to the core engine)
            let response = match msg {
                BridgeMessage::GetSystemInfo => BridgeMessage::SystemInfoResult {
                    info_json: r#"{"app_name": "Flux AI Terminal", "version": "1.0.0"}"#.into()
                },
                _ => BridgeMessage::Error { message: "Not implemented".into() }
            };
            serialize_message(&response).unwrap_or_else(|_| "{}".into())
        }
        Err(e) => {
            serialize_message(&BridgeMessage::Error { message: e.to_string() }).unwrap_or_else(|_| "{}".into())
        }
    }
}
