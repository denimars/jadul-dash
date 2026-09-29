use std::collections::HashSet;
use std::sync::Arc;

use android_auto::{
    AndroidAutoAudioInputTrait, AndroidAutoAudioOutputTrait, AndroidAutoInputChannelTrait,
    AndroidAutoConfiguration, AndroidAutoMainTrait, AndroidAutoSensorTrait,
    AndroidAutoVideoChannelTrait, AndroidAutoWiredTrait, AudioChannelType, HeadUnitInfo,
    InputConfiguration, SendableAndroidAutoMessage, SensorInformation, VideoConfiguration, Wifi,
};
use async_trait::async_trait;

use crate::DashboardWindow;

// Android Auto session handler.
// Every channel is a stub for now: it only logs what the phone sends.
#[derive(Clone)]
pub struct CarHeadUnitBackend {
    video_config: VideoConfiguration,
    input_config: InputConfiguration,
    sensors: SensorInformation,
    ui: slint::Weak<DashboardWindow>,
}

impl CarHeadUnitBackend {
    pub fn new(ui: slint::Weak<DashboardWindow>) -> Self {
        Self {
            // 800x480 fits inside the 1024x600 dashboard screen
            video_config: VideoConfiguration {
                resolution: Wifi::video_resolution::Enum::_480p,
                fps: Wifi::video_fps::Enum::_30,
                dpi: 140,
            },
            input_config: InputConfiguration {
                keycodes: Vec::new(),
                touchscreen: Some((800, 480)),
            },
            sensors: SensorInformation {
                sensors: HashSet::new(),
            },
            ui,
        }
    }

    // Shows the connection status on the dashboard (safe to call from any thread)
    fn set_status(&self, text: &str, connected: bool) {
        let text = slint::SharedString::from(text);
        let _ = self.ui.upgrade_in_event_loop(move |ui| {
            ui.set_status_text(text);
            ui.set_connected(connected);
        });
    }

    // Waits for a phone on USB and runs one Android Auto session.
    // Loops forever so a new session starts each time the phone is plugged back in.
    pub async fn run_forever(self) {
        let setup = android_auto::setup();
        loop {
            println!("Waiting for an Android device on the USB port...");
            self.set_status("Waiting for USB connection...", false);
            let mut tasks = tokio::task::JoinSet::new();
            let backend = Box::new(self.clone());
            if let Err(e) = backend.run(head_unit_config(), &mut tasks, &setup).await {
                eprintln!("Android Auto session error: {e}");
                self.set_status("Connection error, retrying...", false);
            }
            tasks.join_all().await;
        }
    }
}

// How this head unit introduces itself to the phone
fn head_unit_config() -> AndroidAutoConfiguration {
    AndroidAutoConfiguration {
        unit: HeadUnitInfo {
            name: "car_interface".to_string(),
            car_model: "Unknown".to_string(),
            car_year: "Unknown".to_string(),
            car_serial: "0".to_string(),
            // Indonesia drives on the left, so cars are right-hand drive
            left_hand: false,
            head_manufacturer: "DIY".to_string(),
            head_model: "car_interface".to_string(),
            sw_build: "1".to_string(),
            sw_version: env!("CARGO_PKG_VERSION").to_string(),
            native_media: false,
            hide_clock: None,
        },
        custom_certificate: None,
    }
}

// Marks this head unit as supporting wired (USB) Android Auto
impl AndroidAutoWiredTrait for CarHeadUnitBackend {}

#[async_trait]
impl AndroidAutoMainTrait for CarHeadUnitBackend {
    async fn connect(&self) {
        println!("Android phone connected via USB! Starting session...");
        self.set_status("Android phone connected", true);
    }

    async fn disconnect(&self) {
        println!("Android phone disconnected.");
        self.set_status("Android phone disconnected", false);
    }

    async fn get_receiver(&self) -> Option<tokio::sync::mpsc::Receiver<SendableAndroidAutoMessage>> {
        None
    }

    fn supports_wired(&self) -> Option<Arc<dyn AndroidAutoWiredTrait>> {
        Some(Arc::new(self.clone()))
    }
}

// Receives the video stream (e.g. Google Maps) from the phone
#[async_trait]
impl AndroidAutoVideoChannelTrait for CarHeadUnitBackend {
    async fn receive_video(&self, data: Vec<u8>, _timestamp: Option<u64>) {
        println!("Received video packet: {} bytes", data.len());
    }

    async fn setup_video(&self) -> Result<(), ()> {
        Ok(())
    }

    async fn teardown_video(&self) {}

    async fn wait_for_focus(&self) {}

    async fn set_focus(&self, _focus: bool) {}

    fn retrieve_video_configuration(&self) -> &VideoConfiguration {
        &self.video_config
    }
}

// Audio from the phone (music, navigation voice, system sounds)
#[async_trait]
impl AndroidAutoAudioOutputTrait for CarHeadUnitBackend {
    async fn open_output_channel(&self, _t: AudioChannelType) -> Result<(), ()> {
        Ok(())
    }

    async fn close_output_channel(&self, _t: AudioChannelType) -> Result<(), ()> {
        Ok(())
    }

    async fn receive_output_audio(&self, _t: AudioChannelType, _data: Vec<u8>) {}

    async fn start_output_audio(&self, _t: AudioChannelType) {}

    async fn stop_output_audio(&self, _t: AudioChannelType) {}
}

// Microphone input to the phone (voice assistant, calls)
#[async_trait]
impl AndroidAutoAudioInputTrait for CarHeadUnitBackend {
    async fn open_input_channel(&self) -> Result<(), ()> {
        Ok(())
    }

    async fn close_input_channel(&self) -> Result<(), ()> {
        Ok(())
    }

    async fn start_input_audio(&self) {}

    async fn stop_input_audio(&self) {}

    async fn audio_input_ack(&self, _chan: u8, _ack: Wifi::AVMediaAckIndication) {}
}

// Touchscreen and hardware buttons
#[async_trait]
impl AndroidAutoInputChannelTrait for CarHeadUnitBackend {
    async fn binding_request(&self, _code: u32) -> Result<(), ()> {
        Ok(())
    }

    fn retrieve_input_configuration(&self) -> &InputConfiguration {
        &self.input_config
    }
}

// Car sensors (speed, night mode, GPS, ...); none are reported yet
#[async_trait]
impl AndroidAutoSensorTrait for CarHeadUnitBackend {
    fn get_supported_sensors(&self) -> &SensorInformation {
        &self.sensors
    }

    async fn start_sensor(&self, _stype: Wifi::sensor_type::Enum) -> Result<(), ()> {
        Ok(())
    }
}
