<p align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=0:121216,50:f43f5e,100:fb923c&height=220&section=header&text=OpenFlow&fontSize=50&fontColor=ffffff&fontAlignY=38&desc=Local%20Voice-to-Text%20Utility&descSize=15&descColor=ffffff99&animation=fadeIn&fontAlign=62&customColorList=121216,f43f5e,fb923c" width="100%">
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
