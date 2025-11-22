<template>
  <div class="file-transcription">
    <!-- 文件上传区域 -->
    <el-upload
      ref="uploadRef"
      class="upload-area"
      drag
      multiple
      :auto-upload="false"
      :on-change="handleFileChange"
      :show-file-list="false"
      accept="audio/*,video/*,.mp3,.wav,.flac,.m4a,.mp4,.avi,.mkv"
    >
      <el-icon class="upload-icon"><Upload /></el-icon>
      <div class="upload-text">
        <p class="primary">🎬 拖拽文件到此处或点击选择</p>
        <p class="secondary">
          支持格式: MP4, AVI, MKV, MP3, WAV, FLAC, AAC, OGG, M4A
        </p>
      </div>
      <template #tip>
        <div class="upload-actions">
          <el-button type="primary" :icon="FolderOpened" @click="selectFiles">
            📂 选择文件
          </el-button>
          <el-button type="primary" plain :icon="Folder" @click="selectFolder">
            📂 选择文件夹
          </el-button>
        </div>
      </template>
    </el-upload>

    <!-- 文件列表 -->
    <div class="file-list" v-if="files.length > 0">
      <div class="list-header">
        <h3>已添加文件列表 ({{ files.length }})</h3>
        <div class="actions">
          <el-button size="small" @click="clearCompleted">清除已完成</el-button>
          <el-button size="small" type="danger" @click="clearAll">全部清除</el-button>
        </div>
      </div>

      <el-table :data="files" stripe max-height="300">
        <el-table-column prop="name" label="文件名" min-width="200">
          <template #default="{ row }">
            <div class="file-name">
              <el-icon :class="getFileIcon(row.type)">
                <component :is="getFileIconComponent(row.type)" />
              </el-icon>
              {{ row.name }}
            </div>
          </template>
        </el-table-column>

        <el-table-column prop="duration" label="时长" width="100" />

        <el-table-column label="状态" width="200">
          <template #default="{ row }">
            <div v-if="row.status === 'waiting'" class="status-waiting">
              ○ 等待中
            </div>
            <div v-else-if="row.status === 'processing'" class="status-processing">
              <el-progress
                :percentage="row.progress"
                :stroke-width="6"
                :show-text="true"
              />
            </div>
            <div v-else-if="row.status === 'completed'" class="status-completed">
              ✓ 已完成
            </div>
            <div v-else-if="row.status === 'error'" class="status-error">
              ✗ 失败: {{ row.error }}
            </div>
          </template>
        </el-table-column>

        <el-table-column label="操作" width="150">
          <template #default="{ row }">
            <el-button
              v-if="row.status === 'completed'"
              size="small"
              type="primary"
              link
              @click="viewResult(row)"
            >
              📄 查看
            </el-button>
            <el-button
              size="small"
              type="danger"
              link
              @click="removeFile(row.id)"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- 识别选项 -->
    <div class="options-panel">
      <h3>识别选项</h3>
      <el-form :model="options" label-width="100px" size="default">
        <el-row :gutter="20">
          <el-col :span="6">
            <el-form-item label="语言">
              <el-select v-model="options.language">
                <el-option label="自动检测" value="auto" />
                <el-option label="中文" value="zh" />
                <el-option label="英文" value="en" />
                <el-option label="粤语" value="yue" />
                <el-option label="日语" value="ja" />
                <el-option label="韩语" value="ko" />
              </el-select>
            </el-form-item>
          </el-col>

          <el-col :span="6">
            <el-form-item label="模型">
              <el-select v-model="options.model">
                <el-option label="Q4K (推荐)" value="q4k" />
                <el-option label="Q8 (高质量)" value="q8" />
                <el-option label="FP16 (最高质量)" value="fp16" />
              </el-select>
            </el-form-item>
          </el-col>

          <el-col :span="6">
            <el-form-item label="线程数">
              <el-select v-model="options.threads">
                <el-option :label="`${i} 线程`" :value="i" v-for="i in 8" :key="i" />
              </el-select>
            </el-form-item>
          </el-col>

          <el-col :span="6">
            <el-form-item label="GPU 加速">
              <el-switch v-model="options.useGpu" />
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>
    </div>

    <!-- 输出选项 -->
    <div class="output-panel">
      <h3>输出选项</h3>
      <el-form :model="outputOptions" label-width="100px">
        <el-form-item label="输出格式">
          <el-checkbox-group v-model="outputOptions.formats">
            <el-checkbox label="srt">字幕文件 (SRT)</el-checkbox>
            <el-checkbox label="txt">纯文本 (TXT)</el-checkbox>
            <el-checkbox label="json">JSON 格式</el-checkbox>
            <el-checkbox label="timestamp">包含时间戳</el-checkbox>
          </el-checkbox-group>
        </el-form-item>

        <el-form-item label="文本处理">
          <el-checkbox-group v-model="outputOptions.processing">
            <el-checkbox label="correction">错别字修正</el-checkbox>
            <el-checkbox label="punctuation">自动标点</el-checkbox>
            <el-checkbox label="llm">在线 LLM 优化</el-checkbox>
          </el-checkbox-group>
        </el-form-item>

        <el-form-item label="输出目录">
          <el-input
            v-model="outputOptions.directory"
            placeholder="/Users/xxx/Documents/transcripts"
            readonly
          >
            <template #append>
              <el-button :icon="FolderOpened" @click="selectOutputDirectory">
                选择
              </el-button>
            </template>
          </el-input>
        </el-form-item>
      </el-form>
    </div>

    <!-- 开始转录按钮 -->
    <div class="action-panel">
      <el-button
        type="primary"
        size="large"
        :loading="isProcessing"
        :disabled="files.length === 0"
        @click="startBatchTranscription"
      >
        🚀 {{ isProcessing ? '转录中...' : '开始批量转录' }}
      </el-button>
    </div>

    <!-- 结果查看对话框 -->
    <el-dialog
      v-model="showResultDialog"
      title="转录结果"
      width="800px"
      :close-on-click-modal="false"
    >
      <div class="result-content" v-if="currentResult">
        <el-tabs v-model="resultTab">
          <el-tab-pane label="文本" name="text">
            <el-input
              v-model="currentResult.text"
              type="textarea"
              :rows="15"
              placeholder="转录文本"
            />
          </el-tab-pane>
          <el-tab-pane label="时间轴" name="timeline" v-if="currentResult.segments">
            <div class="timeline">
              <div
                v-for="(seg, idx) in currentResult.segments"
                :key="idx"
                class="timeline-item"
              >
                <span class="time">{{ formatTime(seg.start) }}</span>
                <span class="text">{{ seg.text }}</span>
              </div>
            </div>
          </el-tab-pane>
          <el-tab-pane label="统计" name="stats">
            <el-descriptions :column="2" border>
              <el-descriptions-item label="文件名">
                {{ currentResult.fileName }}
              </el-descriptions-item>
              <el-descriptions-item label="时长">
                {{ currentResult.duration }}
              </el-descriptions-item>
              <el-descriptions-item label="语言">
                {{ currentResult.language }}
              </el-descriptions-item>
              <el-descriptions-item label="字数">
                {{ currentResult.text?.length || 0 }}
              </el-descriptions-item>
              <el-descriptions-item label="处理时间">
                {{ currentResult.processTime }}s
              </el-descriptions-item>
              <el-descriptions-item label="置信度">
                {{ (currentResult.confidence * 100).toFixed(1) }}%
              </el-descriptions-item>
            </el-descriptions>
          </el-tab-pane>
        </el-tabs>
      </div>

      <template #footer>
        <el-button @click="showResultDialog = false">关闭</el-button>
        <el-button type="primary" @click="copyResult">📋 复制文本</el-button>
        <el-button type="primary" @click="exportResult">💾 导出文件</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import { Upload, FolderOpened, Folder, VideoCamera, Headset } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'

interface FileItem {
  id: string
  name: string
  path: string
  type: 'audio' | 'video'
  duration: string
  status: 'waiting' | 'processing' | 'completed' | 'error'
  progress: number
  error?: string
  result?: any
}

const uploadRef = ref()
const files = ref<FileItem[]>([])
const isProcessing = ref(false)
const showResultDialog = ref(false)
const currentResult = ref<any>(null)
const resultTab = ref('text')

const options = reactive({
  language: 'auto',
  model: 'q4k',
  threads: 4,
  useGpu: true,
})

const outputOptions = reactive({
  formats: ['srt', 'txt', 'timestamp'],
  processing: ['correction', 'punctuation'],
  directory: '/Users/xxx/Documents/transcripts',
})

const handleFileChange = (file: any) => {
  const fileItem: FileItem = {
    id: Math.random().toString(36),
    name: file.name,
    path: file.path || file.raw.path,
    type: file.name.match(/\.(mp4|avi|mkv)$/i) ? 'video' : 'audio',
    duration: '00:00:00',
    status: 'waiting',
    progress: 0,
  }
  files.value.push(fileItem)
}

const selectFiles = async () => {
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: 'Media',
        extensions: ['mp3', 'wav', 'flac', 'm4a', 'mp4', 'avi', 'mkv', 'aac', 'ogg'],
      },
    ],
  })

  if (selected) {
    const paths = Array.isArray(selected) ? selected : [selected]
    paths.forEach((path) => {
      handleFileChange({ name: path.split('/').pop(), path })
    })
  }
}

const selectFolder = async () => {
  const selected = await open({
    directory: true,
  })

  if (selected && typeof selected === 'string') {
    try {
      const filePaths = await invoke<string[]>('scan_directory', { path: selected })
      filePaths.forEach((path) => {
        handleFileChange({ name: path.split('/').pop(), path })
      })
    } catch (error) {
      ElMessage.error('扫描目录失败')
    }
  }
}

const selectOutputDirectory = async () => {
  const selected = await open({
    directory: true,
  })

  if (selected && typeof selected === 'string') {
    outputOptions.directory = selected
  }
}

const getFileIcon = (type: string) => {
  return type === 'video' ? 'video-icon' : 'audio-icon'
}

const getFileIconComponent = (type: string) => {
  return type === 'video' ? VideoCamera : Headset
}

const startBatchTranscription = async () => {
  isProcessing.value = true

  for (const file of files.value) {
    if (file.status !== 'waiting') continue

    file.status = 'processing'
    file.progress = 0

    try {
      // 调用 Tauri 后端
      const result = await invoke('transcribe_file', {
        filePath: file.path,
        options: {
          language: options.language,
          model: options.model,
          threads: options.threads,
          useGpu: options.useGpu,
        },
      })

      file.status = 'completed'
      file.progress = 100
      file.result = result

      ElMessage.success(`${file.name} 转录完成`)
    } catch (error: any) {
      file.status = 'error'
      file.error = error.message || '转录失败'
      ElMessage.error(`${file.name} 转录失败: ${file.error}`)
    }
  }

  isProcessing.value = false
}

const viewResult = (file: FileItem) => {
  currentResult.value = file.result
  showResultDialog.value = true
}

const removeFile = (id: string) => {
  files.value = files.value.filter((f) => f.id !== id)
}

const clearCompleted = () => {
  files.value = files.value.filter((f) => f.status !== 'completed')
}

const clearAll = () => {
  files.value = []
}

const formatTime = (seconds: number) => {
  const h = Math.floor(seconds / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  const s = Math.floor(seconds % 60)
  return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`
}

const copyResult = () => {
  if (currentResult.value?.text) {
    navigator.clipboard.writeText(currentResult.value.text)
    ElMessage.success('已复制到剪贴板')
  }
}

const exportResult = async () => {
  // TODO: 实现导出功能
  ElMessage.info('导出功能开发中...')
}
</script>

<style lang="scss" scoped>
.file-transcription {
  padding: 20px;

  .upload-area {
    margin-bottom: 30px;

    :deep(.el-upload-dragger) {
      width: 100%;
      height: 200px;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      border: 2px dashed var(--el-border-color);
      border-radius: 8px;
      transition: all 0.3s;

      &:hover {
        border-color: var(--el-color-primary);
        background: var(--el-fill-color-light);
      }
    }

    .upload-icon {
      font-size: 48px;
      color: var(--el-text-color-secondary);
      margin-bottom: 16px;
    }

    .upload-text {
      text-align: center;

      .primary {
        font-size: 16px;
        font-weight: 500;
        color: var(--el-text-color-primary);
        margin-bottom: 8px;
      }

      .secondary {
        font-size: 12px;
        color: var(--el-text-color-secondary);
      }
    }

    .upload-actions {
      display: flex;
      gap: 12px;
      justify-content: center;
      margin-top: 16px;
    }
  }

  .file-list {
    margin-bottom: 30px;

    .list-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 16px;

      h3 {
        font-size: 16px;
        font-weight: 600;
      }

      .actions {
        display: flex;
        gap: 8px;
      }
    }

    .file-name {
      display: flex;
      align-items: center;
      gap: 8px;

      .video-icon {
        color: #f56c6c;
      }

      .audio-icon {
        color: #409eff;
      }
    }

    .status-waiting {
      color: var(--el-text-color-secondary);
    }

    .status-processing {
      min-width: 150px;
    }

    .status-completed {
      color: var(--el-color-success);
      font-weight: 500;
    }

    .status-error {
      color: var(--el-color-danger);
      font-size: 12px;
    }
  }

  .options-panel,
  .output-panel {
    background: var(--el-fill-color-light);
    padding: 20px;
    border-radius: 8px;
    margin-bottom: 20px;

    h3 {
      font-size: 14px;
      font-weight: 600;
      margin-bottom: 16px;
    }
  }

  .action-panel {
    text-align: center;

    .el-button {
      padding: 12px 48px;
      font-size: 16px;
    }
  }

  .result-content {
    .timeline {
      max-height: 400px;
      overflow-y: auto;

      .timeline-item {
        display: flex;
        gap: 16px;
        padding: 8px 0;
        border-bottom: 1px solid var(--el-border-color-lighter);

        .time {
          min-width: 80px;
          color: var(--el-text-color-secondary);
          font-family: monospace;
        }

        .text {
          flex: 1;
        }
      }
    }
  }
}
</style>
