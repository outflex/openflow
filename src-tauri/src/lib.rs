mod audio;
mod injector;
mod transcriber;

use audio::AudioController;
use parking_lot::Mutex;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use transcriber::Transcriber;

#[cfg(target_os = "macos")]
use core_graphics::event::CGEvent;
#[cfg(target_os = "macos")]
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

#[tauri::command]
fn resize_window(app: tauri::AppHandle, width: f64, height: f64) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize::new(width, height)));
    }
}

#[tauri::command]
fn set_mute(muted: bool, audio_ctrl: tauri::State<AudioController>) {
    audio_ctrl.set_mute(muted);
}

// Новая команда для обновления настроек Whisper
#[tauri::command]
fn set_settings(language: String, prompt: String, transcriber: tauri::State<Arc<Mutex<Transcriber>>>) {
    let mut tr = transcriber.lock();
    tr.language = language;
    tr.custom_prompt = prompt;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let audio_controller = AudioController::new();
    let is_recording_state = Arc::new(AtomicBool::new(false));
    let transcriber = Arc::new(Mutex::new(Transcriber::new()));

    tauri::Builder::default()
        .manage(audio_controller.clone())
        .manage(transcriber.clone()) // Передаем нейросеть в глобальный стейт
        .invoke_handler(tauri::generate_handler![resize_window, set_mute, set_settings])
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let audio_ctrl = audio_controller.clone();
            let is_rec = is_recording_state.clone();
            let transcriber_arc = transcriber.clone();

            if let Some(window) = app_handle.get_webview_window("main") {
                let _ = window.set_shadow(false);
            }

            let app_dir = app.path().app_data_dir().unwrap_or(PathBuf::from("."));
            let _ = fs::create_dir_all(&app_dir);

            let small_model = app_dir.join("ggml-small.bin");
            let base_model = app_dir.join("ggml-base.bin");
            let model_path = if small_model.exists() {
                small_model
            } else {
                base_model
            };

            let transcriber_init = transcriber.clone();
            let app_emit = app_handle.clone();
            thread::spawn(move || {
                if model_path.exists() {
                    let mut tr = transcriber_init.lock();
                    let _ = tr.load_model(&model_path);
                    let _ = app_emit.emit("model-ready", true);
                } else {
                    let _ = app_emit.emit("model-ready", false);
                }
            });

            let shortcut: Shortcut = "CmdOrCtrl+Shift+Space".parse().unwrap();

            app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let currently_recording = is_rec.load(Ordering::SeqCst);

                        if currently_recording {
                            is_rec.store(false, Ordering::SeqCst);
                            let _ = app_handle.emit("recording-status", false);
                            let _ = app_handle.emit("transcribing-status", true);

                            #[cfg(target_os = "macos")]
                            let _ = Command::new("afplay").arg("/System/Library/Sounds/Tink.aiff").spawn();

                            let samples = match audio_ctrl.stop() {
                                Ok(s) => s,
                                Err(e) => {
                                    eprintln!("[OpenFlow Audio] Ошибка остановки: {}", e);
                                    let _ = window.hide();
                                    return;
                                }
                            };

                            let tr_clone = transcriber_arc.clone();
                            let win_clone = window.clone();
                            let emitter = app_handle.clone();

                            thread::spawn(move || {
                                let tr = tr_clone.lock();
                                if tr.is_loaded() {
                                    match tr.transcribe(&samples) {
                                        Ok(text) => {
                                            println!("[OpenFlow Распознано]: {}", text);
                                            let _ = win_clone.hide();

                                            if let Err(e) = injector::paste_text(&text) {
                                                eprintln!("[OpenFlow Paste Error]: {}", e);
                                            }
                                            let _ = emitter.emit("transcription-result", text);
                                        }
                                        Err(e) => {
                                            eprintln!("[OpenFlow Transcribe Error]: {}", e);
                                            let _ = win_clone.hide();
                                        }
                                    }
                                } else {
                                    let _ = win_clone.hide();
                                }
                                let _ = emitter.emit("transcribing-status", false);
                            });
                        } else {
                            position_window_near_mouse(&window);

                            #[cfg(target_os = "macos")]
                            let _ = Command::new("afplay").arg("/System/Library/Sounds/Pop.aiff").spawn();

                            let emitter = app_handle.clone();
                            if let Err(e) = audio_ctrl.start(move |rms| {
                                let _ = emitter.emit("mic-volume", rms);
                            }) {
                                eprintln!("[OpenFlow Audio Error]: {}", e);
                                return;
                            }

                            is_rec.store(true, Ordering::SeqCst);
                            let _ = app_handle.emit("mute-status", false);
                            let _ = window.show();
                            let _ = app_handle.emit("recording-status", true);
                        }
                    }
                }
            })?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn position_window_near_mouse(window: &tauri::WebviewWindow) {
    #[cfg(target_os = "macos")]
    {
        if let Ok(source) = CGEventSource::new(CGEventSourceStateID::CombinedSessionState) {
            if let Ok(event) = CGEvent::new(source) {
                let mouse_loc = event.location();
                let win_w = 340.0;
                let win_h = 80.0;

                let mut target_x = mouse_loc.x + 16.0;
                let mut target_y = mouse_loc.y + 16.0;

                if let Ok(Some(monitor)) = window.current_monitor() {
                    let scale = monitor.scale_factor();
                    let screen_w = monitor.size().width as f64 / scale;
                    let screen_h = monitor.size().height as f64 / scale;

                    if target_x + win_w > screen_w {
                        target_x = mouse_loc.x - win_w - 10.0;
                    }
                    if target_y + win_h > screen_h {
                        target_y = mouse_loc.y - win_h - 10.0;
                    }
                }

                let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(
                    target_x, target_y,
                )));
            }
        }
    }
}