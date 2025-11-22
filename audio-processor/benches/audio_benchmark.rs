use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sensevoice_audio_processor::simd::SimdProcessor;

fn bench_i16_to_f32(c: &mut Criterion) {
    let sizes = [1024, 4096, 16384, 65536];

    let mut group = c.benchmark_group("i16_to_f32");

    for size in sizes {
        let input: Vec<i16> = (0..size).map(|i| (i % 32768) as i16).collect();

        // 标量实现
        group.bench_with_input(BenchmarkId::new("Scalar", size), &input, |b, input| {
            b.iter(|| {
                let output: Vec<f32> =
                    black_box(input.iter().map(|&x| x as f32 / 32768.0).collect());
                output
            })
        });

        // SIMD 实现
        group.bench_with_input(BenchmarkId::new("SIMD", size), &input, |b, input| {
            b.iter(|| SimdProcessor::i16_to_f32(black_box(input)))
        });
    }

    group.finish();
}

fn bench_stereo_to_mono(c: &mut Criterion) {
    let sizes = [2048, 8192, 32768, 131072];

    let mut group = c.benchmark_group("stereo_to_mono");

    for size in sizes {
        let stereo: Vec<f32> = (0..size).map(|i| (i as f32 % 100.0) / 100.0).collect();

        // 标量实现
        group.bench_with_input(BenchmarkId::new("Scalar", size), &stereo, |b, stereo| {
            b.iter(|| {
                let mut mono = Vec::with_capacity(stereo.len() / 2);
                for chunk in stereo.chunks_exact(2) {
                    mono.push((chunk[0] + chunk[1]) * 0.5);
                }
                black_box(mono)
            })
        });

        // SIMD 实现
        group.bench_with_input(BenchmarkId::new("SIMD", size), &stereo, |b, stereo| {
            b.iter(|| SimdProcessor::stereo_to_mono(black_box(stereo)))
        });
    }

    group.finish();
}

fn bench_normalize(c: &mut Criterion) {
    let sizes = [1024, 4096, 16384, 65536];

    let mut group = c.benchmark_group("normalize");

    for size in sizes {
        let audio: Vec<f32> = (0..size).map(|i| (i as f32 % 100.0) / 100.0 - 0.5).collect();

        // 标量实现
        group.bench_with_input(BenchmarkId::new("Scalar", size), &audio, |b, audio| {
            b.iter(|| {
                let mut data = audio.clone();
                let peak = data.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
                if peak > 0.0 {
                    let scale = 0.95 / peak;
                    for sample in &mut data {
                        *sample *= scale;
                    }
                }
                black_box(data)
            })
        });

        // SIMD 实现
        group.bench_with_input(BenchmarkId::new("SIMD", size), &audio, |b, audio| {
            b.iter(|| {
                let mut data = audio.clone();
                SimdProcessor::normalize(black_box(&mut data), 0.95);
                data
            })
        });
    }

    group.finish();
}

fn bench_find_peak(c: &mut Criterion) {
    let sizes = [1024, 4096, 16384, 65536];

    let mut group = c.benchmark_group("find_peak");

    for size in sizes {
        let audio: Vec<f32> = (0..size).map(|i| (i as f32 % 100.0) / 100.0 - 0.5).collect();

        // 标量实现
        group.bench_with_input(BenchmarkId::new("Scalar", size), &audio, |b, audio| {
            b.iter(|| {
                black_box(audio.iter().map(|x| x.abs()).fold(0.0f32, f32::max))
            })
        });

        // SIMD 实现
        group.bench_with_input(BenchmarkId::new("SIMD", size), &audio, |b, audio| {
            b.iter(|| SimdProcessor::find_peak(black_box(audio)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_i16_to_f32,
    bench_stereo_to_mono,
    bench_normalize,
    bench_find_peak
);
criterion_main!(benches);
