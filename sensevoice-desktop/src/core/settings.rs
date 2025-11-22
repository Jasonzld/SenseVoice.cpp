//! 设置管理模块
//!
//! 负责应用配置的加载、保存和管理

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用设置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    /// 快捷键设置
    pub hotkeys: HotkeySettings,
    /// 模型设置
    pub model: ModelSettings,
    /// LLM 设置
    pub llm: LLMSettings,
    /// 输出设置
    pub output: OutputSettings,
}

/// 快捷键设置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotkeySettings {
    /// 录音快捷键
    pub record: String,
    /// 语音输入快捷键
    pub voice_input: String,
    /// 实时字幕快捷键
    pub realtime_subtitle: String,
}

/// 模型设置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelSettings {
    /// 模型文件路径
    pub model_path: PathBuf,
    /// 模型类型
    pub model_type: ModelType,
    /// 是否使用 GPU
    pub use_gpu: bool,
    /// 线程数 (None 表示自动)
    pub num_threads: Option<usize>,
}

/// 模型类型
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModelType {
    Q3K,
    Q4K,
    Q8,
    FP16,
}

/// LLM 设置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LLMSettings {
    /// 是否启用
    pub enabled: bool,
    /// 提供商
    pub provider: LLMProvider,
    /// API 端点
    pub api_endpoint: String,
    /// API 密钥
    pub api_key: String,
    /// 模型名称
    pub model_name: String,
}

/// LLM 提供商
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LLMProvider {
    OpenAI,
    Anthropic,
    Ollama,
    Custom,
}

/// 输出设置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputSettings {
    /// 输出目录
    pub output_dir: PathBuf,
    /// 完成后自动打开
    pub auto_open: bool,
    /// 输出格式
    pub formats: Vec<OutputFormat>,
    /// 包含时间戳
    pub include_timestamps: bool,
}

/// 输出格式
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum OutputFormat {
    /// SRT 字幕
    SRT,
    /// ASS 字幕
    ASS,
    /// VTT 字幕
    VTT,
    /// 纯文本
    TXT,
    /// Markdown
    Markdown,
    /// JSON
    JSON,
}

impl Settings {
    /// 加载设置
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            match std::fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(settings) => {
                        log::info!("Settings loaded from {}", config_path.display());
                        return settings;
                    }
                    Err(e) => {
                        log::warn!("Failed to parse settings: {}, using defaults", e);
                    }
                },
                Err(e) => {
                    log::warn!("Failed to read settings: {}, using defaults", e);
                }
            }
        }

        Self::default()
    }

    /// 保存设置
    pub fn save(&self) -> anyhow::Result<()> {
        let config_dir = Self::config_dir();
        std::fs::create_dir_all(&config_dir)?;

        let config_path = Self::config_path();
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;

        log::info!("Settings saved to {}", config_path.display());
        Ok(())
    }

    /// 获取配置目录
    fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sensevoice")
    }

    /// 获取配置文件路径
    fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hotkeys: HotkeySettings {
                record: "Ctrl+Shift+Space".to_string(),
                voice_input: "Ctrl+Alt+V".to_string(),
                realtime_subtitle: "Ctrl+Shift+S".to_string(),
            },
            model: ModelSettings {
                model_path: PathBuf::from("/models/sensevoice-q4k.gguf"),
                model_type: ModelType::Q4K,
                use_gpu: true,
                num_threads: None,
            },
            llm: LLMSettings {
                enabled: false,
                provider: LLMProvider::OpenAI,
                api_endpoint: "https://api.openai.com/v1".to_string(),
                api_key: String::new(),
                model_name: "gpt-4-turbo".to_string(),
            },
            output: OutputSettings {
                output_dir: dirs::document_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("SenseVoice/Output"),
                auto_open: true,
                formats: vec![OutputFormat::SRT, OutputFormat::TXT],
                include_timestamps: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.hotkeys.record, "Ctrl+Shift+Space");
        assert_eq!(settings.model.use_gpu, true);
        assert_eq!(settings.output.auto_open, true);
    }

    #[test]
    fn test_serialize_deserialize() {
        let settings = Settings::default();
        let toml_str = toml::to_string(&settings).unwrap();
        let deserialized: Settings = toml::from_str(&toml_str).unwrap();

        assert_eq!(settings.hotkeys.record, deserialized.hotkeys.record);
    }
}
