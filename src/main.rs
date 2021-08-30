#![deny(clippy::all)]
#![forbid(unsafe_code)]

extern crate rusticnes_core;
extern crate cpal;
extern crate ringbuf;

use std::rc::Rc;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event as WinitEvent, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use rusticnes_core::palettes::NTSC_PAL;
use rusticnes_ui_common::application::RuntimeState;
use rusticnes_ui_common::events::Event;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::RingBuffer;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 240;

pub fn dispatch_event(runtime: &mut RuntimeState, event: Event) -> Vec<Event> {
    let mut responses: Vec<Event> = Vec::new();
    responses.extend(runtime.handle_event(event.clone()));
    return responses;
  }

  pub fn resolve_events(runtime: &mut RuntimeState, mut events: Vec<Event>) {
    while events.len() > 0 {
      let event = events.remove(0);
      let responses = dispatch_event(runtime, event);
      events.extend(responses);
    }
  }

  pub fn load_rom(runtime: &mut RuntimeState, cart_data: &[u8]) {
    let mut events: Vec<Event> = Vec::new();
    let bucket_of_nothing: Vec<u8> = Vec::new();
    let cartridge_data = cart_data.to_vec();
    events.push(Event::LoadCartridge("cartridge".to_string(), Rc::new(cartridge_data), Rc::new(bucket_of_nothing)));
    resolve_events(runtime, events);
  }

  pub fn run_until_vblank(runtime: &mut RuntimeState) {
    let mut events: Vec<Event> = Vec::new();
    events.push(Event::NesRunFrame);
    resolve_events(runtime, events);
  }

  pub fn render_screen_pixels(runtime: &mut RuntimeState, frame: &mut [u8]) {
    let nes = &runtime.nes;

    for x in 0 .. 256 {
      for y in 0 .. 240 {
        let palette_index = ((nes.ppu.screen[y * 256 + x]) as usize) * 3;
        let pixel_offset = (y * 256 + x) * 4;
        frame[pixel_offset + 0] = NTSC_PAL[palette_index + 0];
        frame[pixel_offset + 1] = NTSC_PAL[palette_index + 1];
        frame[pixel_offset + 2] = NTSC_PAL[palette_index + 2];
        frame[((y * 256 + x) * 4) + 3] = 255;

      }
    }
  }

  pub fn set_audio_samplerate(runtime: &mut RuntimeState, sample_rate: u32) {
    let nes = &mut runtime.nes;
    nes.apu.set_sample_rate(sample_rate as u64);
  }

  pub fn set_audio_buffersize(runtime: &mut RuntimeState, buffer_size: u32) {
    let nes = &mut runtime.nes;
    nes.apu.set_buffer_size(buffer_size as usize);
  }

  pub fn audio_buffer_full(runtime: &mut RuntimeState) -> bool {
    let nes = &runtime.nes;
    return nes.apu.buffer_full;
  }

  pub fn get_audio_buffer(runtime: &mut RuntimeState) -> Vec<i16> {
    let nes = &mut runtime.nes;
    nes.apu.buffer_full = false;
    return nes.apu.output_buffer.to_owned();
  }

  fn main() -> Result<(), Error> {
    env_logger::init();
    let host = cpal::default_host();

    let output_device = host.default_output_device().expect("Could not get default output device");
    let input_device = host.default_input_device().expect("Could not get default input device");
    println!("Output device: {}", output_device.name().unwrap());
    println!("Iutput device: {}", input_device.name().unwrap());
    let output_config = output_device.default_output_config().unwrap();
    println!("Default output config: {:?}", output_config);
    let input_config = input_device.default_input_config().unwrap();
    println!("Default input config: {:?}", input_config);
    
    match output_config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&output_device, &output_config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&output_device, &output_config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&output_device, &output_config.into()),
    }
}

pub fn run<T>(output_device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), Error>
where
T: cpal::Sample,
{
    let mut runtime: RuntimeState = RuntimeState::new();
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    const LATENCY: f32 = 1000.0;
    // Create a delay in case the input and output devices aren't synced.
    let latency_frames = (LATENCY / 1_000.0) * sample_rate as f32;
    let latency_samples = latency_frames as usize * channels as usize;
    
    use std::fs;
    load_rom(&mut runtime, fs::read("rom.nes").expect("Could not read ROM rom.nes").as_slice());
    set_audio_samplerate(&mut runtime, sample_rate as u32*2); //TODO: Why * 2??
    set_audio_buffersize(&mut runtime, (latency_samples / 2) as u32);
    
    
    println!("TED: {:?},{:?}", latency_frames, latency_samples);

    // The buffer to share samples
    let ring = RingBuffer::<f32>::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        producer.push(0.0).unwrap();
    }
    
    let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        
        for sample in data {
            *sample = match consumer.pop() {
                Some(s) => s,
                None => {
                    input_fell_behind = true;
                    0.0
                }
            };
        }
        if input_fell_behind {
            eprintln!("input stream fell behind: try increasing latency");
        }
    };
    let output_stream = output_device.build_output_stream(&config, output_data_fn, err_fn).expect("Could not build sound output stream");

    output_stream.play().expect("Could not start playing output stream");

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let WinitEvent::RedrawRequested(_) = event {
            //println!("render");
            render_screen_pixels(&mut runtime, pixels.get_frame());

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            run_until_vblank(&mut runtime);
            //println!("request redraw");
            window.request_redraw();

            if audio_buffer_full(&mut runtime) {
                let audio_buffer = get_audio_buffer(&mut runtime);
                for e in audio_buffer {
                    producer.push(e as f32/ 32768.0).expect("Could not push the audio buffer"); //TODO: Why / 32768.0?
                }
                //producer.push_slice(audio_buffer.as_slice());
            }

        }
    });
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
