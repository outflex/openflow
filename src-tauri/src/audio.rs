use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

pub enum AudioCommand {
    Start(Box<dyn Fn(f32) + Send + Sync + 'static>),
    Stop(Sender<Vec<f32>>),
    SetMute(bool),
}

#[derive(Clone)]
pub struct AudioController {
    sender: Sender<AudioCommand>,
}

impl AudioController {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = channel::<AudioCommand>();

        thread::spawn(move || {
            run_audio_thread(cmd_rx);
        });

        Self { sender: cmd_tx }
    }

    pub fn start<F>(&self, on_rms: F) -> Result<(), String>
    where
        F: Fn(f32) + Send + Sync + 'static,
    {
        self.sender
            .send(AudioCommand::Start(Box::new(on_rms)))
            .map_err(|e| e.to_string())
    }

    pub fn stop(&self) -> Result<Vec<f32>, String> {
        let (resp_tx, resp_rx) = channel();
        self.sender
            .send(AudioCommand::Stop(resp_tx))
            .map_err(|e| e.to_string())?;

        resp_rx.recv().map_err(|e| e.to_string())
    }

    pub fn set_mute(&self, muted: bool) {
        let _ = self.sender.send(AudioCommand::SetMute(muted));
    }
}

fn run_audio_thread(rx: Receiver<AudioCommand>) {
    let mut current_stream: Option<cpal::Stream> = None;
    let audio_buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let is_recording = Arc::new(Mutex::new(false));
    let is_muted = Arc::new(AtomicBool::new(false));

    while let Ok(cmd) = rx.recv() {
        match cmd {
            AudioCommand::Start(on_rms) => {
                let host = cpal::default_host();
                let device = match host.default_input_device() {
                    Some(d) => d,
                    None => {
                        eprintln!("[Audio Error] Микрофон не найден!");
                        continue;
                    }
                };

                let default_config = match device.default_input_config() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("[Audio Error] Ошибка конфига: {}", e);
                        continue;
                    }
                };

                let channels = default_config.channels() as usize;
                let sample_rate = default_config.sample_rate().0;
                let stream_config: cpal::StreamConfig = default_config.into();

                *is_recording.lock() = true;
                is_muted.store(false, Ordering::SeqCst);
                audio_buffer.lock().clear();

                let is_rec_flag = is_recording.clone();
                let is_muted_flag = is_muted.clone();
                let buf_clone = audio_buffer.clone();

                let stream_res = device.build_input_stream(
                    &stream_config,
                    move |data: &[f32], _| {
                        if !*is_rec_flag.lock() {
                            return;
                        }
                        
                        // Если микрофон замьючен, просто игнорируем входящие данные
                        if is_muted_flag.load(Ordering::SeqCst) {
                            on_rms(0.0); // Сбрасываем эквалайзер в 0
                            return;
                        }
                        
                        process_samples(data, channels, sample_rate, &buf_clone, &on_rms);
                    },
                    |err| eprintln!("[Audio Stream Error]: {}", err),
                    None,
                );

                match stream_res {
                    Ok(stream) => {
                        if stream.play().is_ok() {
                            current_stream = Some(stream);
                        }
                    }
                    Err(e) => eprintln!("[Audio Error] Ошибка создания стрима: {}", e),
                }
            }
            AudioCommand::Stop(resp_tx) => {
                *is_recording.lock() = false;
                drop(current_stream.take());
                let samples = audio_buffer.lock().clone();
                audio_buffer.lock().clear();
                let _ = resp_tx.send(samples);
            }
            AudioCommand::SetMute(muted) => {
                is_muted.store(muted, Ordering::SeqCst);
                println!("[OpenFlow] Микрофон {}", if muted { "отключен (Mute)" } else { "активен" });
            }
        }
    }
}

fn process_samples(
    data: &[f32],
    channels: usize,
    sample_rate: u32,
    buffer: &Arc<Mutex<Vec<f32>>>,
    on_rms: &Box<dyn Fn(f32) + Send + Sync + 'static>,
) {
    if data.is_empty() { return; }

    let mut mono_samples = Vec::with_capacity(data.len() / channels);
    for chunk in data.chunks(channels) {
        let sum: f32 = chunk.iter().sum();
        mono_samples.push(sum / channels as f32);
    }

    let sum_sq: f32 = mono_samples.iter().map(|s| s * s).sum();
    let rms = (sum_sq / mono_samples.len() as f32).sqrt();
    on_rms(rms);

    let resampled = if sample_rate != 16000 {
        let ratio = 16000.0 / sample_rate as f32;
        let target_len = (mono_samples.len() as f32 * ratio) as usize;
        let mut out = Vec::with_capacity(target_len);
        for i in 0..target_len {
            let idx = (i as f32 / ratio) as usize;
            if idx < mono_samples.len() {
                out.push(mono_samples[idx]);
            }
        }
        out
    } else {
        mono_samples
    };

    buffer.lock().extend_from_slice(&resampled);
}