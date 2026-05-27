import { useEffect, useRef, useState } from 'react';
import { sendChatMessage } from '../services/api';
import type { ChatMessage } from '../types';

interface ChatPosition {
  x: number;
  y: number;
}

interface DragState {
  active: boolean;
  pointerId: number | null;
  offsetX: number;
  offsetY: number;
}

function friendlyLinkLabel(url: string): string {
  const lowerUrl = url.toLowerCase();

  if (lowerUrl.includes('github.com')) {
    return 'GitHub Link ↗';
  }

  if (lowerUrl.includes('huggingface.co')) {
    return 'Hugging Face Link ↗';
  }

  if (lowerUrl.includes('linkedin.com')) {
    return 'LinkedIn Link ↗';
  }

  if (lowerUrl.includes('drive.google.com')) {
    return 'Google Drive Link ↗';
  }

  if (lowerUrl.includes('kaggle.com')) {
    return 'Kaggle Link ↗';
  }

  if (lowerUrl.includes('instagram.com')) {
    return 'Instagram Link ↗';
  }

  return 'Open Link ↗';
}

function renderMessageContent(content: string) {
  const urlRegex = /(https?:\/\/[^\s]+)/g;
  const parts = content.split(urlRegex);

  return parts.map((part, index) => {
    if (part.match(urlRegex)) {
      const cleanUrl = part.replace(/[),.]+$/, '');
      const trailing = part.slice(cleanUrl.length);

      return (
        <span key={`${part}-${index}`}>
          <a className="message-link" href={cleanUrl} target="_blank" rel="noreferrer">
            {friendlyLinkLabel(cleanUrl)}
          </a>
          {trailing && <span className="message-text-part">{trailing}</span>}
        </span>
      );
    }

    return <span className="message-text-part" key={`${part}-${index}`}>{part}</span>;
  });
}

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

function Chatbot() {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [isWaiting, setIsWaiting] = useState(false);
  const [isTypingReply, setIsTypingReply] = useState(false);
  const [isOpen, setIsOpen] = useState(false);
  const [isClosing, setIsClosing] = useState(false);
  const [position, setPosition] = useState<ChatPosition | null>(null);

  const messagesRef = useRef<HTMLDivElement | null>(null);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const popRef = useRef<HTMLElement | null>(null);
  const typingTimerRef = useRef<number | null>(null);
  const dragRef = useRef<DragState>({
    active: false,
    pointerId: null,
    offsetX: 0,
    offsetY: 0,
  });

  const scrollToBottom = () => {
    window.requestAnimationFrame(() => {
      if (messagesRef.current) {
        messagesRef.current.scrollTop = messagesRef.current.scrollHeight;
      }
    });
  };

  const getDefaultPosition = (): ChatPosition => {
    const width = Math.min(360, window.innerWidth - 24);
    const height = Math.min(520, window.innerHeight - 104);

    return {
      x: Math.max(10, window.innerWidth - width - 18),
      y: Math.max(10, window.innerHeight - height - 78),
    };
  };

  const stopDragging = () => {
    dragRef.current = {
      active: false,
      pointerId: null,
      offsetX: 0,
      offsetY: 0,
    };
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isWaiting, isTypingReply, isOpen]);

  useEffect(() => {
    return () => {
      if (typingTimerRef.current) {
        window.clearTimeout(typingTimerRef.current);
      }
    };
  }, []);

  useEffect(() => {
    const handleResize = () => {
      setPosition((currentPosition) => {
        if (!currentPosition || !popRef.current) {
          return currentPosition;
        }

        const rect = popRef.current.getBoundingClientRect();

        return {
          x: clamp(currentPosition.x, 10, Math.max(10, window.innerWidth - rect.width - 10)),
          y: clamp(currentPosition.y, 10, Math.max(10, window.innerHeight - rect.height - 10)),
        };
      });
    };

    const handleWindowPointerEnd = () => {
      stopDragging();
    };

    window.addEventListener('resize', handleResize);
    window.addEventListener('pointerup', handleWindowPointerEnd);
    window.addEventListener('pointercancel', handleWindowPointerEnd);
    window.addEventListener('blur', handleWindowPointerEnd);

    return () => {
      window.removeEventListener('resize', handleResize);
      window.removeEventListener('pointerup', handleWindowPointerEnd);
      window.removeEventListener('pointercancel', handleWindowPointerEnd);
      window.removeEventListener('blur', handleWindowPointerEnd);
    };
  }, []);

  const openChat = () => {
    setIsClosing(false);
    setPosition(getDefaultPosition());
    setIsOpen(true);

    window.setTimeout(() => {
      scrollToBottom();
    }, 260);
  };

  const closeChat = () => {
    stopDragging();
    setIsClosing(true);

    window.setTimeout(() => {
      setIsOpen(false);
      setIsClosing(false);
      setPosition(null);
    }, 300);
  };

  const typeAssistantReply = (reply: string) => {
    const safeReply = reply.trim() || 'No reply received from H7 Assistant.';

    setIsTypingReply(true);
    setMessages((previousMessages) => [...previousMessages, { role: 'assistant', content: '' }]);

    let index = 0;

    const typingDelay = (character: string) => {
      if (character === '\n') {
        return 90;
      }

      if (/[.!?]/.test(character)) {
        return 72;
      }

      if (/[,;:]/.test(character)) {
        return 48;
      }

      return 24;
    };

    const typeNext = () => {
      index += 1;

      setMessages((previousMessages) => {
        const nextMessages = [...previousMessages];
        const lastIndex = nextMessages.length - 1;

        if (lastIndex >= 0 && nextMessages[lastIndex].role === 'assistant') {
          nextMessages[lastIndex] = {
            ...nextMessages[lastIndex],
            content: safeReply.slice(0, index),
          };
        }

        return nextMessages;
      });

      if (index < safeReply.length) {
        typingTimerRef.current = window.setTimeout(typeNext, typingDelay(safeReply[index - 1] ?? ''));
        return;
      }

      setIsTypingReply(false);
      setIsWaiting(false);
    };

    typingTimerRef.current = window.setTimeout(typeNext, 160);
  };

  const handleSend = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const userMessage = input.trim();

    if (!userMessage || isWaiting || isTypingReply) {
      return;
    }

    if (typingTimerRef.current) {
      window.clearTimeout(typingTimerRef.current);
    }

    const updatedHistory: ChatMessage[] = [...messages, { role: 'user', content: userMessage }];

    setMessages(updatedHistory);
    setInput('');
    setIsWaiting(true);

    try {
      const reply = await sendChatMessage(updatedHistory);
      typeAssistantReply(reply);
    } catch {
      typeAssistantReply('I could not connect to H7 Assistant. Please try again.');
    }
  };

  const startDrag = (event: React.PointerEvent<HTMLDivElement>) => {
    if (!popRef.current) {
      return;
    }

    if (event.pointerType === 'mouse' && event.button !== 0) {
      return;
    }

    const rect = popRef.current.getBoundingClientRect();

    dragRef.current = {
      active: true,
      pointerId: event.pointerId,
      offsetX: event.clientX - rect.left,
      offsetY: event.clientY - rect.top,
    };

    event.currentTarget.setPointerCapture(event.pointerId);
    event.preventDefault();
  };

  const moveDrag = (event: React.PointerEvent<HTMLDivElement>) => {
    const drag = dragRef.current;

    if (!drag.active || drag.pointerId !== event.pointerId || !popRef.current) {
      return;
    }

    if (event.pointerType === 'mouse' && event.buttons !== 1) {
      stopDragging();
      return;
    }

    const rect = popRef.current.getBoundingClientRect();

    setPosition({
      x: clamp(event.clientX - drag.offsetX, 10, Math.max(10, window.innerWidth - rect.width - 10)),
      y: clamp(event.clientY - drag.offsetY, 10, Math.max(10, window.innerHeight - rect.height - 10)),
    });
  };

  const stopDrag = (event: React.PointerEvent<HTMLDivElement>) => {
    if (dragRef.current.pointerId === event.pointerId) {
      stopDragging();

      try {
        event.currentTarget.releasePointerCapture(event.pointerId);
      } catch {
        
      }
    }
  };

  return (
    <>
      {!isOpen && !isClosing && (
        <button className="h7-launcher" type="button" onClick={openChat}>
          <span className="h7-launcher-orb">H7</span>
          <span>
            <strong>H7 Ask</strong>
            <span>Open assistant</span>
          </span>
        </button>
      )}

      {(isOpen || isClosing) && (
        <section
          ref={popRef}
          className={`chat-pop ${isOpen && !isClosing ? 'open' : ''} ${isClosing ? 'closing' : ''}`}
          style={
            position
              ? {
                  left: position.x,
                  top: position.y,
                  right: 'auto',
                  bottom: 'auto',
                }
              : undefined
          }
        >
          <header className="chat-header">
            <div className="chat-top">
              <div
                className="chat-drag-zone"
                onPointerDown={startDrag}
                onPointerMove={moveDrag}
                onPointerUp={stopDrag}
                onPointerCancel={stopDrag}
                onLostPointerCapture={stopDrag}
              >
                <div className="chat-title">
                  <h2>H7 Assistant</h2>
                  <p>Hold this title area to move. Ask about projects, skills, certificates, or education.</p>
                </div>
              </div>

              <button className="close-btn" type="button" onClick={closeChat}>
                ✕
              </button>
            </div>
          </header>

          <div className="chat-messages" ref={messagesRef}>
            {messages.length === 0 && !isWaiting && !isTypingReply && (
              <div className="empty-chat">
                <strong>Welcome to H7 Assistant</strong>
                <p>Ask anything about this portfolio. Try project 1, skills, certificates, or strengths.</p>
              </div>
            )}

            {messages.map((message, index) => (
              <div className={`msg ${message.role}`} key={`${message.role}-${index}`}>
                <span className="msg-label">{message.role === 'user' ? 'You' : 'H7 Assistant'}</span>
                <p>{renderMessageContent(message.content)}</p>
              </div>
            ))}

            {isWaiting && !isTypingReply && (
              <div className="msg assistant">
                <span className="msg-label">H7 Assistant</span>
                <div className="typing">
                  <i />
                  <i />
                  <i />
                </div>
              </div>
            )}
          </div>

          <form className="chat-form" onSubmit={handleSend}>
            <input
              ref={inputRef}
              type="text"
              value={input}
              onChange={(event) => setInput(event.target.value)}
              placeholder="Ask H7 Assistant..."
              disabled={isWaiting || isTypingReply}
            />
            <button className="send-btn" type="submit" disabled={isWaiting || isTypingReply || !input.trim()}>
              Send
            </button>
          </form>
        </section>
      )}
    </>
  );
}

export default Chatbot;
