import { lazy, Suspense, useRef, useState } from 'react';

const Chatbot = lazy(() => import('./Chatbot'));
const preloadAssistant = () => void import('./Chatbot');

function AssistantLauncher() {
  const [isOpen, setIsOpen] = useState(false);
  const launcherRef = useRef<HTMLButtonElement | null>(null);

  return (
    <>
      {!isOpen && (
        <button
          ref={launcherRef}
          className="assistant-launcher"
          type="button"
          onClick={() => setIsOpen(true)}
          onFocus={preloadAssistant}
          onPointerEnter={preloadAssistant}
          aria-label="Open H7 Assistant"
        >
          <span className="assistant-launcher-mark" aria-hidden="true">H7</span>
          <span><strong>Ask H7</strong><small>Portfolio assistant</small></span>
        </button>
      )}

      {isOpen && (
        <Suspense fallback={<div className="assistant-loading" role="status">Opening H7 Assistant...</div>}>
          <Chatbot
            onClose={() => {
              setIsOpen(false);
              window.requestAnimationFrame(() => launcherRef.current?.focus());
            }}
          />
        </Suspense>
      )}
    </>
  );
}

export default AssistantLauncher;
