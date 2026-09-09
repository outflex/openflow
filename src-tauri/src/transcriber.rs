use std::path::Path;
use std::sync::Arc;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct Transcriber {
    context: Option<Arc<WhisperContext>>,
    pub language: String,
    pub custom_prompt: String,
}

impl Transcriber {
    pub fn new() -> Self {
        Self { 
            context: None,
            language: "auto".to_string(),
            custom_prompt: "Gemini, OpenFlow, TypeScript, Rust, Tauri, React, деплой, фича, баг".to_string(),
        }
    }

    pub fn load_model<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let mut params = WhisperContextParameters::default();
        params.use_gpu = true;

        let ctx = WhisperContext::new_with_params(path.as_ref().to_str().unwrap(), params)
            .map_err(|e| format!("Ошибка загрузки модели Whisper: {:?}", e))?;

        self.context = Some(Arc::new(ctx));
        println!("[Transcriber] Модель успешно загружена в память");
        Ok(())
    }

    pub fn is_loaded(&self) -> bool {
        self.context.is_some()
    }

    pub fn transcribe(&self, audio_data: &[f32]) -> Result<String, String> {
        let ctx = match &self.context {
            Some(c) => c.clone(),
            None => return Err("Модель не инициализирована".into()),
        };

        let mut state = ctx
            .create_state()
            .map_err(|e| format!("Ошибка создания стейта: {:?}", e))?;

        let mut params = FullParams::new(SamplingStrategy::BeamSearch {
            beam_size: 5,
            patience: -1.0,
        });
        
        params.set_language(Some(&self.language));
        params.set_initial_prompt(&self.custom_prompt);
        
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        params.set_no_speech_thold(0.4);
        params.set_logprob_thold(-0.8);
        params.set_temperature(0.0);
        params.set_suppress_blank(true);

        state
            .full(params, audio_data)
            .map_err(|e| format!("Ошибка инференса: {:?}", e))?;

        let num_segments = state
            .full_n_segments()
            .map_err(|e| format!("Ошибка получения сегментов: {:?}", e))?;

        let mut raw_text = String::new();
        for i in 0..num_segments {
            if let Ok(segment) = state.full_get_segment_text(i) {
                raw_text.push_str(&segment);
            }
        }

        Ok(Self::clean_hallucinations(&raw_text))
    }

    fn clean_hallucinations(text: &str) -> String {
        let mut result = text.to_string();
        
        // Прямая и безопасная замена мусорных тегов (покрывает 99% случаев)
        let exact_blacklist = [
            "[музыка]", "[Музыка]", "(музыка)", "(Музыка)",
            "[звук]", "[Звук]", "(звук)", "(Звук)",
            "[аплодисменты]", "[Аплодисменты]", "(аплодисменты)", "(Аплодисменты)",
            "Подпишись на канал", "подпишись на канал",
            "Ставьте лайки", "ставьте лайки",
            "Спасибо за просмотр", "спасибо за просмотр"
        ];
        
        for phrase in exact_blacklist.iter() {
            result = result.replace(phrase, "");
        }
        
        // Убираем двойные пробелы, если они остались после удаления тегов
        while result.contains("  ") {
            result = result.replace("  ", " ");
        }
        
        result.trim().to_string()
    }
}