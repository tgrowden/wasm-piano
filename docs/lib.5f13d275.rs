use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

#[wasm_bindgen]
pub struct Player {
    ctx: AudioContext,
    primary: web_sys::OscillatorNode,
    gain: web_sys::GainNode,
}

impl Drop for Player {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Player, JsValue> {
        let ctx = web_sys::AudioContext::new()?;

        let primary = ctx.create_oscillator()?;

        let gain = ctx.create_gain()?;

        primary.set_type(OscillatorType::Triangle);

        gain.gain().set_value(0.0);

        primary.connect_with_audio_node(&gain)?;

        gain.connect_with_audio_node(&ctx.destination())?;

        primary.start()?;

        Ok(Player { ctx, primary, gain })
    }

    #[wasm_bindgen(js_name = setGain)]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.gain.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn stop(&self) {
        self.set_frequency(0_f32);
        self.set_gain(0_f32);
    }

    #[wasm_bindgen(js_name = setFrequency)]
    pub fn set_frequency(&self, freq: f32) {
        self.primary.frequency().set_value(freq);
    }

    #[wasm_bindgen(js_name = setNote)]
    pub fn set_note(&self, note: u8) {
        let freq = midi_to_freq(note);
        self.set_frequency(freq);
    }
}
