<p align="center">
  <img src="https://raw.githubusercontent.com/outflex/openflow/main/app-icon.png" width="96" height="96" style="border-radius: 22px; margin-bottom: 12px;" alt="OpenFlow Logo">
</p>

<h1 align="center" style="font-weight: 800; letter-spacing: -0.025em;">OpenFlow</h1>

<p align="center">
  <b style="color: #94a3b8;">Локальный голосовой ассистент для мгновенного ввода текста с фокусом на приватность</b>
</p>

<p align="center">
  <a href="https://github.com/outflex/openflow/releases/tag/v0.1.0-beta"><img src="https://img.shields.io/badge/VERSION-v0.1.0--beta-f43f5e?style=for-the-badge&logoColor=white" alt="Version"></a>
  <img src="https://img.shields.io/badge/PLATFORM-macOS-1e1e24?style=for-the-badge&logo=apple&logoColor=white" alt="Platform">
  <img src="https://img.shields.io/badge/ENGINE-Whisper.cpp-7c3aed?style=for-the-badge&logo=rust&logoColor=white" alt="Engine">
  <img src="https://img.shields.io/badge/STACK-Tauri_v2_%7C_React-0ea5e9?style=for-the-badge&logo=react&logoColor=white" alt="Stack">
</p>

<hr style="border: none; height: 1px; background: #ffffff10; margin: 30px 0;">

# OpenFlow

Локальное десктопное приложение для преобразования речи в текст с фокусом на приватность и мгновенным вводом под курсор. 

**Версия:** v0.1.0-beta  
**Разработчик:** Ron Developer  

## Текущий функционал (v0.1.0-beta)
* **Локальный инференс:** Интеграция `whisper.cpp` через `whisper-rs` (в текущей сборке задействовано ускорение Metal GPU).
* **Глобальный перехват:** Активация записи по системному хоткею (`Cmd + Shift + Space`).
* **Прямая инъекция текста:** Автоматическая эмуляция ввода (`Cmd + V` / `Ctrl + V`) в активное окно пользователя (через CoreGraphics и резервный AppleScript).
* **Интерфейс (HUD):** Динамическое плавающее окно с привязкой к координатам курсора мыши.
* **Анализ аудио:** Реактивный эквалайзер громкости с применением Low-Pass фильтра.
* **Управление контекстом:** Пользовательский словарь для повышения точности специфических терминов.
* **Мультиязычность:** Автоматическое определение языка (с возможностью принудительного выбора RU/EN).
* **Фильтрация артефактов:** Алгоритм подавления системных галлюцинаций Whisper.

## Технологический стек
* **Ядро:** Tauri v2, Rust
* **Интерфейс:** React, TypeScript, Tailwind CSS, Vite

## Заметки по портированию на Windows
В текущей ветке аппаратное ускорение жестко привязано к macOS (Metal). Для корректной компиляции и работы на Windows потребуется перенастроить флаги сборки `whisper-rs` для активации CUDA и заменить системные звуки macOS (`afplay`) на кроссплатформенные аналоги.
