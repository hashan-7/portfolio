import { useEffect, useState } from 'react';
import AssistantLauncher from '../components/AssistantLauncher';
import Profile from '../components/Profile';
import { getProfile } from '../services/api';
import type { PublicProfile } from '../types';
import '../styles/portfolio.css';

function PortfolioLoading() {
  return (
    <main className="portfolio-loading" aria-busy="true" aria-label="Loading portfolio">
      <div className="loading-nav"><span>H7</span><i /><i /><i /></div>
      <div className="loading-hero">
        <div><span /><strong /><strong /><p /></div>
        <div className="loading-portrait" />
      </div>
      <span className="sr-only">Loading portfolio content...</span>
    </main>
  );
}

function HomePage() {
  const [profile, setProfile] = useState<PublicProfile | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [hasError, setHasError] = useState(false);
  const [requestId, setRequestId] = useState(0);

  useEffect(() => {
    let active = true;
    const controller = new AbortController();

    getProfile(controller.signal)
      .then((data) => {
        if (active) setProfile(data);
      })
      .catch((error: unknown) => {
        if (active && !(error instanceof DOMException && error.name === 'AbortError')) {
          setHasError(true);
        }
      })
      .finally(() => {
        if (active) setIsLoading(false);
      });

    return () => {
      active = false;
      controller.abort();
    };
  }, [requestId]);

  if (isLoading) return <PortfolioLoading />;

  if (hasError || !profile) {
    return (
      <main className="portfolio-error">
        <span className="site-mark">H7</span>

<p className="eyebrow">Connection interrupted</p>
        <h1>The portfolio is temporarily unavailable.</h1>
        <p>The portfolio service may be waking up. Wait a moment, then try again.</p>
        <button
          type="button"
          onClick={() => {
            setHasError(false);
            setIsLoading(true);
            setRequestId((value) => value + 1);
          }}
        >
          Retry loading
        </button>
      </main>
    );
  }

  return (
    <>
      <Profile profile={profile} />
      <AssistantLauncher />
    </>
  );
}

export default HomePage;
