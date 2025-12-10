const API_BASE = '';

function parseRateLimitHeaders(headers: Headers) {
  const limit = headers.get('x-ratelimit-limit');
  const remaining = headers.get('x-ratelimit-remaining');
  const reset = headers.get('x-ratelimit-reset');
  return {
    limit: limit ? Number(limit) : undefined,
    remaining: remaining ? Number(remaining) : undefined,
    reset: reset ? Number(reset) : undefined,
  };
}

export const api = {
  // Sessions
  async getSessions(): Promise<string[]> {
    const res = await fetch(`${API_BASE}/sessions`);
    if (!res.ok) {
      if (res.status === 429) {
        const rl = parseRateLimitHeaders(res.headers);
        const err: any = new Error('Rate limit exceeded');
        err.status = 429;
        err.rateLimit = rl;
        throw err;
      }
      throw new Error('Failed to fetch sessions');
    }
    return res.json();
  },

  async deleteSession(sessionId: string): Promise<void> {
    const res = await fetch(`${API_BASE}/chat/history/${sessionId}`, {
      method: 'DELETE',
    });
    if (!res.ok) throw new Error('Failed to delete session');
  },

  // History
  async getHistory(sessionId: string): Promise<Array<{ role: string; content: string }>> {
    const res = await fetch(`${API_BASE}/chat/history/${sessionId}`);
    if (!res.ok) {
      if (res.status === 429) {
        const rl = parseRateLimitHeaders(res.headers);
        const err: any = new Error('Rate limit exceeded');
        err.status = 429;
        err.rateLimit = rl;
        throw err;
      }
      throw new Error('Failed to fetch history');
    }
    return res.json();
  },

  async rollbackHistory(sessionId: string, amount: number): Promise<void> {
    const res = await fetch(`${API_BASE}/chat/history/${sessionId}/rollback`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ amount }),
    });
    if (!res.ok) {
      if (res.status === 429) {
        const rl = parseRateLimitHeaders(res.headers);
        const err: any = new Error('Rate limit exceeded');
        err.status = 429;
        err.rateLimit = rl;
        throw err;
      }
      throw new Error('Failed to rollback history');
    }
  },

  // WebSocket
  createWebSocket(): WebSocket {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/chat/ws`;
    return new WebSocket(wsUrl);
  },
};
