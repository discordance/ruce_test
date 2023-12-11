#![feature(portable_simd)]

mod svf;
use core::simd::f32x2;
use svf::SVFSimper;

use ruce::ffi;
use ruce::ruce_types::{PluginProcessor, PluginProcessorImpl};
pub struct MyProcessorType {
    lpf: SVFSimper
}

impl PluginProcessor for MyProcessorType {
    fn new() -> Self {
        return MyProcessorType {
            // default filter
            lpf: SVFSimper::new(800.0, 0.6, 44100.0)
        };
    }

    // JUCE triggers this
    fn prepare_to_play(&mut self, sample_rate: f32, maximum_expected_samples_per_block: usize) {
        println!(
            "{} {} {}",
            "Rust Audio Processor", sample_rate, maximum_expected_samples_per_block
        );

        // reset the lpf
        self.lpf.set(800.0, 0.6, sample_rate);
    }

    // JUCE calls this function passing a channel interleaved mutable buffer.
    // This callback runs in the audio thread.
    fn process_block(&mut self, audio_data: &mut [f32], num_channels: usize, _num_samples: usize) {
        // process all channels
        for frame in audio_data.chunks_mut(num_channels) {
            // assume stereo frame
            let simd_frame = f32x2::from_slice(frame);
            let lp = self.lpf.process(simd_frame);

            frame.copy_from_slice(&lp.to_array());
        }
    }
}

ruce::ruce_vst3::create_processor!(MyProcessorType);
