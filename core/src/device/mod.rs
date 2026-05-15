//! Device API Integration (Camera, Microphone, Sensors, Location, etc)
pub mod diagnostics;

use crate::{FluxResult, FluxError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceApiRequest {
    /// Capture a photo via camera
    CapturePhoto { high_res: bool },
    /// Record audio from microphone
    RecordAudio { duration_seconds: u32 },
    /// Read text from clipboard
    ReadClipboard,
    /// Write text to clipboard
    WriteClipboard { content: String },
    /// Get current GPS location
    GetLocation,
    /// Trigger local notification
    SendNotification { title: String, message: String },
    /// Trigger biometric authentication prompt
    AuthenticateBiometric { reason: String },
    /// Open native file picker to select a file
    PickFile { mime_type: String },
    /// Read accelerometer and gyroscope
    ReadSensors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceApiResponse {
    PhotoCaptured { path: String, size_bytes: u64 },
    AudioRecorded { path: String, duration_seconds: u32 },
    ClipboardContent { content: String },
    ClipboardWritten,
    LocationData { latitude: f64, longitude: f64, accuracy_meters: f32 },
    NotificationSent,
    BiometricSuccess,
    BiometricFailed { reason: String },
    FilePicked { uri: String, name: String, size_bytes: u64 },
    SensorData { accel_x: f32, accel_y: f32, accel_z: f32, gyro_x: f32, gyro_y: f32, gyro_z: f32 },
}

pub struct DeviceManager {
    // In a real app, this would hold state or FFI callbacks to trigger the native device hardware
    pub permissions_granted: bool,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            permissions_granted: true, // Mocking permission granted for now
        }
    }

    /// Process a device API request. 
    /// In a fully integrated app, this generates a BridgeMessage to the native UI (Android/iOS)
    /// to actually open the camera, prompt location, etc.
    pub fn handle_request(&self, request: DeviceApiRequest) -> FluxResult<DeviceApiResponse> {
        if !self.permissions_granted {
            return Err(FluxError::PermissionDenied("Device permission not granted".into()));
        }

        // Return mocked responses indicating the request was processed
        match request {
            DeviceApiRequest::CapturePhoto { .. } => {
                tracing::info!("Camera capture requested");
                Ok(DeviceApiResponse::PhotoCaptured { path: "/tmp/photo.jpg".into(), size_bytes: 102400 })
            }
            DeviceApiRequest::RecordAudio { duration_seconds } => {
                tracing::info!("Audio recording requested for {}s", duration_seconds);
                Ok(DeviceApiResponse::AudioRecorded { path: "/tmp/audio.m4a".into(), duration_seconds })
            }
            DeviceApiRequest::ReadClipboard => {
                tracing::info!("Read clipboard requested");
                Ok(DeviceApiResponse::ClipboardContent { content: "(mock clipboard content)".into() })
            }
            DeviceApiRequest::WriteClipboard { content } => {
                tracing::info!("Write clipboard requested: {}", content);
                Ok(DeviceApiResponse::ClipboardWritten)
            }
            DeviceApiRequest::GetLocation => {
                tracing::info!("Location requested");
                Ok(DeviceApiResponse::LocationData { latitude: -6.2088, longitude: 106.8456, accuracy_meters: 5.0 })
            }
            DeviceApiRequest::SendNotification { title, message } => {
                tracing::info!("Notification requested: {} - {}", title, message);
                Ok(DeviceApiResponse::NotificationSent)
            }
            DeviceApiRequest::AuthenticateBiometric { reason } => {
                tracing::info!("Biometric requested for: {}", reason);
                Ok(DeviceApiResponse::BiometricSuccess)
            }
            DeviceApiRequest::PickFile { mime_type } => {
                tracing::info!("File picker requested for type: {}", mime_type);
                Ok(DeviceApiResponse::FilePicked { uri: "content://mock/file".into(), name: "document.pdf".into(), size_bytes: 4096 })
            }
            DeviceApiRequest::ReadSensors => {
                tracing::info!("Sensors read requested");
                Ok(DeviceApiResponse::SensorData { accel_x: 0.0, accel_y: 9.81, accel_z: 0.0, gyro_x: 0.0, gyro_y: 0.0, gyro_z: 0.0 })
            }
        }
    }
}
