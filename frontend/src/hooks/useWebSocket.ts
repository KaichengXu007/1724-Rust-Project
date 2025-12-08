import { useEffect, useRef, useCallback } from 'react';
import { useChatStore } from '../store/chatStore';
import { api } from '../services/api';

export const useWebSocket = () => {
  const wsRef = useRef<WebSocket | null>(null);
  const startTimeRef = useRef<number>(0);
  const tokenCountRef = useRef<number>(0);
  
  const {
    sessionId,
    settings,
    addMessage,
    updateLastMessage,
    setIsGenerating,
    setIsConnected,
    setTokenCount,
    setTokensPerSecond,
  } = useChatStore();

  const sendMessage = useCallback((prompt: string) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.close();
    }

    // Add user message
    addMessage({ role: 'user', content: prompt });

    // Add empty assistant message
    addMessage({ role: 'assistant', content: '' });

    const ws = api.createWebSocket();
    wsRef.current = ws;

    startTimeRef.current = Date.now();
    tokenCountRef.current = 0;
    setTokenCount(0);
    setTokensPerSecond(0);
    setIsGenerating(true);
    setIsConnected(false);

    ws.onopen = () => {
      setIsConnected(true);

      const stopSequences = settings.stopSequences
        ? settings.stopSequences.split(',').map(s => s.trim()).filter(s => s)
        : [];

      const payload: any = {
        'model-name': settings.model,
        prompt,
        'session-id': sessionId,
        'max-token': settings.maxTokens,
        temperature: settings.temperature,
        'top-p': settings.topP,
        'top-k': settings.topK,
        'repeat-penalty': settings.repeatPenalty,
        device: settings.device,
        stop: stopSequences,
      };

      if (settings.systemPrompt) {
        payload.messages = [
          { role: 'system', content: settings.systemPrompt },
          { role: 'user', content: prompt },
        ];
      }

      ws.send(JSON.stringify(payload));
    };

    ws.onmessage = (event) => {
      const token = event.data;

      if (token.startsWith('__ERROR__')) {
        const errorMsg = token.substring(9);
        updateLastMessage(`\n\n*[Error: ${errorMsg}]*`);
        setIsConnected(false);
        return;
      }

      tokenCountRef.current++;
      const elapsed = (Date.now() - startTimeRef.current) / 1000;
      const tps = elapsed > 0 ? tokenCountRef.current / elapsed : 0;

      setTokenCount(tokenCountRef.current);
      setTokensPerSecond(tps);

      // Append token to last message
      useChatStore.setState((state) => {
        const messages = [...state.messages];
        if (messages.length > 0) {
          const lastMessage = messages[messages.length - 1];
          messages[messages.length - 1] = {
            ...lastMessage,
            content: lastMessage.content + token,
          };
        }
        return { messages };
      });
    };

    ws.onclose = () => {
      setIsGenerating(false);
      wsRef.current = null;
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      setIsConnected(false);
      setIsGenerating(false);
    };
  }, [sessionId, settings, addMessage, updateLastMessage, setIsGenerating, setIsConnected, setTokenCount, setTokensPerSecond]);

  const stopGeneration = useCallback(() => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
      setIsGenerating(false);
    }
  }, [setIsGenerating]);

  useEffect(() => {
    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, []);

  return { sendMessage, stopGeneration };
};
