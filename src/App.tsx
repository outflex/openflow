import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { Mic, Loader2, Settings2, MicOff, BookOpen, SlidersHorizontal } from "lucide-react";

export default function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [isTranscribing, setIsTranscribing] = useState(false);
  const [volume, setVolume] = useState(0);
  
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [isMuted, setIsMuted] = useState(false);
  const [activeTab, setActiveTab] = useState<'settings' | 'help'>('settings');

  const [language, setLanguage] = useState(localStorage.getItem('of_lang') || 'auto');
  const [prompt, setPrompt] = useState(localStorage.getItem('of_prompt') || 'React, Tailwind, баг, фича, деплой');

  useEffect(() => {
    localStorage.setItem('of_lang', language);
    localStorage.setItem('of_prompt', prompt);
    invoke('set_settings', { language, prompt });
  }, [language, prompt]);

  useEffect(() => {
    const unlistenStatus = listen<boolean>("recording-status", (event) => {
      setIsRecording(event.payload);
      if (!event.payload) {
        setVolume(0);
        setIsMuted(false);
      }
    });

    const unlistenTranscribing = listen<boolean>("transcribing-status", (event) => {
      setIsTranscribing(event.payload);
    });

    const unlistenVolume = listen<number>("mic-volume", (event) => {
      const rawRms = event.payload;
      const amplified = Math.min(Math.sqrt(rawRms) * 3.5, 1.0);
      setVolume((prev) => prev * 0.4 + amplified * 0.6);
    });

    return () => {
      unlistenStatus.then((fn) => fn());
      unlistenTranscribing.then((fn) => fn());
      unlistenVolume.then((fn) => fn());
    };
  }, []);

  const toggleSettings = async () => {
    try {
      if (!isSettingsOpen) {
        // Делаем системное окно с запасом по высоте (320px)
        await invoke("resize_window", { width: 340, height: 320 });
        setIsSettingsOpen(true);
      } else {
        setIsSettingsOpen(false);
        setTimeout(async () => {
          await invoke("resize_window", { width: 340, height: 80 });
          setActiveTab('settings');
        }, 200); 
      }
    } catch (e) {
      console.error("Ошибка изменения размера:", e);
    }
  };

  const toggleMute = async () => {
    const newState = !isMuted;
    setIsMuted(newState);
    if (newState) setVolume(0);
    await invoke("set_mute", { muted: newState });
  };

  return (
    <div className="w-screen h-screen flex flex-col p-2 bg-transparent select-none">
      <div
        className={`w-full bg-[#121216]/95 backdrop-blur-2xl border border-white/10 flex flex-col transition-all duration-200 overflow-hidden ${
          isSettingsOpen ? "rounded-[24px] h-[300px]" : "rounded-[40px] h-[64px]"
        }`}
      >
        {/* Верхняя панель HUD */}
        <div data-tauri-drag-region className="flex items-center justify-between px-5 h-[64px] shrink-0 cursor-grab active:cursor-grabbing">
          <div className="flex items-center gap-3 pointer-events-none">
            <div className="relative flex items-center justify-center">
              {isRecording && !isMuted && (
                <span className="absolute inline-flex h-full w-full rounded-full bg-rose-500 opacity-75 animate-ping duration-1000" />
              )}
              <div
                className={`w-8 h-8 rounded-full flex items-center justify-center transition-colors duration-300 ${
                  isMuted 
                    ? "bg-white/10 text-white/40"
                    : isTranscribing
                    ? "bg-amber-500/20 text-amber-400"
                    : "bg-gradient-to-tr from-rose-500/20 to-orange-500/20 text-rose-400"
                }`}
              >
                {isTranscribing ? (
                  <Loader2 size={18} strokeWidth={1.5} className="animate-spin" />
                ) : isMuted ? (
                  <MicOff size={18} strokeWidth={1.5} />
                ) : (
                  <Mic size={18} strokeWidth={1.5} />
                )}
              </div>
            </div>
            <div className="flex flex-col">
              <span className="text-xs font-medium text-white/90 tracking-wide transition-colors">
                {isTranscribing ? "Расшифровка..." : isMuted ? "Пауза записи..." : isRecording ? "Запись речи..." : "OpenFlow"}
              </span>
              <span className="text-[10px] text-white/40">Cmd + Shift + Space</span>
            </div>
          </div>

          <div className="flex items-center gap-2">
            <div className={`flex items-center gap-1 h-6 pointer-events-none w-6 transition-opacity duration-200 ${isMuted ? 'opacity-20' : 'opacity-100'}`}>
              {isTranscribing ? (
                <div className="flex gap-1">
                  <span className="w-1.5 h-1.5 rounded-full bg-rose-400 animate-bounce" />
                  <span className="w-1.5 h-1.5 rounded-full bg-rose-400 animate-bounce [animation-delay:0.2s]" />
                  <span className="w-1.5 h-1.5 rounded-full bg-rose-400 animate-bounce [animation-delay:0.4s]" />
                </div>
              ) : (
                [0.5, 0.9, 0.4].map((baseMultiplier, i) => {
                  const phase = Math.sin(volume * 15 + i) * 0.3;
                  const dynamicMultiplier = Math.max(0, baseMultiplier + phase);
                  const barHeight = isRecording && !isMuted ? 4 + volume * dynamicMultiplier * 20 : 4;

                  return (
                    <div
                      key={i}
                      className="w-1 bg-gradient-to-t from-orange-400 to-rose-400 rounded-full transition-[height] duration-150 ease-out"
                      style={{ height: `${Math.min(24, Math.max(4, barHeight))}px` }}
                    />
                  );
                })
              )}
            </div>

            <button
              onClick={toggleMute}
              className={`p-1.5 rounded-full transition-colors cursor-pointer z-50 ml-1 ${
                isMuted ? "bg-rose-500/20 text-rose-400" : "hover:bg-white/5 text-white/40 hover:text-white/80"
              }`}
            >
              <MicOff size={16} strokeWidth={1.5} />
            </button>

            <button
              onClick={toggleSettings}
              className={`p-1.5 rounded-full transition-colors cursor-pointer z-50 ${
                isSettingsOpen ? "bg-white/10 text-white" : "hover:bg-white/5 text-white/40 hover:text-white/80"
              }`}
            >
              <Settings2 size={16} strokeWidth={1.5} />
            </button>
          </div>
        </div>

        {/* Раскрывающаяся панель (Настройки / Справка) */}
        <div className={`px-5 pb-4 transition-opacity duration-200 delay-75 flex flex-col h-full ${isSettingsOpen ? "opacity-100" : "opacity-0"}`}>
          
          <div className="flex gap-4 mb-4 border-b border-white/5 pb-2">
            <button 
              onClick={() => setActiveTab('settings')}
              className={`flex items-center gap-1.5 text-xs font-medium transition-colors ${activeTab === 'settings' ? 'text-white' : 'text-white/40 hover:text-white/70'}`}
            >
              <SlidersHorizontal size={12} /> Настройки
            </button>
            <button 
              onClick={() => setActiveTab('help')}
              className={`flex items-center gap-1.5 text-xs font-medium transition-colors ${activeTab === 'help' ? 'text-white' : 'text-white/40 hover:text-white/70'}`}
            >
              <BookOpen size={12} /> Справка
            </button>
          </div>
          
          {activeTab === 'settings' && (
            <div className="space-y-4 animate-in fade-in zoom-in-95 duration-200">
              <div className="flex justify-between items-center">
                <span className="text-xs text-white/60">Язык распознавания</span>
                <select 
                  value={language}
                  onChange={(e) => setLanguage(e.target.value)}
                  className="bg-white/5 text-white text-xs px-2 py-1 outline-none rounded-md border border-white/10 cursor-pointer hover:bg-white/10 transition-colors"
                >
                  <option value="auto">Авто (Мультиязычный)</option>
                  <option value="ru">Русский (ru)</option>
                  <option value="en">Английский (en)</option>
                </select>
              </div>
              <div className="flex flex-col gap-1.5">
                <span className="text-xs text-white/60">Свой словарь (через запятую)</span>
                <textarea 
                  value={prompt}
                  onChange={(e) => setPrompt(e.target.value)}
                  className="bg-[#0A0A0C] text-white/90 text-[11px] p-2 rounded-md border border-white/5 outline-none resize-none h-[60px] placeholder:text-white/20 focus:border-rose-500/30 transition-colors"
                  placeholder="React, Tailwind, баг, деплой, фича..."
                />
              </div>
            </div>
          )}

          {activeTab === 'help' && (
            // Добавлен pb-4, чтобы текст не прилипал к низу
            <div className="space-y-3 animate-in fade-in zoom-in-95 duration-200 flex flex-col h-full overflow-y-auto pr-1 pb-4">
              <div className="flex gap-2 items-start">
                <span className="text-rose-400 text-xs mt-0.5">🎙️</span>
                <p className="text-[11px] text-white/70 leading-relaxed">Нажмите <strong className="text-white/90 font-medium">Cmd + Shift + Space</strong> и начните говорить. OpenFlow появится прямо возле курсора.</p>
              </div>
              <div className="flex gap-2 items-start">
                <span className="text-rose-400 text-xs mt-0.5">⌨️</span>
                <p className="text-[11px] text-white/70 leading-relaxed">Завершите речь повторным нажатием хоткея. Текст <strong className="text-white/90 font-medium">автоматически напечатается</strong> в активное окно и сохранится в буфере.</p>
              </div>
              <div className="flex gap-2 items-start">
                <span className="text-rose-400 text-xs mt-0.5">🤫</span>
                <p className="text-[11px] text-white/70 leading-relaxed">Клик по микрофону справа <strong className="text-white/90 font-medium">приостановит запись</strong>, чтобы вы могли настроить словарь или сделать паузу.</p>
              </div>
            </div>
          )}

        </div>
      </div>
    </div>
  );
}