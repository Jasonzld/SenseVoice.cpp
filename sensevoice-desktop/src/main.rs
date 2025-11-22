//! SenseVoice Desktop - 桌面语音识别应用
//!
//! 提供以下功能：
//! - 文件转录
//! - 实时字幕
//! - 语音输入法
//! - 历史记录管理

mod core;
mod engine;
mod platform;
mod utils;

use anyhow::Result;
use log::info;

fn main() -> Result<()> {
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("SenseVoice Desktop v{}", env!("CARGO_PKG_VERSION"));
    info!("Starting application...");

    // TODO: 当 gpui 集成完成后，这里将启动 GUI 应用
    // 现在先运行 CLI 版本进行测试

    println!("╔════════════════════════════════════════╗");
    println!("║      🎙️  SenseVoice Desktop          ║");
    println!("║                                        ║");
    println!("║  高性能语音识别桌面应用                ║");
    println!("╚════════════════════════════════════════╝");
    println!();
    println!("状态: 项目结构已创建");
    println!();
    println!("下一步:");
    println!("  1. 实现 SenseVoice.cpp C++ 绑定");
    println!("  2. 集成 gpui GUI 框架");
    println!("  3. 实现核心业务逻辑");
    println!();
    println!("文档:");
    println!("  - docs/GPUI_IMPLEMENTATION_PLAN.md");
    println!("  - docs/UI_PROTOTYPES_SUMMARY.md");
    println!();

    Ok(())
}
