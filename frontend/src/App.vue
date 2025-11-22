<template>
  <div id="app" :class="{ dark: isDarkMode }">
    <div class="app-container">
      <!-- 顶部导航栏 -->
      <header class="app-header">
        <div class="header-left">
          <img src="./assets/logo.svg" alt="SenseVoice" class="logo" />
          <h1 class="app-title">SenseVoice Desktop</h1>
        </div>
        <div class="header-right">
          <el-badge :value="processingCount" :hidden="processingCount === 0">
            <el-button :icon="DocumentCopy" circle />
          </el-badge>
          <el-button :icon="Setting" circle @click="showSettings = true" />
          <el-button
            :icon="isDarkMode ? Sunny : Moon"
            circle
            @click="toggleTheme"
          />
        </div>
      </header>

      <!-- 主标签页 -->
      <el-tabs v-model="activeTab" class="main-tabs">
        <el-tab-pane label="📁 文件转录" name="file">
          <FileTranscription />
        </el-tab-pane>
        <el-tab-pane label="🎙️ 实时字幕" name="live">
          <LiveCaption />
        </el-tab-pane>
        <el-tab-pane label="⌨️ 语音输入" name="input">
          <VoiceInput />
        </el-tab-pane>
      </el-tabs>

      <!-- 底部状态栏 -->
      <footer class="app-footer">
        <div class="status-info">
          <span>模型: {{ currentModel }}</span>
          <el-divider direction="vertical" />
          <span>内存: {{ memoryUsage }} MB</span>
          <el-divider direction="vertical" />
          <span>GPU: {{ gpuStatus }}</span>
        </div>
      </footer>

      <!-- 设置对话框 -->
      <SettingsDialog v-model="showSettings" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { DocumentCopy, Setting, Sunny, Moon } from '@element-plus/icons-vue'
import FileTranscription from './views/FileTranscription.vue'
import LiveCaption from './views/LiveCaption.vue'
import VoiceInput from './views/VoiceInput.vue'
import SettingsDialog from './components/SettingsDialog.vue'
import { useAppStore } from './store/app'

const appStore = useAppStore()
const activeTab = ref('file')
const showSettings = ref(false)
const isDarkMode = ref(false)

const processingCount = computed(() => appStore.processingFiles.length)
const currentModel = computed(() => appStore.config.model || 'SenseVoice-Q4K')
const memoryUsage = computed(() => appStore.stats.memoryUsage)
const gpuStatus = computed(() => appStore.stats.gpuEnabled ? '启用' : '禁用')

const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value
  document.documentElement.classList.toggle('dark', isDarkMode.value)
}

onMounted(() => {
  appStore.loadConfig()
})
</script>

<style lang="scss">
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

#app {
  font-family: 'Inter', 'PingFang SC', 'Microsoft YaHei', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  width: 100vw;
  height: 100vh;
  overflow: hidden;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--el-bg-color);
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 20px;
  background: var(--el-bg-color-overlay);
  border-bottom: 1px solid var(--el-border-color);
  user-select: none;
  -webkit-app-region: drag; // Tauri 窗口拖动

  .header-left {
    display: flex;
    align-items: center;
    gap: 12px;

    .logo {
      width: 32px;
      height: 32px;
    }

    .app-title {
      font-size: 18px;
      font-weight: 600;
      color: var(--el-text-color-primary);
    }
  }

  .header-right {
    display: flex;
    gap: 8px;
    -webkit-app-region: no-drag; // 按钮可点击
  }
}

.main-tabs {
  flex: 1;
  overflow: hidden;
  padding: 0 20px;

  :deep(.el-tabs__content) {
    height: calc(100vh - 180px);
    overflow-y: auto;
  }
}

.app-footer {
  padding: 8px 20px;
  background: var(--el-bg-color-overlay);
  border-top: 1px solid var(--el-border-color);

  .status-info {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 12px;
    color: var(--el-text-color-secondary);
  }
}

// 暗色模式
.dark {
  --el-bg-color: #1a1a1a;
  --el-bg-color-overlay: #242424;
}
</style>
