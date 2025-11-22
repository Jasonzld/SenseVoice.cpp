//! 批量音频处理示例
//!
//! 用法：
//! ```bash
//! cargo run --example batch --release -- audio_folder/ output_folder/
//! ```

use sensevoice_audio_processor::{batch_process_audio_files, AudioConfig, AudioEncoder};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("用法: {} <输入目录> <输出目录>", args[0]);
        eprintln!("示例: {} audio/ output/", args[0]);
        std::process::exit(1);
    }

    let input_dir = Path::new(&args[1]);
    let output_dir = Path::new(&args[2]);

    // 创建输出目录
    fs::create_dir_all(output_dir)?;

    println!("🎵 批量音频处理器");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("输入目录: {}", input_dir.display());
    println!("输出目录: {}", output_dir.display());
    println!();

    // 扫描音频文件
    let audio_extensions = ["mp3", "wav", "flac", "aac", "ogg", "m4a"];
    let mut audio_files: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    if audio_extensions.contains(&ext_str.to_lowercase().as_str()) {
                        audio_files.push(path);
                    }
                }
            }
        }
    }

    if audio_files.is_empty() {
        eprintln!("❌ 未找到音频文件");
        std::process::exit(1);
    }

    println!("找到 {} 个音频文件", audio_files.len());
    println!();

    // 配置
    let config = AudioConfig {
        target_sample_rate: 16000,
        target_channels: 1,
        use_simd: true,
        normalize: true,
        target_peak: 0.95,
    };

    // 批量处理
    println!("⏳ 批量处理中...");
    let start = Instant::now();

    let results = batch_process_audio_files(&audio_files, &config)?;

    let total_time = start.elapsed();

    // 保存结果
    println!("\n💾 保存结果...");
    for (i, (file_name, audio)) in results.iter().enumerate() {
        let output_path = output_dir.join(format!("{}.wav", file_name.replace('.', "_")));

        AudioEncoder::encode_wav(audio, config.target_sample_rate, 1, &output_path)?;

        println!(
            "  [{}/{}] {} -> {} ({} 样本)",
            i + 1,
            results.len(),
            file_name,
            output_path.display(),
            audio.len()
        );
    }

    // 统计
    println!("\n✅ 批量处理完成！");
    println!();
    println!("统计信息:");
    println!("  处理文件数: {}", results.len());
    println!("  总耗时: {:.2} 秒", total_time.as_secs_f64());
    println!(
        "  平均耗时: {:.2} 秒/文件",
        total_time.as_secs_f64() / results.len() as f64
    );

    let total_samples: usize = results.iter().map(|(_, audio)| audio.len()).sum();
    let total_duration = total_samples as f64 / config.target_sample_rate as f64;

    println!("  总音频时长: {:.2} 秒", total_duration);
    println!(
        "  整体 RTF: {:.4}",
        total_time.as_secs_f64() / total_duration
    );
    println!(
        "  处理速度: {:.1}x 实时",
        total_duration / total_time.as_secs_f64()
    );

    Ok(())
}
