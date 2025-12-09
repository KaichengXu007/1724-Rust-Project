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
  
  removeSession: (id) => set((state) => ({ sessions: state.sessions.filter(s => s !== id) })),

  updateSettings: (newSettings) => set((state) => ({
    settings: { ...state.settings, ...newSettings },
  })),

  resetSettings: () => set({ settings: defaultSettings }),

  setIsGenerating: (generating) => set({ isGenerating: generating }),
  
  setIsConnected: (connected) => set({ isConnected: connected }),

  setTokenCount: (count) => set({ tokenCount: count }),

  setTokensPerSecond: (tps) => set({ tokensPerSecond: tps }),

  clearMessages: () => set({ messages: [] }),
}));
