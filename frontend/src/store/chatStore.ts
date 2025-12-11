import { create } from 'zustand';

export interface Message {
  role: 'user' | 'assistant' | 'system';
  content: string;
}

export interface ChatSettings {
  model: string;
  device: string;
  temperature: number;
  topP: number;
  topK: number;
  maxTokens: number;
  repeatPenalty: number;
  systemPrompt: string;
  stopSequences: string;
}

interface ChatState {
  sessionId: string;
  messages: Message[];
  sessions: string[];
  settings: ChatSettings;
  isGenerating: boolean;
  isConnected: boolean;
  tokenCount: number;
  tokensPerSecond: number;
  
  // Actions
  setSessionId: (id: string) => void;
  setMessages: (messages: Message[]) => void;
  addMessage: (message: Message) => void;
  updateLastMessage: (content: string) => void;
  setSessions: (sessions: string[]) => void;
  addSession: (id: string, title?: string) => void;
  updateSessionTitle: (id: string, title: string) => void;
  removeSession: (id: string) => void;
  updateSettings: (settings: Partial<ChatSettings>) => void;
  resetSettings: () => void;
  setIsGenerating: (generating: boolean) => void;
  setIsConnected: (connected: boolean) => void;
  setTokenCount: (count: number) => void;
  setTokensPerSecond: (tps: number) => void;
  clearMessages: () => void;
  saveSessionSettings: (sessionId: string, settings: ChatSettings) => void;
  loadSessionSettings: (sessionId: string) => ChatSettings | null;
}

const defaultSettings: ChatSettings = {
  model: 'Qwen/Qwen2.5-0.5B-Instruct',
  device: 'cpu',
  temperature: 0.7,
  topP: 0.95,
  topK: 40,
  maxTokens: 512,
  repeatPenalty: 1.1,
  systemPrompt: '',
  stopSequences: '',
};

export const useChatStore = create<ChatState>((set) => ({
  sessionId: localStorage.getItem('rust_llm_session_id') || crypto.randomUUID(),
  messages: [],
  sessions: [],
  settings: defaultSettings,
  isGenerating: false,
  isConnected: true,
  tokenCount: 0,
  tokensPerSecond: 0,

  setSessionId: (id) => {
    localStorage.setItem('rust_llm_session_id', id);
    set({ sessionId: id });
  },

  setMessages: (messages) => set({ messages }),
  
  addMessage: (message) => set((state) => ({
    messages: [...state.messages, message],
  })),

  updateLastMessage: (content) => set((state) => {
    const messages = [...state.messages];
    if (messages.length > 0) {
      messages[messages.length - 1] = {
        ...messages[messages.length - 1],
        content,
      };
    }
    return { messages };
  }),

  setSessions: (sessions) => {
    console.log('Store: Setting sessions to', sessions);
    set({ sessions });
  },
  
  addSession: (id) => set((state) => ({ sessions: [id, ...state.sessions.filter(s => s !== id)] })),
  
  updateSessionTitle: () => {}, // No-op for compatibility
  
  removeSession: (id) => set((state) => {
    // Also remove session metadata from localStorage
    try {
      const metadata = localStorage.getItem('rust_llm_session_metadata');
      if (metadata) {
        const parsed = JSON.parse(metadata);
        delete parsed[id];
        localStorage.setItem('rust_llm_session_metadata', JSON.stringify(parsed));
      }
    } catch (e) {
      console.error('Failed to remove session metadata:', e);
    }
    return { sessions: state.sessions.filter(s => s !== id) };
  }),

  updateSettings: (newSettings) => set((state) => {
    const updated = { ...state.settings, ...newSettings };
    // Save settings for current session
    try {
      const metadata = localStorage.getItem('rust_llm_session_metadata');
      const parsed = metadata ? JSON.parse(metadata) : {};
      parsed[state.sessionId] = {
        settings: updated,
        lastUsed: Date.now(),
      };
      localStorage.setItem('rust_llm_session_metadata', JSON.stringify(parsed));
    } catch (e) {
      console.error('Failed to save session metadata:', e);
    }
    return { settings: updated };
  }),

  resetSettings: () => set({ settings: defaultSettings }),

  setIsGenerating: (generating) => set({ isGenerating: generating }),
  
  setIsConnected: (connected) => set({ isConnected: connected }),

  setTokenCount: (count) => set({ tokenCount: count }),

  setTokensPerSecond: (tps) => set({ tokensPerSecond: tps }),

  clearMessages: () => set({ messages: [] }),

  saveSessionSettings: (sessionId: string, settings: ChatSettings) => {
    try {
      const metadata = localStorage.getItem('rust_llm_session_metadata');
      const parsed = metadata ? JSON.parse(metadata) : {};
      parsed[sessionId] = {
        settings,
        lastUsed: Date.now(),
      };
      localStorage.setItem('rust_llm_session_metadata', JSON.stringify(parsed));
    } catch (e) {
      console.error('Failed to save session settings:', e);
    }
  },

  loadSessionSettings: (sessionId: string): ChatSettings | null => {
    try {
      const metadata = localStorage.getItem('rust_llm_session_metadata');
      if (metadata) {
        const parsed = JSON.parse(metadata);
        const sessionData = parsed[sessionId];
        if (sessionData && sessionData.settings) {
          return sessionData.settings;
        }
      }
    } catch (e) {
      console.error('Failed to load session settings:', e);
    }
    return null;
  },
}));
