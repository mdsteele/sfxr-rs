extern crate sfxr;
extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

struct Sample {
    generator: sfxr::Generator
}

impl AudioCallback for Sample {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        self.generator.generate(out)
    }
}
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1),
        samples: None
    };

    let mut sample = sfxr::Sample::new();
    sample.mutate();
    let generator = sfxr::Generator::new(sample);

    let device = audio_subsystem.open_playback(None, &desired_spec, |_spec| {
        Sample { generator }
    }).unwrap();

    device.resume();
    std::thread::sleep(Duration::from_millis(1_000));
}
