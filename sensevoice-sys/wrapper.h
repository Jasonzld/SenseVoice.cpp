#ifndef SENSEVOICE_WRAPPER_H
#define SENSEVOICE_WRAPPER_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// 前向声明
typedef struct SenseVoiceContext SenseVoiceContext;

/**
 * 创建 SenseVoice 上下文
 *
 * @param model_path 模型文件路径
 * @param num_threads 推理线程数 (0 表示自动检测)
 * @param use_gpu 是否使用 GPU 加速 (1=是, 0=否)
 * @return 成功返回上下文指针，失败返回 NULL
 */
SenseVoiceContext* sensevoice_create_context(
    const char* model_path,
    int num_threads,
    int use_gpu
);

/**
 * 销毁 SenseVoice 上下文
 *
 * @param ctx 上下文指针
 */
void sensevoice_free_context(SenseVoiceContext* ctx);

/**
 * 转录片段
 */
typedef struct {
    char* text;          // 文本内容 (需要调用者释放)
    float start_time;    // 开始时间 (秒)
    float end_time;      // 结束时间 (秒)
} SenseVoiceSegment;

/**
 * 转录结果
 */
typedef struct {
    SenseVoiceSegment* segments;  // 片段数组
    size_t num_segments;          // 片段数量
    char* language;               // 检测到的语言 (需要调用者释放)
} SenseVoiceResult;

/**
 * 转录音频
 *
 * @param ctx 上下文指针
 * @param audio_data 音频数据 (f32, 16kHz, mono)
 * @param audio_length 音频样本数
 * @return 转录结果，使用后需调用 sensevoice_free_result 释放
 */
SenseVoiceResult* sensevoice_transcribe(
    SenseVoiceContext* ctx,
    const float* audio_data,
    size_t audio_length
);

/**
 * 释放转录结果
 *
 * @param result 结果指针
 */
void sensevoice_free_result(SenseVoiceResult* result);

/**
 * 获取最后一次错误信息
 *
 * @return 错误信息字符串 (内部缓冲区，不需要释放)
 */
const char* sensevoice_get_last_error(void);

#ifdef __cplusplus
}
#endif

#endif // SENSEVOICE_WRAPPER_H
