import { useState } from 'react';
import { sendChatMessage } from '../services/api';
import type { ChatMessage } from '../types';

function Chatbot() {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const handleSend = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const userMessage = input.trim();

    if (!userMessage || isLoading) {
      return;
    }

    const updatedHistory: ChatMessage[] = [
      ...messages,
      { role: 'user', content: userMessage },
    ];

    setMessages(updatedHistory);
    setInput('');
    setIsLoading(true);

    try {
      const reply = await sendChatMessage(updatedHistory);

      setMessages((previousMessages) => [
        ...previousMessages,
        { role: 'assistant', content: reply },
      ]);
    } catch {
      setMessages((previousMessages) => [
        ...previousMessages,
        {
          role: 'assistant',
          content: 'I could not connect to the AI assistant. Please try again.',
        },
      ]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <section className="chatbot">
      <div className="chatbot-header">
        <p className="eyebrow">AI Assistant</p>
        <h2>H7 AI Assistant</h2>
      </div>

      <div className="chatbot-messages">
        {messages.length === 0 && (
          <p className="chatbot-placeholder">
            Ask me about Hashan&apos;s skills, projects, certificates, or experience.
          </p>
        )}

        {messages.map((message, index) => (
          <div className={`chat-message ${message.role}`} key={`${message.role}-${index}`}>
            <span>{message.role === 'user' ? 'You' : 'Assistant'}</span>
            <p>{message.content}</p>
          </div>
        ))}

        {isLoading && (
          <div className="chat-message assistant">
            <span>Assistant</span>
            <p>Typing...</p>
          </div>
        )}
      </div>

      <form className="chatbot-form" onSubmit={handleSend}>
        <input
          type="text"
          value={input}
          onChange={(event) => setInput(event.target.value)}
          placeholder="Ask a portfolio question..."
          disabled={isLoading}
        />
        <button type="submit" disabled={isLoading || !input.trim()}>
          Send
        </button>
      </form>
    </section>
  );
}

export default Chatbot;