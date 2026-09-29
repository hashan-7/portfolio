import { useEffect, useRef, useState } from 'react';
import { sendChatMessage } from '../services/api';
import type { ChatMessage } from '../types';
import '../styles/assistant.css';

interface ChatbotProps {
  onClose: () => void;
}

interface Position {
  x: number;
  y: number;
}

interface DragState {
  active: boolean;
  pointerId: number;
  startX: number;
  startY: number;
  originX: number;
  originY: number;
  minX: number;
  maxX: number;
  minY: number;
  maxY: number;
}

const inactiveDrag: DragState = {
  active: false,
  pointerId: -1,
  startX: 0,
  startY: 0,
  originX: 0,
  originY: 0,
  minX: 0,
  maxX: 0,
  minY: 0,
  maxY: 0,
};

function linkLabel(url: string): string {
  const host = new URL(url).hostname.replace(/^www\./, '');
  const isDomain = (domain: string) => host === domain || host.endsWith(`.${domain}`);

  if (isDomain('github.com')) return 'GitHub';
  if (isDomain('huggingface.co')) return 'Hugging Face';
  if (isDomain('linkedin.com')) return 'LinkedIn';
  if (isDomain('kaggle.com')) return 'Kaggle';
  return host;
}

function MessageContent({ content }: { content: string }) {
  const parts = content.split(/(https?:\/\/[^\s]+)/g);

  return parts.map((part, index) => {
    if (!/^https?:\/\//.test(part)) {
      return <span key={`text-${index}`}>{part}</span>;
    }

    const cleanUrl = part.replace(/[),.;!?]+$/, '');
    const trailingText = part.slice(cleanUrl.length);

    try {
      return (
        <span key={`${cleanUrl}-${index}`}>
          <a className="message-link" href={cleanUrl} target="_blank" rel="noopener noreferrer">
            {linkLabel(cleanUrl)} <span aria-hidden="true">↗</span>
          </a>
          {trailingText}
        </span>
      );
    } catch {
      return <span key={`invalid-${index}`}>{part}</span>;
    }
  });
}

function clamp(value: number, minimum: number, maximum: number): number {
  return Math.min(Math.max(value, minimum), maximum);
}

const MIN_TYPING_INDICATOR_MS = 1000;
const RESPONSE_CHUNK_MS = 45;

function responseChunks(content: string): string[] {
  return content.match(/(?:\S+\s*){1,3}/g) ?? [];
}

function waitForDelay(duration: number, signal: AbortSignal): Promise<boolean> {
  return new Promise((resolve) => {
    if (signal.aborted) {
      resolve(false);
      return;
    }

    const finish = () => {
      window.clearTimeout(timer);
      signal.removeEventListener('abort', finish);
      resolve(!signal.aborted);
    };
    const timer = window.setTimeout(finish, duration);
    signal.addEventListener('abort', finish, { once: true });
  });
}

function Chatbot({ onClose }: ChatbotProps) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [isWaiting, setIsWaiting] = useState(false);
  const [streamingReply, setStreamingReply] = useState<string | null>(null);
  const [position, setPosition] = useState<Position>({ x: 0, y: 0 });
  const panelRef = useRef<HTMLElement | null>(null);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const messagesRef = useRef<HTMLDivElement | null>(null);
  const dragRef = useRef<DragState>(inactiveDrag);
  const requestControllerRef = useRef<AbortController | null>(null);

  useEffect(() => {
    inputRef.current?.focus();

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') onClose();
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      requestControllerRef.current?.abort();
    };
  }, [onClose]);

  useEffect(() => {
    const messagesElement = messagesRef.current;
    if (!messagesElement) return;
    messagesElement.scrollTop = messagesElement.scrollHeight;
  }, [isWaiting, messages, streamingReply]);

  const startDrag = (event: React.PointerEvent<HTMLDivElement>) => {
    const panel = panelRef.current;
    const isCompact = window.matchMedia('(max-width: 720px), (pointer: coarse)').matches;

    if (!panel || isCompact || (event.pointerType === 'mouse' && event.button !== 0)) return;

    const rect = panel.getBoundingClientRect();
    const baseLeft = rect.left - position.x;
    const baseTop = rect.top - position.y;

    dragRef.current = {
      active: true,
      pointerId: event.pointerId,
      startX: event.clientX,
      startY: event.clientY,
      originX: position.x,
      originY: position.y,
      minX: 12 - baseLeft,
      maxX: window.innerWidth - rect.width - 12 - baseLeft,
      minY: 12 - baseTop,
      maxY: window.innerHeight - rect.height - 12 - baseTop,
    };

    event.currentTarget.setPointerCapture(event.pointerId);
    event.preventDefault();
  };

  const moveDrag = (event: React.PointerEvent<HTMLDivElement>) => {
    const drag = dragRef.current;
    if (!drag.active || drag.pointerId !== event.pointerId) return;

    setPosition({
      x: clamp(drag.originX + event.clientX - drag.startX, drag.minX, drag.maxX),
      y: clamp(drag.originY + event.clientY - drag.startY, drag.minY, drag.maxY),
    });
  };

  const stopDrag = (event: React.PointerEvent<HTMLDivElement>) => {
    if (dragRef.current.pointerId !== event.pointerId) return;
    dragRef.current = inactiveDrag;

    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
  };

  const handleSend = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const content = input.trim();

    if (!content || isWaiting || requestControllerRef.current) return;

    const nextHistory: ChatMessage[] = [...messages, { role: 'user', content }];
    setMessages(nextHistory);
    setInput('');
    setIsWaiting(true);
    const controller = new AbortController();
    requestControllerRef.current = controller;

    try {
      const [reply, typingMinimumCompleted] = await Promise.all([
        sendChatMessage(nextHistory, controller.signal),
        waitForDelay(MIN_TYPING_INDICATOR_MS, controller.signal),
      ]);

      if (!typingMinimumCompleted || controller.signal.aborted) return;

      const response = reply.trim() || 'No reply received from H7 Assistant.';
      const chunks = responseChunks(response);
      const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

      if (reducedMotion || chunks.length <= 1) {
        setMessages((history) => [
          ...history,
          { role: 'assistant', content: response },
        ]);
        return;
      }

      let visibleReply = chunks[0];
      setStreamingReply(visibleReply);

      for (let index = 1; index < chunks.length; index += 1) {
        if (!(await waitForDelay(RESPONSE_CHUNK_MS, controller.signal))) return;
        visibleReply += chunks[index];
        setStreamingReply(visibleReply);
      }

      setMessages((history) => [
        ...history,
        { role: 'assistant', content: response },
      ]);
    } catch (error) {
      if (controller.signal.aborted || (error instanceof DOMException && error.name === 'AbortError')) return;

      setMessages((history) => [
        ...history,
        {
          role: 'assistant',
          content: 'H7 Assistant is temporarily unavailable. Please try again.',
        },
      ]);
    } finally {
      if (requestControllerRef.current === controller) {
        requestControllerRef.current = null;
        const wasAborted = controller.signal.aborted;
        controller.abort();
        if (!wasAborted) {
          setStreamingReply(null);
          setIsWaiting(false);
          window.requestAnimationFrame(() => inputRef.current?.focus());
        }
      }
    }
  };

  return (
    <section
      ref={panelRef}
      className="assistant-panel"
      role="dialog"
      aria-modal="false"
      aria-labelledby="assistant-title"
      aria-describedby="assistant-description"
      style={{ transform: `translate3d(${position.x}px, ${position.y}px, 0)` }}
    >
      <header className="assistant-header">
        <div
          className="assistant-drag-handle"
          onPointerDown={startDrag}
          onPointerMove={moveDrag}
          onPointerUp={stopDrag}
          onPointerCancel={stopDrag}
        >
          <span className="assistant-status" aria-hidden="true" />
          <div>
            <h2 id="assistant-title">H7 Assistant</h2>
            <p id="assistant-description">Ask about this portfolio's projects, skills, certificates, or education.</p>
          </div>
        </div>
        <button className="assistant-close" type="button" onClick={onClose} aria-label="Close H7 Assistant">×</button>
      </header>

      <div
        className="assistant-messages"
        ref={messagesRef}
        role="log"
        aria-live="polite"
        aria-relevant="additions"
        aria-busy={isWaiting}
      >
        {messages.length === 0 && (
          <div className="assistant-welcome">
            <span>PORTFOLIO CONTEXT / READY</span>
            <h3>What would you like to know?</h3>
            <p>Try asking about a project, the technical stack, certifications, education, or public contact links.</p>
          </div>
        )}

        {messages.map((message, index) => (
          <article className={`assistant-message ${message.role}`} key={`${message.role}-${index}`}>
            <span>{message.role === 'user' ? 'You' : 'H7'}</span>
            <p><MessageContent content={message.content} /></p>
          </article>
        ))}

        {streamingReply !== null && (
          <article className="assistant-message assistant" aria-hidden="true">
            <span>H7</span>
            <p>{responseChunks(streamingReply).map((chunk, index) => (
              <span className="assistant-response-chunk" key={index}>
                <MessageContent content={chunk} />
              </span>
            ))}</p>
          </article>
        )}

        {isWaiting && streamingReply === null && (
          <div className="assistant-thinking" role="status">
            <span>H7 Typing...</span>
            <i aria-hidden="true" /><i aria-hidden="true" /><i aria-hidden="true" />
          </div>
        )}
      </div>

      <form className="assistant-form" onSubmit={handleSend}>
        <label className="sr-only" htmlFor="assistant-input">Ask H7 Assistant</label>
        <input
          ref={inputRef}
          id="assistant-input"
          type="text"
          value={input}
          onChange={(event) => setInput(event.target.value)}
          placeholder="Ask about the portfolio..."
          autoComplete="off"
          maxLength={500}
          disabled={isWaiting}
        />
        <button type="submit" disabled={isWaiting || !input.trim()} aria-label="Send message">Send</button>
      </form>
    </section>
  );
}

export default Chatbot;
