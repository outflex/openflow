<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=12,14,18,22&height=220&section=header&text=OpenFlow&fontSize=70&fontColor=ff5c5c&fontAlignY=38&desc=Local%20AI%20Voice-to-Text%20Utility%20for%20macOS&descSize=18&descColor=ffffff99&animation=fadeIn&fontAlign=50">
    <img src="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=12,14,18,22&height=220&section=header&text=OpenFlow&fontSize=70&fontColor=ff5c5c&fontAlignY=38&desc=Local%20AI%20Voice-to-Text%20Utility%20for%20macOS&descSize=18&descColor=ffffff99&animation=fadeIn&fontAlign=50" width="100%">
  </picture>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Version-v0.1.0--beta-ff5c5c?style=for-the-badge&logo=appveyor&logoColor=white" alt="Version">
  <img src="https://img.shields.io/badge/Platform-macOS-lightgrey?style=for-the-badge&logo=apple&logoColor=white" alt="Platform">
  <img src="https://img.shields.io/badge/Engine-Whisper.cpp-blueviolet?style=for-the-badge&logo=rust&logoColor=white" alt="Engine">
  <img src="https://img.shields.io/badge/Stack-Tauri%20v2%20%7C%20React-blue?style=for-the-badge&logo=react&logoColor=white" alt="Stack">
</p>

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
