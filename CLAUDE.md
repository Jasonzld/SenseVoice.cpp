# CLAUDE.md - AI Assistant Guide for SenseVoice.cpp

This document provides comprehensive information about the SenseVoice.cpp codebase structure, development workflows, and key conventions for AI assistants working on this project.

## Project Overview

**SenseVoice.cpp** is a C++ implementation of SenseVoice, an audio foundation model with audio understanding capabilities including:
- Automatic Speech Recognition (ASR)
- Language Identification (LID)
- Speech Emotion Recognition (SER)
- Acoustic Event Classification (AEC) / Acoustic Event Detection (AED)

**Key Features:**
- Based on the [ggml](https://github.com/ggerganov/ggml) inference framework
- Supports multiple languages: Chinese, Cantonese, English, Japanese, Korean
- Multi-threaded feature extraction (inspired by kaldi-native-fbank)
- Flash attention decoding support
- Quantization support: Q3, Q4, Q5, Q6, Q8
- Multiple backend support: CPU, Metal (Apple Silicon), BLAS, CUDA, Vulkan, CANN

**Version:** 1.4.0

**Original Model:** [SenseVoice from FunAudioLLM](https://github.com/FunAudioLLM/SenseVoice)

## Repository Structure

```
SenseVoice.cpp/
├── sense-voice/               # Core library
│   └── csrc/                  # C++ source files
│       ├── common.h           # Common definitions, data structures, macros
│       ├── common.cc          # Common utilities implementation
│       ├── sense-voice.h      # Main API header
│       ├── sense-voice.cc     # Main API implementation
│       ├── sense-voice-encoder.{h,cc}    # Encoder implementation
│       ├── sense-voice-decoder.{h,cc}    # Decoder implementation
│       ├── sense-voice-frontend.{h,cc}   # Audio feature extraction
│       ├── sense-voice-cmvn.h            # CMVN (Cepstral Mean Variance Normalization)
│       ├── log-mel-filter-bank.h         # Mel-filterbank feature extraction
│       ├── fftsg.cc                      # FFT implementation
│       ├── silero-vad.{h,cc}            # Voice Activity Detection
│       ├── ThreadPool.h                  # Thread pool for parallel processing
│       ├── main.cc                       # Legacy main (non-streaming)
│       ├── third-party/                  # Third-party dependencies
│       │   └── ggml/                     # GGML inference framework (submodule)
│       └── CMakeLists.txt
├── examples/                  # Example applications
│   ├── zcr_main/              # Non-streaming VAD + SenseVoice (main binary)
│   │   └── main.cc            # Entry point for sense-voice-main
│   ├── stream/                # Streaming ASR with real-time VAD
│   │   └── stream.cc          # Entry point for sense-voice-stream
│   ├── quantize/              # Model quantization tool
│   │   └── quantize.cc        # GGUF quantization implementation
│   ├── common-ggml.{h,cc}     # Common GGML utilities
│   ├── common-sdl.{h,cpp}     # SDL2 audio capture utilities
│   └── CMakeLists.txt
├── scripts/                   # Conversion and utility scripts
│   └── convert-pt-to-gguf.py  # PyTorch to GGUF model converter
├── docs/                      # Documentation
│   └── build.md               # Detailed build instructions
├── cmake/                     # CMake modules
│   └── build-info.cmake       # Build information generation
├── .github/workflows/         # CI/CD workflows
│   ├── build.yaml             # Multi-platform build workflow
│   └── build-debug.yaml       # Debug build workflow
├── CMakeLists.txt             # Root CMake configuration
├── README.md                  # Chinese documentation
├── README-EN.md               # English documentation
└── LICENSE                    # Project license
```

## Architecture

### Core Components

#### 1. Frontend (Feature Extraction)
- **Location:** `sense-voice/csrc/sense-voice-frontend.{h,cc}`
- **Purpose:** Extract mel-filterbank features from audio
- **Key files:**
  - `log-mel-filter-bank.h`: Mel-filterbank implementation
  - `fftsg.cc`: FFT (Fast Fourier Transform) implementation
  - `sense-voice-cmvn.h`: Cepstral Mean Variance Normalization

#### 2. Encoder
- **Location:** `sense-voice/csrc/sense-voice-encoder.{h,cc}`
- **Purpose:** Process audio features through transformer-based encoder
- **Features:**
  - 50 encoder layers
  - 4 attention heads
  - 512 hidden state dimensions
  - 2048 linear units
  - SANM (Streaming Attention with Non-streaming Memory) architecture

#### 3. Decoder
- **Location:** `sense-voice/csrc/sense-voice-decoder.{h,cc}`
- **Purpose:** Generate text output from encoder representations
- **Output:** Transcription with optional language, emotion, and event tags

#### 4. VAD (Voice Activity Detection)
- **Location:** `sense-voice/csrc/silero-vad.{h,cc}`
- **Purpose:** Detect speech segments in audio
- **Implementation:** Based on Silero VAD model
- **Usage:** Non-streaming mode uses model-based VAD; streaming mode uses signal processing

#### 5. GGML Integration
- **Location:** `sense-voice/csrc/third-party/ggml/` (submodule)
- **Purpose:** Tensor operations and backend management
- **Backends:** CPU, Metal, CUDA, Vulkan, CANN, BLAS

### Data Flow

```
Audio (WAV) → Frontend (Mel Features) → CMVN → Encoder → Decoder → Text Output
                                                    ↑
                                              VAD Segments
```

## Build System

### CMake Configuration

**Root CMakeLists.txt** (`/CMakeLists.txt`):
- Project version: 1.4.0
- C++ Standard: C++11
- Build types: Release (default), Debug
- Output directories:
  - Binaries: `build/bin/`
  - Libraries: `build/lib/`

**Build Options:**
```cmake
SENSE_VOICE_BUILD_EXAMPLES     # Build example applications (default: ON)
SENSE_VOICE_BUILD_TESTS        # Build tests (default: OFF)
SENSE_VOICE_CCACHE             # Use ccache if available (default: ON)
SENSE_VOICE_ALL_WARNINGS       # Enable all compiler warnings (default: ON)
SENSE_VOICE_FATAL_WARNINGS     # Enable -Werror flag (default: OFF)

# Backend options
GGML_USE_BLAS                  # Enable BLAS backend
GGML_CUDA                      # Enable CUDA backend
GGML_METAL                     # Enable Metal backend (auto on macOS)
GGML_VULKAN                    # Enable Vulkan backend
GGML_CANN                      # Enable CANN backend (Ascend NPU)

# Sanitizers
SENSE_VOICE_SANITIZE_THREAD    # Thread sanitizer (default: OFF)
SENSE_VOICE_SANITIZE_ADDRESS   # Address sanitizer (default: OFF)
SENSE_VOICE_SANITIZE_UNDEFINED # Undefined sanitizer (default: OFF)
```

### Standard Build Commands

**CPU-only build:**
```bash
git clone https://github.com/lovemefan/SenseVoice.cpp
cd SenseVoice.cpp
git submodule sync && git submodule update --init --recursive
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release .. && make -j 8
```

**CUDA build:**
```bash
mkdir build && cd build
cmake -DGGML_CUDA=ON .. && make -j 8
```

**Metal build (macOS):**
Metal is enabled by default on macOS. To disable:
```bash
cmake -DGGML_METAL=OFF .. && make -j 8
```

**Vulkan build:**
```bash
mkdir build && cd build
cmake -DGGML_VULKAN=ON .. && make -j 8
```

See `docs/build.md` for detailed backend-specific build instructions.

## Executables

### 1. sense-voice-main (Non-streaming ASR)
- **Source:** `examples/zcr_main/main.cc`
- **Binary:** `build/bin/sense-voice-main`
- **Usage:** VAD-based segmentation + batch ASR
- **Parameters:**
  - `-t N`: Thread count (default: 4)
  - `-l LANG`: Language (`auto`, `zh`, `en`, `yue`, `ja`, `ko`)
  - `-m FNAME`: Model path
  - `-f FNAME`: WAV file path (16kHz required)
  - `--min_speech_duration_ms`: VAD min speech duration (default: 250ms)
  - `--max_speech_duration_ms`: VAD max speech duration (default: 15000ms)
  - `--min_silence_duration_ms`: VAD min silence duration (default: 100ms)
  - `-ng`: Disable GPU
  - `-fa`: Enable flash attention
  - `-itn`: Enable Inverse Text Normalization (punctuation)
  - `-prefix`: Show language/emotion/event tags

### 2. sense-voice-stream (Streaming ASR)
- **Source:** `examples/stream/stream.cc`
- **Binary:** `build/bin/sense-voice-stream`
- **Usage:** Real-time audio capture and ASR
- **Dependencies:** SDL2 library
- **Parameters:**
  - `-t N`: Thread count (default: 4)
  - `--chunk_size`: VAD chunk size in ms (default: 100)
  - `-mmc`: Min mute chunks (default: 10)
  - `-mnc`: Max non-mute chunks (default: 80)
  - `--use-vad`: Enable VAD (default: false)
  - `-c ID`: Audio capture device ID (default: -1)
  - Other parameters same as sense-voice-main

### 3. quantize (Model Quantization)
- **Source:** `examples/quantize/quantize.cc`
- **Binary:** `build/bin/quantize`
- **Purpose:** Convert GGUF models to quantized formats (Q4, Q5, Q8, etc.)

## Model Management

### Model Format: GGUF
All models use the GGUF (GGML Universal Format) file format.

### Model Sources
- **HuggingFace:** https://huggingface.co/lovemefan/sense-voice-gguf
- **ModelScope:** https://www.modelscope.cn/models/lovemefan/SenseVoiceGGUF

### Model Conversion

**From PyTorch to GGUF:**
```bash
python scripts/convert-pt-to-gguf.py \
    --model /path/to/SenseVoiceSmall \
    --output /path/to/output.gguf \
    --out_type f32  # or f16
```

**Requirements:**
- Python packages: `torch`, `numpy`, `gguf`, `sentencepiece`, `pyyaml`
- Input: Official SenseVoice PyTorch model with `config.yaml`
- Output: GGUF file ready for inference

**Conversion Process:**
1. Loads model from `model.pt` and VAD from `silero_vad.pt`
2. Reads configuration from `config.yaml`
3. Loads tokenizer from `chn_jpn_yue_eng_ko_spectok.bpe.model`
4. Converts tensors to GGUF format
5. Handles special cases (e.g., `linear_q_k_v` split into Q/K/V)

### Quantization
After converting to GGUF (f32 or f16), quantize using:
```bash
./build/bin/quantize input.gguf output.gguf q4_k
```

Supported quantization types: `q3_k`, `q4_k`, `q5_k`, `q6_k`, `q8_0`

## Code Conventions

### File Naming
- Headers: `.h` extension
- C++ sources: `.cc` or `.cpp` extension
- Class files: `sense-voice-{component}.{h,cc}` (e.g., `sense-voice-encoder.cc`)

### Coding Style
- Use `.clang-format` for formatting (config in root directory)
- Indentation: Typically spaces (check `.clang-format` for specifics)
- Naming conventions:
  - Functions: `snake_case` (e.g., `sense_voice_init`)
  - Structs: `snake_case` with `sense_voice_` prefix
  - Macros: `UPPER_SNAKE_CASE` with `SENSEVOICE_` or `SENSE_VOICE_` prefix

### Logging Macros
Defined in `sense-voice/csrc/common.h`:
```cpp
SENSEVOICE_LOG_ERROR(...)   // Error messages
SENSEVOICE_LOG_WARN(...)    // Warnings
SENSEVOICE_LOG_INFO(...)    // Info messages
SENSEVOICE_LOG_DEBUG(...)   // Debug messages (only if SENSEVOICE_DEBUG defined)
```

### Assertions
```cpp
SENSEVOICE_ASSERT(condition)  // Asserts with file/line info
```

### Memory Management
- Use GGML's memory management for tensors
- Backend allocation handled by `ggml-backend.h`
- Thread pool defined in `ThreadPool.h` for parallel processing

## Development Workflow

### Typical Development Tasks

#### 1. Adding a New Feature
1. Identify the component (frontend, encoder, decoder)
2. Add implementation in `sense-voice/csrc/`
3. Update headers if adding new public APIs
4. Update examples if needed
5. Test with both quantized and non-quantized models
6. Update documentation

#### 2. Fixing a Bug
1. Reproduce the issue
2. Check relevant logs (use `SENSEVOICE_LOG_DEBUG`)
3. Fix in the appropriate component
4. Test with multiple models and backends
5. Verify no regression in other features

#### 3. Optimizing Performance
- Profile with different thread counts
- Test with different backends (Metal, CUDA, CPU)
- Compare quantized vs. non-quantized models
- Check memory usage and buffer sizes

#### 4. Adding Backend Support
1. Add backend-specific CMake options
2. Implement backend initialization in encoder/decoder
3. Test with model loading and inference
4. Update `docs/build.md` with build instructions
5. Add to backend support table in README

### Git Workflow

**Branches:**
- `main`: Stable branch
- Feature branches: Use descriptive names (e.g., `feature/flash-attention`, `fix/vad-segmentation`)

**Commit Messages:**
- Use clear, descriptive messages
- Reference issues if applicable
- Example: "Fix VAD segmentation for short audio files (#104)"

**Recent Development:**
- Recent work includes merging VAD parameters and streaming optimizations
- Memory optimizations: removing unused computation graph parts
- Stream input for file reading

## Testing

### Manual Testing
Test both binaries with various inputs:

```bash
# Test non-streaming mode
./build/bin/sense-voice-main \
    -m models/sense-voice-small-q4_k.gguf \
    audio.wav -t 4 -l auto -itn -prefix

# Test streaming mode (requires SDL2)
./build/bin/sense-voice-stream \
    -m models/sense-voice-small-q4_k.gguf \
    -t 4 -l auto --use-vad -itn
```

### Test Cases
- Different languages: Chinese, English, Japanese, Korean, Cantonese
- Different audio lengths: short (<1s), medium (5-10s), long (>30s)
- Different sample rates: 16kHz (native), resampled
- Different backends: CPU, Metal, CUDA, Vulkan
- Different quantization levels: f32, f16, q4_k, q8_0

### Debugging
- Enable debug logging: Define `SENSEVOICE_DEBUG` in `common.h`
- Use sanitizers for memory issues:
  ```bash
  cmake -DSENSE_VOICE_SANITIZE_ADDRESS=ON ..
  ```
- Check GGML backend logs for GPU issues
- Monitor thread performance with `-t 1` vs `-t N`

## Common Pitfalls

### 1. Submodule Issues
**Problem:** GGML submodule not initialized
**Solution:**
```bash
git submodule sync && git submodule update --init --recursive
```

### 2. Audio Format
**Problem:** Only 16kHz audio is supported
**Solution:** Resample audio to 16kHz before processing
```bash
ffmpeg -i input.wav -ar 16000 output.wav
```

### 3. Model Loading Failures
**Problem:** Model file not found or corrupted
**Solution:**
- Verify model path is correct
- Check model was properly converted from PyTorch
- Ensure GGUF file is not corrupted (re-download if needed)

### 4. GPU Backend Not Working
**Problem:** GPU not being used despite CUDA/Metal build
**Solution:**
- Check if `-ng` flag is set (this disables GPU)
- Verify backend initialization logs
- Check environment variables (e.g., `CUDA_VISIBLE_DEVICES`)

### 5. Performance Issues
**Problem:** Slow inference
**Solution:**
- Use quantized models (q4_k recommended)
- Enable flash attention with `-fa`
- Increase thread count with `-t`
- Use GPU backend if available
- Check for unnecessary debug logging

## Important API Functions

### Initialization
```cpp
// Initialize context with parameters
struct sense_voice_context * sense_voice_small_init_from_file_with_params(
    const char * path_model,
    struct sense_voice_context_params params
);

// Get default parameters
struct sense_voice_context_params sense_voice_context_default_params();
```

### Inference
```cpp
// Full parallel inference (non-streaming)
int sense_voice_full_parallel(
    struct sense_voice_context * ctx,
    sense_voice_full_params &params,
    std::vector<double> &samples,
    int n_samples,
    int n_processors
);

// Batch processing
int sense_voice_batch_pcmf(
    struct sense_voice_context *ctx,
    const sense_voice_full_params &params,
    std::vector<std::vector<float>> &pcmf32,
    size_t max_batch_len=90000,
    size_t max_batch_cnt=1,
    bool use_prefix=true,
    bool use_itn=true
);
```

### Output
```cpp
// Print recognition results
void sense_voice_print_output(
    struct sense_voice_context * ctx,
    bool need_prefix,
    bool use_itn,
    bool refresh_self=false
);
```

### Language Support
```cpp
// Get language ID from string
int sense_voice_lang_id(const char * lang);

// Get language string from ID
const char * sense_voice_lang_str(int id);
```

Supported languages: `zh`, `en`, `yue`, `ja`, `ko`, `auto`

## Key Data Structures

### Model Parameters (from GGUF)
- `n_vocab`: 25055 tokens
- `n_encoder_hidden_state`: 512
- `n_encoder_linear_units`: 2048
- `n_encoder_attention_heads`: 4
- `n_encoder_layers`: 50
- `n_mels`: 80

### Audio Parameters
- Sample rate: 16000 Hz
- Frame length: 400 samples (25ms)
- Frame shift: 160 samples (10ms)
- LFR (Low Frame Rate) parameters: m=7, n=6

## Dependencies

### Build Dependencies
- CMake >= 3.12
- C++11 compatible compiler (GCC, Clang, MSVC)
- Backend-specific:
  - CUDA Toolkit (for CUDA backend)
  - Vulkan SDK (for Vulkan backend)
  - Metal (automatic on macOS)
  - SDL2 (for streaming example)

### Python Dependencies (for model conversion)
```
torch
numpy
gguf
sentencepiece
pyyaml
```

### Third-party Code
- **GGML:** Inference framework (submodule)
- **Kaldi Native FBank:** Feature extraction algorithms (referenced)
- **Silero VAD:** Voice activity detection model
- **FunASR:** Model structure and algorithms (reference)
- **Whisper.cpp:** Code structure inspiration

## Performance Expectations

### Typical RTF (Real-Time Factor) on M1 MacBook Pro
- Model: sense-voice-small-q4_k.gguf
- Threads: 1
- RTF: ~0.018-0.02 (50x faster than real-time)
- 5.5s audio processes in ~0.1s

### Optimization Tips
1. Use quantized models (q4_k recommended for balance)
2. Enable GPU backend when available
3. Adjust thread count based on CPU cores
4. Use flash attention for longer sequences
5. Batch process multiple files when possible

## AI Assistant Guidelines

### When Modifying Code

1. **Always read before editing:** Use the Read tool on files before making changes
2. **Respect the architecture:** Keep frontend, encoder, decoder, and VAD components separate
3. **Follow existing patterns:** Match the coding style and naming conventions
4. **Test thoroughly:** Changes should be tested with multiple models and backends
5. **Update documentation:** Keep README and docs in sync with code changes

### When Adding Features

1. **Check existing implementations:** Look at similar features in the codebase first
2. **Consider all backends:** Ensure changes work with CPU, Metal, CUDA, etc.
3. **Think about quantization:** Features should work with quantized models
4. **Add appropriate logging:** Use the logging macros for debugging
5. **Update examples:** Add usage examples if adding user-facing features

### When Debugging

1. **Enable debug logging:** Define `SENSEVOICE_DEBUG` for detailed logs
2. **Check GGML backend:** Many issues are backend-related
3. **Test with f32 model:** Quantization can hide bugs
4. **Isolate components:** Test frontend, encoder, decoder separately if possible
5. **Compare with reference:** Check against FunASR implementation when stuck

### Common Questions to Ask

- Does this change affect model compatibility?
- Will this work with all supported backends?
- Does this impact performance significantly?
- Is this change backward compatible?
- Should this be configurable via parameters?

## References

- **Project Repository:** https://github.com/lovemefan/SenseVoice.cpp
- **Original Model:** https://github.com/FunAudioLLM/SenseVoice
- **GGML Framework:** https://github.com/ggerganov/ggml
- **Whisper.cpp (inspiration):** https://github.com/ggerganov/whisper.cpp
- **FunASR:** https://github.com/alibaba-damo-academy/FunASR
- **Kaldi Native FBank:** https://github.com/csukuangfj/kaldi-native-fbank

## Contact

For issues and contributions, please visit:
https://github.com/lovemefan/SenseVoice.cpp/issues

---

**Last Updated:** 2025-11-22
**Version:** 1.4.0
**Maintained for:** Claude Code AI Assistant
