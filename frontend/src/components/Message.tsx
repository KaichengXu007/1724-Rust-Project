import React, { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import rehypeHighlight from 'rehype-highlight';
import type { Message as MessageType } from '../store/chatStore';
import 'highlight.js/styles/github-dark.css';

interface MessageProps {
  message: MessageType;
}

const Message: React.FC<MessageProps> = ({ message }) => {
  const [copied, setCopied] = useState(false);

  const copyCode = (code: string) => {
    navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (message.role === 'user') {
    return (
      <div className="flex w-full mb-6 message-fade-in justify-end items-center gap-2 group">
        <div className="relative max-w-[85%] rounded-2xl px-5 py-3.5 bg-gray-800 text-white shadow-lg">
          {message.content}
        </div>
      </div>
    );
  }

  return (
    <div className="flex w-full mb-6 message-fade-in justify-start">
      <div className="relative max-w-[85%] rounded-2xl px-5 py-3.5 text-gray-100 prose prose-invert prose-pre:p-0 prose-pre:bg-transparent max-w-none">
        {message.content ? (
          <ReactMarkdown
            rehypePlugins={[rehypeHighlight]}
            components={{
              code({node, inline, className, children, ...props}: any) {
                const codeString = String(children).replace(/\n$/, '');
                
                return !inline ? (
                  <div className="relative group/code">
                    <pre className={className}>
                      <code {...props} className={className}>
                        {children}
                      </code>
                    </pre>
                    <button
                      onClick={() => copyCode(codeString)}
                      className="absolute top-2 right-2 opacity-0 group-hover/code:opacity-100 px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-xs text-gray-300 transition-all"
                    >
                      {copied ? '✓ Copied!' : 'Copy'}
                    </button>
                  </div>
                ) : (
                  <code {...props} className={className}>
                    {children}
                  </code>
                );
              }
            }}
          >
            {message.content}
          </ReactMarkdown>
        ) : (
          <div className="typing-indicator">
            <span /><span /><span />
          </div>
        )}
      </div>
    </div>
  );
};

export default Message;
