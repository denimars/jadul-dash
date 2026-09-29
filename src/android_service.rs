use android_auto::{
    AndroidAutoMainTrait, AndroidAutoVideoChannelTrait, 
    AndroidAutoInputChannelTrait, AndroidAutoAudioOutputTrait
};
use async_trait::async_trait;

// Android Auto session handler
struct CarHeadUnitBackend;

#[async_trait]
impl AndroidAutoMainTrait for CarHeadUnitBackend {
    async fn on_connect(&mut self) {
        println!("Android phone connected via USB! Starting session...");
    }

    async fn on_disconnect(&mut self) {
        println!("Android phone disconnected.");
    }
}

// Receives the video stream (e.g. Google Maps) from the phone
impl AndroidAutoVideoChannelTrait for CarHeadUnitBackend {
    fn on_video_packet(&mut self, data: &[u8]) {
        println!("Received video packet: {} bytes", data.len());
    }
}