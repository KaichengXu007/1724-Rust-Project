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
    addSession,
    clearMessages
  } = useChatStore();

  useEffect(() => {
    // Load sessions list
    api.getSessions().then((sessions) => {
      setSessions(sessions);
    }).catch((err) => {
      if ((err as any)?.status === 429 && (err as any)?.rateLimit) {
        const rl = (err as any).rateLimit;
        alert(`Rate limit exceeded. Remaining: ${rl.remaining ?? 'unknown'} / ${rl.limit ?? 'unknown'}`);
      } else {
        console.error('Failed to load sessions:', err);
      }
    });

    // Ensure session ID is saved
    if (!localStorage.getItem('rust_llm_session_id')) {
      const newId = crypto.randomUUID();
      setSessionId(newId);
    }

    // Load history
    loadHistory();
  }, []);

  const loadHistory = async () => {
    try {
      const history = await api.getHistory(sessionId);
      const messages = history.filter((msg) => msg.role !== 'system');
      setMessages(messages as any);
    } catch (error) {
      if ((error as any)?.status === 429 && (error as any)?.rateLimit) {
        const rl = (error as any).rateLimit;
        alert(`Rate limit exceeded. Remaining: ${rl.remaining ?? 'unknown'} / ${rl.limit ?? 'unknown'}`);
      } else {
        console.error('Failed to load history:', error);
      }
      setMessages([]);
    }
  };

  const handleNewChat = () => {
    const newId = crypto.randomUUID();
    console.log('Creating new chat with ID:', newId);
    setSessionId(newId);
    addSession(newId);
    clearMessages();
  };

  const handleSwitchSession = async (id: string) => {
    setSessionId(id);
    try {
      const history = await api.getHistory(id);
      const messages = history.filter((msg) => msg.role !== 'system');
      setMessages(messages as any);
    } catch (error) {
      if ((error as any)?.status === 429 && (error as any)?.rateLimit) {
        const rl = (error as any).rateLimit;
        alert(`Rate limit exceeded. Remaining: ${rl.remaining ?? 'unknown'} / ${rl.limit ?? 'unknown'}`);
      } else {
        console.error('Failed to load history:', error);
      }
      setMessages([]);
    }
  };

  const handleDeleteSession = async (id: string) => {
    try {
      console.log('Deleting session:', id);
      const isCurrentSession = id === sessionId;
      
      await api.deleteSession(id);
      console.log('Delete API call completed');
      
      // Small delay to ensure backend has processed the delete
      await new Promise(resolve => setTimeout(resolve, 100));
      
      // Reload sessions from backend to ensure consistency
      const sessions = await api.getSessions();
      console.log('Sessions after delete:', sessions);
      setSessions(sessions);
      
      // If we deleted the current session, switch to another one or create new
      if (isCurrentSession) {
        if (sessions.length > 0) {
          // Switch to the first available session
          console.log('Switching to existing session:', sessions[0]);
          setSessionId(sessions[0]);
          const history = await api.getHistory(sessions[0]);
          const messages = history.filter((msg) => msg.role !== 'system');
          setMessages(messages as any);
        } else {
          // No sessions left, create a new one
          console.log('No sessions left, creating new chat');
          handleNewChat();
        }
      }
    } catch (error) {
      if ((error as any)?.status === 429 && (error as any)?.rateLimit) {
        const rl = (error as any).rateLimit;
        alert(`Rate limit exceeded. Remaining: ${rl.remaining ?? 'unknown'} / ${rl.limit ?? 'unknown'}`);
      } else {
        console.error('Failed to delete session:', error);
      }
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


