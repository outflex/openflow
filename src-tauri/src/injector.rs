use arboard::Clipboard;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "macos")]
use core_graphics::event::{CGEvent, CGEventFlags, CGKeyCode};
#[cfg(target_os = "macos")]
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

#[cfg(not(target_os = "macos"))]
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

pub fn paste_text(text: &str) -> Result<(), String> {
    let clean_text = text.trim();
    if clean_text.is_empty() {
        return Ok(());
    }

    // 1. Копируем в буфер обмена
    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard error: {}", e))?;
    clipboard
        .set_text(clean_text)
        .map_err(|e| format!("Clipboard set error: {}", e))?;
    println!("[OpenFlow] Текст в буфере: \"{}\"", clean_text);

    // 2. Даем ОС вернуть фокус окну ввода
    thread::sleep(Duration::from_millis(150));

    // 3. Эмулируем ровно ОДНО нажатие Cmd+V
    #[cfg(target_os = "macos")]
    simulate_paste_macos()?;

    #[cfg(not(target_os = "macos"))]
    simulate_paste_other()?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn simulate_paste_macos() -> Result<(), String> {
    const KEY_CODE_V: CGKeyCode = 9;

    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
        .map_err(|_| "Не удалось создать CGEventSource".to_string())?;

    let press_event = CGEvent::new_keyboard_event(source.clone(), KEY_CODE_V, true)
        .map_err(|_| "Ошибка создания нажатия V")?;
    press_event.set_flags(CGEventFlags::CGEventFlagCommand);
    press_event.post(core_graphics::event::CGEventTapLocation::Session);

    thread::sleep(Duration::from_millis(30));

    let release_event = CGEvent::new_keyboard_event(source, KEY_CODE_V, false)
        .map_err(|_| "Ошибка создания отпускания V")?;
    release_event.set_flags(CGEventFlags::CGEventFlagCommand);
    release_event.post(core_graphics::event::CGEventTapLocation::Session);

    println!("[OpenFlow] Одиночный Cmd+V отправлен");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn simulate_paste_other() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| format!("{:?}", e))?;
    let _ = enigo.key(Key::Control, Direction::Press);
    let _ = enigo.key(Key::Unicode('v'), Direction::Click);
    let _ = enigo.key(Key::Control, Direction::Release);
    Ok(())
}