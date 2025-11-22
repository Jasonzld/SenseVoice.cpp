//! 音频转换示例
//!
//! 用法：
//! ```bash
//! cargo run --example convert --release -- input.mp3 output.wav
//! ```

use sensevoice_audio_processor::{process_audio_file, AudioConfig, AudioEncoder};
use std::env;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("用法: {} <输入文件> <输出文件>", args[0]);
        eprintln!("示例: {} audio.mp3 output.wav", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    println!("🎵 音频转换器");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("输入文件: {}", input_path);
    println!("输出文件: {}", output_path);
    println!();

    // 配置
    let config = AudioConfig {
        target_sample_rate: 16000,
        target_channels: 1,
        use_simd: true,
        normalize: true,
        target_peak: 0.95,
    };

    println!("处理配置:");
    println!("  目标采样率: {} Hz", config.target_sample_rate);
    println!("  目标声道数: {}", config.target_channels);
    println!("  SIMD 加速: {}", if config.use_simd { "启用" } else { "禁用" });
    println!("  音量归一化: {}", if config.normalize { "启用" } else { "禁用" });
    println!();

    // 处理音频
    println!("⏳ 处理中...");
    let start = Instant::now();

    let audio = process_audio_file(input_path, &config)?;

    let process_time = start.elapsed();

    // 保存结果
    AudioEncoder::encode_wav(&audio, config.target_sample_rate, 1, output_path)?;

    let audio_duration = audio.len() as f64 / config.target_sample_rate as f64;
    let rtf = process_time.as_secs_f64() / audio_duration;

    println!("✅ 转换完成！");
    println!();
    println!("统计信息:");
    println!("  样本数: {}", audio.len());
    println!("  音频时长: {:.2} 秒", audio_duration);
    println!("  处理时间: {:.2} 秒", process_time.as_secs_f64());
    println!("  实时因子 (RTF): {:.4}", rtf);
    println!("  处理速度: {:.1}x 实时", 1.0 / rtf);

    Ok(())
}
