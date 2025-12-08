import { useEffect } from 'react';
import Sidebar from './components/Sidebar';
import ChatContainer from './components/ChatContainer';
import { useChatStore } from './store/chatStore';
import { api } from './services/api';

function App() {
  const { 
    sessionId, 
    setSessionId, 
    setMessages, 
    setSessions,
    clearMessages 
  } = useChatStore();

  useEffect(() => {
    // Ensure session ID is saved
    if (!localStorage.getItem('rust_llm_session_id')) {
      const newId = crypto.randomUUID();
      setSessionId(newId);
    }

    // Load sessions and history
    loadSessions();
    loadHistory();
  }, []);

  const loadSessions = async () => {
    try {
      const sessionIds = await api.getSessions();
      setSessions(sessionIds);
    } catch (error) {
      console.error('Failed to load sessions:', error);
    }
  };

  const loadHistory = async () => {
    try {
      const history = await api.getHistory(sessionId);
      const messages = history.filter((msg) => msg.role !== 'system');
      setMessages(messages as any);
    } catch (error) {
      console.error('Failed to load history:', error);
      setMessages([]);
    }
  };

  const handleNewChat = () => {
    const newId = crypto.randomUUID();
    setSessionId(newId);
    clearMessages();
    loadSessions();
  };

  const handleSwitchSession = async (id: string) => {
    setSessionId(id);
    try {
      const history = await api.getHistory(id);
      const messages = history.filter((msg) => msg.role !== 'system');
      setMessages(messages as any);
    } catch (error) {
      console.error('Failed to load history:', error);
      setMessages([]);
    }
    loadSessions();
  };

  const handleDeleteSession = async (id: string) => {
    try {
      await api.deleteSession(id);
      if (id === sessionId) {
        handleNewChat();
      } else {
        loadSessions();
      }
    } catch (error) {
      console.error('Failed to delete session:', error);
    }
  };

  return (
    <div className="flex h-screen overflow-hidden bg-gray-900 text-gray-100">
      <Sidebar
        onNewChat={handleNewChat}
        onSwitchSession={handleSwitchSession}
        onDeleteSession={handleDeleteSession}
      />
      <ChatContainer />
    </div>
  );
}

export default App;
