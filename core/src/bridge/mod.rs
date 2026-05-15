//! JNI/FFI bridge for Android and iOS native UIs
use crate::{FluxResult, FluxError};
use serde::{Deserialize, Serialize};

/// Bridge message types for cross-platform communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeMessage {
    // Terminal
    ExecuteCommand { session_id: String, command: String },
    CommandOutput { session_id: String, stdout: String, stderr: String, exit_code: i32 },
    CreateTab { shell_type: String },
    CloseTab { tab_id: String },
    ResizeTerminal { tab_id: String, rows: u32, cols: u32 },
    // AI
    AiChat { message: String, context: Option<String> },
    AiResponse { content: String },
    AiSetProvider { provider: String, api_key: String, model: String, endpoint: String },
    // Package
    AptCommand { args: Vec<String> },
    PackageResult { output: String, exit_code: i32 },
    // Filesystem
    ListDir { path: String },
    ReadFile { path: String },
    WriteFile { path: String, content: String },
    CreateDir { path: String },
    DeletePath { path: String },
    FileResult { data: String },
    // Config
    GetConfig,
    SetConfig { key: String, value: String },
    ConfigResult { config_json: String },
    // Security
    Unlock { biometric_token: Option<String> },
    Lock,
    StoreSecret { key: String, value: String },
    // Sync
    SyncNow,
    SyncStatus { status_json: String },
    // System
    GetSystemInfo,
    SystemInfoResult { info_json: String },
    // Device Hardware
    DeviceRequest { request_json: String },
    DeviceResponse { response_json: String },
    Error { message: String },
}

/// Serialize a bridge message to JSON for FFI transport
pub fn serialize_message(msg: &BridgeMessage) -> FluxResult<String> {
    serde_json::to_string(msg).map_err(|e| FluxError::Bridge(e.to_string()))
}

/// Deserialize a bridge message from JSON
pub fn deserialize_message(json: &str) -> FluxResult<BridgeMessage> {
    serde_json::from_str(json).map_err(|e| FluxError::Bridge(e.to_string()))
}

use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

// Global instance of the FluxEngine, protected by a Mutex and running on a global Tokio runtime
static ENGINE: OnceLock<Arc<Mutex<crate::FluxEngine>>> = OnceLock::new();
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

/// Helper to get the tokio runtime
fn get_runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build tokio runtime")
    })
}

// ── JNI exports for Android ─────────────────────────────────────────────
#[cfg(feature = "android")]
pub mod android {
    use super::*;
    use jni::JNIEnv;
    use jni::objects::{JClass, JString};
    use jni::sys::jboolean;

    /// JNI entry point: initialize Flux core
    #[no_mangle]
    pub extern "system" fn Java_dev_fluxai_app_bridge_FluxBridge_initialize(
        mut env: JNIEnv, _class: JClass, data_dir: JString,
    ) -> jboolean {
        if ENGINE.get().is_some() {
            return 1; // Already initialized
        }

        let dir: String = match env.get_string(&data_dir) {
            Ok(s) => s.into(),
            Err(_) => return 0,
        };

        let rt = get_runtime();
        let init_result = rt.block_on(async {
            crate::initialize(&dir).await
        });

        match init_result {
            Ok(engine) => {
                let _ = ENGINE.set(Arc::new(Mutex::new(engine)));
                1
            }
            Err(e) => {
                tracing::error!("Failed to initialize FluxEngine: {}", e);
                0
            }
        }
    }

    /// JNI entry point: send message to core
    #[no_mangle]
    pub extern "system" fn Java_dev_fluxai_app_bridge_FluxBridge_sendMessage<'local>(
        mut env: JNIEnv<'local>, _class: JClass, message_json: JString<'local>,
    ) -> JString<'local> {
        let json: String = match env.get_string(&message_json) {
            Ok(s) => s.into(),
            Err(_) => return env.new_string("{\"Error\": {\"message\": \"Invalid JString\"}}").unwrap(),
        };

        let msg = match deserialize_message(&json) {
            Ok(m) => m,
            Err(e) => return env.new_string(format!("{{\"Error\": {{\"message\": \"{}\"}}}}", e)).unwrap(),
        };

        let engine_arc = match ENGINE.get() {
            Some(e) => e,
            None => return env.new_string("{\"Error\": {\"message\": \"Engine not initialized\"}}").unwrap(),
        };

        let rt = get_runtime();
        
        let response = rt.block_on(async {
            let mut engine = engine_arc.lock().await;
            match msg {
                BridgeMessage::ExecuteCommand { session_id, command } => {
                    match engine.execute_command(&session_id, &command).await {
                        Ok(out) => BridgeMessage::CommandOutput {
                            session_id,
                            stdout: out.stdout,
                            stderr: out.stderr,
                            exit_code: out.exit_code,
                        },
                        Err(e) => BridgeMessage::Error { message: e.to_string() },
                    }
                }
                BridgeMessage::AiChat { message, context: _ } => {
                    match engine.ai_chat(&message, None).await {
                        Ok(content) => BridgeMessage::AiResponse { content },
                        Err(e) => BridgeMessage::Error { message: e.to_string() },
                    }
                }
                BridgeMessage::GetSystemInfo => {
                    let info = engine.system_info();
                    BridgeMessage::SystemInfoResult { info_json: serde_json::to_string(&info).unwrap_or_default() }
                }
                BridgeMessage::Unlock { biometric_token } => {
                    if let Some(token) = biometric_token {
                        match engine.unlock_security(&token) {
                            Ok(_) => BridgeMessage::ConfigResult { config_json: "{\"unlocked\": true}".into() },
                            Err(e) => BridgeMessage::Error { message: format!("{}", e) },
                        }
                    } else {
                        BridgeMessage::Error { message: "Biometric token missing".into() }
                    }
                }
                BridgeMessage::Lock => {
                    engine.security.lock();
                    BridgeMessage::ConfigResult { config_json: "{\"locked\": true}".into() }
                }
                BridgeMessage::StoreSecret { key, value } => {
                    match engine.security.store_secret(&key, &value) {
                        Ok(_) => BridgeMessage::ConfigResult { config_json: "{\"stored\": true}".into() },
                        Err(e) => BridgeMessage::Error { message: format!("{}", e) },
                    }
                }
                _ => BridgeMessage::Error { message: "Unhandled message type".into() }
            }
        });

        let res_json = serialize_message(&response).unwrap_or_else(|_| "{}".to_string());
        env.new_string(res_json).unwrap()
    }
}

// ── C exports for iOS FFI ───────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn flux_initialize(data_dir: *const std::os::raw::c_char) -> bool {
    if data_dir.is_null() { return false; }
    true
}

#[no_mangle]
pub extern "C" fn flux_send_message(_json: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let response = "{\"status\": \"ok\"}";
    let c_str = std::ffi::CString::new(response).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn flux_free_string(s: *mut std::os::raw::c_char) {
    if !s.is_null() {
        unsafe { let _ = std::ffi::CString::from_raw(s); }
    }
}
