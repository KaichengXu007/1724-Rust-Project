import React from 'react';
import { useChatStore } from '../store/chatStore';

interface SidebarProps {
  onNewChat: () => void;
  onSwitchSession: (id: string) => void;
  onDeleteSession: (id: string) => void;
}

const Sidebar: React.FC<SidebarProps> = ({
  onNewChat,
  onSwitchSession,
  onDeleteSession,
}) => {
  const { sessionId, sessions, settings, updateSettings, resetSettings, isConnected, tokenCount, tokensPerSecond, messages } = useChatStore();

  return (
    <aside className="w-64 bg-gray-900 border-r border-gray-800 flex flex-col">
      {/* Header */}
      <div className="p-4 border-b border-gray-800 flex items-center gap-2">
        <div className="w-8 h-8 bg-indigo-500 rounded-lg flex items-center justify-center">
          <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        </div>
        <h1 className="font-semibold text-lg tracking-tight">Rust LLM</h1>
      </div>

      {/* New Chat Button */}
      <div className="p-3 space-y-2">
        <button
          onClick={onNewChat}
          className="w-full flex items-center gap-2 px-4 py-3 bg-gray-800 hover:bg-gray-700 rounded-lg transition-colors text-sm font-medium border border-gray-700"
        >
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          New Chat
        </button>
        <button
          onClick={() => {
            const data = {
              sessionId,
              sessions,
              messages,
              settings,
              exportDate: new Date().toISOString(),
              messageCount: messages.length,
            };
            const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `chat-history-${sessionId.slice(0, 8)}-${Date.now()}.json`;
            a.click();
            URL.revokeObjectURL(url);
          }}
          className="w-full flex items-center gap-2 px-4 py-2 bg-transparent hover:bg-gray-800 rounded-lg transition-colors text-sm border border-gray-700 text-gray-400 hover:text-gray-200"
        >
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
          </svg>
          Export History
        </button>
      </div>

      {/* Session List */}
      <div className="flex-1 overflow-y-auto px-3 py-2 space-y-1">
        {sessions.map((id) => {
          const isActive = id === sessionId;
          return (
            <div
              key={id}
              className={`group relative flex items-center justify-between gap-2 px-3 py-2 rounded-lg text-sm transition-colors cursor-pointer ${
                isActive
                  ? 'bg-gray-800 text-white'
                  : 'bg-transparent text-gray-400 hover:bg-gray-800/50 hover:text-gray-300'
              }`}
              onClick={() => onSwitchSession(id)}
            >
              <span className="flex-1 truncate">
                {id.slice(0, 8)}...
              </span>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  if (confirm('Delete this chat?')) {
                    onDeleteSession(id);
                  }
                }}
                className="opacity-0 group-hover:opacity-100 p-1 rounded text-gray-400 hover:bg-gray-700 hover:text-red-400 transition-all"
                title="Delete"
              >
                <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
          );
        })}
      </div>

      {/* Settings Panel */}
      <div className="p-4 border-t border-gray-800 text-xs text-gray-500 space-y-3">
        {/* Model Selection */}
        <div className="space-y-1">
          <label className="block text-gray-400 font-medium">Model</label>
          <select
            value={settings.model}
            onChange={(e) => updateSettings({ model: e.target.value })}
            className="w-full bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-xs focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all text-white"
          >
            <option value="Qwen/Qwen2.5-0.5B-Instruct">🤖 Qwen 2.5 (0.5B)</option>
            <option value="microsoft/Phi-3.5-mini-instruct">🧠 Phi-3.5 Mini</option>
          </select>
        </div>

        {/* Device Selection */}
        <div className="space-y-1">
          <label className="block text-gray-400 font-medium">Device</label>
          <select
            value={settings.device}
            onChange={(e) => updateSettings({ device: e.target.value })}
            className="w-full bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-xs focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all text-white"
          >
            <option value="cuda">⚡ GPU (CUDA)</option>
            <option value="cpu">💻 CPU</option>
          </select>
        </div>

        {/* Advanced Settings */}
        <details className="group">
          <summary className="cursor-pointer text-gray-400 hover:text-gray-300 font-medium list-none flex items-center justify-between">
            <span>⚙️ Advanced</span>
            <svg className="w-4 h-4 transition-transform group-open:rotate-180" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
            </svg>
          </summary>
          <div className="mt-2 space-y-3 pt-2 border-t border-gray-700 max-h-96 overflow-y-auto">
            {/* Temperature */}
            <div>
              <label className="block text-gray-400 mb-1 text-[11px]">
                Temperature: <span className="text-white font-medium">{settings.temperature}</span>
              </label>
              <input
                type="range"
                min="0"
                max="2"
                step="0.1"
                value={settings.temperature}
                onChange={(e) => updateSettings({ temperature: parseFloat(e.target.value) })}
                className="w-full h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer"
              />
              <p className="text-[10px] text-gray-500 mt-1">Controls randomness (0=deterministic, 2=very random)</p>
            </div>

            {/* Top P */}
            <div>
              <label className="block text-gray-400 mb-1 text-[11px]">
                Top P: <span className="text-white font-medium">{settings.topP}</span>
              </label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.05"
                value={settings.topP}
                onChange={(e) => updateSettings({ topP: parseFloat(e.target.value) })}
                className="w-full h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer"
              />
              <p className="text-[10px] text-gray-500 mt-1">Nucleus sampling threshold</p>
            </div>

            {/* Top K */}
            <div>
              <label className="block text-gray-400 mb-1 text-[11px]">
                Top K: <span className="text-white font-medium">{settings.topK}</span>
              </label>
              <input
                type="range"
                min="1"
                max="100"
                step="1"
                value={settings.topK}
                onChange={(e) => updateSettings({ topK: parseInt(e.target.value) })}
                className="w-full h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer"
              />
              <p className="text-[10px] text-gray-500 mt-1">Limits vocabulary to top K tokens</p>
            </div>

            {/* Max Tokens */}
            <div>
              <label className="block text-gray-400 mb-1 text-[11px]">
                Max Tokens: <span className="text-white font-medium">{settings.maxTokens}</span>
              </label>
              <input
                type="range"
                min="128"
                max="2048"
                step="128"
                value={settings.maxTokens}
                onChange={(e) => updateSettings({ maxTokens: parseInt(e.target.value) })}
                className="w-full h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer"
              />
              <p className="text-[10px] text-gray-500 mt-1">Maximum response length</p>
            </div>

            {/* Repeat Penalty */}
            <div>
              <label className="block text-gray-400 mb-1 text-[11px]">
                Repeat Penalty: <span className="text-white font-medium">{settings.repeatPenalty}</span>
              </label>
              <input
                type="range"
                min="1"
                max="2"
                step="0.1"
                value={settings.repeatPenalty}
                onChange={(e) => updateSettings({ repeatPenalty: parseFloat(e.target.value) })}
                className="w-full h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer"
              />
              <p className="text-[10px] text-gray-500 mt-1">Penalizes token repetition (1=off)</p>
            </div>

            {/* System Prompt */}
            <div>
              <label className="block text-gray-400 mb-1 text-[11px]">System Prompt</label>
              <textarea
                rows={3}
                value={settings.systemPrompt}
                onChange={(e) => updateSettings({ systemPrompt: e.target.value })}
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-2 py-1.5 text-[11px] focus:outline-none focus:ring-1 focus:ring-indigo-500 resize-none text-white"
                placeholder="You are a helpful assistant..."
              />
              <p className="text-[10px] text-gray-500 mt-1">Sets the AI's behavior and role</p>
            </div>

            {/* Stop Sequences */}
            <div>
              <label className="block text-gray-400 mb-1 text-[11px]">Stop Sequences</label>
              <input
                type="text"
                value={settings.stopSequences}
                onChange={(e) => updateSettings({ stopSequences: e.target.value })}
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-2 py-1.5 text-[11px] focus:outline-none focus:ring-1 focus:ring-indigo-500 text-white"
                placeholder="e.g., \n\n, END, ###"
              />
              <p className="text-[10px] text-gray-500 mt-1">Comma-separated stop tokens</p>
            </div>

            {/* Reset Button */}
            <button
              onClick={resetSettings}
              className="w-full px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded-lg text-[11px] text-gray-300 transition-colors"
            >
              Reset to Defaults
            </button>
          </div>
        </details>

        {/* Stats */}
        <div className="pt-2 border-t border-gray-700 flex items-center justify-between text-[10px]">
          <span className="flex items-center gap-1">
            <span className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-500' : 'bg-red-500'}`} />
            <span>{isConnected ? 'Connected' : 'Disconnected'}</span>
          </span>
          {tokenCount > 0 && (
            <span className="pulse-slow">
              {tokenCount} tokens ({tokensPerSecond.toFixed(1)} t/s)
            </span>
          )}
        </div>
      </div>
    </aside>
  );
};

export default Sidebar;
