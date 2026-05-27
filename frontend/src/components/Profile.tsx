import { useEffect, useMemo, useRef, useState } from 'react';
import type { Certificate, Education, PublicProfile, PublicProject, SocialLinks } from '../types';
import { emailLink, formatExternalLink, phoneLink } from '../utils/links';

interface ProfileProps {
  profile: PublicProfile;
}

type SocialKey =
  | 'github'
  | 'linkedin'
  | 'huggingface'
  | 'kaggle'
  | 'email'
  | 'phone'
  | 'resume'
  | 'instagram'
  | 'website';

interface SocialItem {
  key: SocialKey;
  label: string;
  href: string;
}

function padCount(value: number): string {
  return String(value).padStart(2, '0');
}

function mediaSource(path?: string): string | undefined {
  const trimmed = path?.trim();

  if (!trimmed) {
    return undefined;
  }

  if (
    trimmed.startsWith('/') ||
    trimmed.startsWith('http://') ||
    trimmed.startsWith('https://') ||
    trimmed.startsWith('data:')
  ) {
    return trimmed;
  }

  return `/${trimmed}`;
}

function projectImages(project?: PublicProject): string[] {
  if (!project) {
    return [];
  }

  const paths = project.image_paths?.length ? project.image_paths : project.image_path ? [project.image_path] : [];

  return paths
    .map(mediaSource)
    .filter((path): path is string => Boolean(path))
    .slice(0, 3);
}

function socialItems(socialLinks?: SocialLinks): SocialItem[] {
  if (!socialLinks) {
    return [];
  }

  const items: Array<SocialItem | undefined> = [
    socialLinks.github
      ? { key: 'github', label: 'GitHub', href: formatExternalLink(socialLinks.github) ?? socialLinks.github }
      : undefined,
    socialLinks.linkedin
      ? { key: 'linkedin', label: 'LinkedIn', href: formatExternalLink(socialLinks.linkedin) ?? socialLinks.linkedin }
      : undefined,
    socialLinks.huggingface
      ? {
          key: 'huggingface',
          label: 'Hugging Face',
          href: formatExternalLink(socialLinks.huggingface) ?? socialLinks.huggingface,
        }
      : undefined,
    socialLinks.kaggle
      ? { key: 'kaggle', label: 'Kaggle', href: formatExternalLink(socialLinks.kaggle) ?? socialLinks.kaggle }
      : undefined,
    socialLinks.resume
      ? { key: 'resume', label: 'CV', href: formatExternalLink(socialLinks.resume) ?? socialLinks.resume }
      : undefined,
    socialLinks.email ? { key: 'email', label: 'Email', href: emailLink(socialLinks.email) ?? socialLinks.email } : undefined,
    socialLinks.phone ? { key: 'phone', label: 'Phone', href: phoneLink(socialLinks.phone) ?? socialLinks.phone } : undefined,
    socialLinks.instagram
      ? { key: 'instagram', label: 'Instagram', href: formatExternalLink(socialLinks.instagram) ?? socialLinks.instagram }
      : undefined,
    socialLinks.website
      ? { key: 'website', label: 'Website', href: formatExternalLink(socialLinks.website) ?? socialLinks.website }
      : undefined,
  ];

  return items.filter((item): item is SocialItem => Boolean(item));
}

function SocialIcon({ type }: { type: SocialKey }) {
  if (type === 'resume') {
    return <span className="cv-social-text">CV</span>;
  }

  if (type === 'github') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M12 .5a12 12 0 0 0-3.79 23.39c.6.11.82-.26.82-.58v-2.04c-3.34.73-4.04-1.42-4.04-1.42-.55-1.39-1.34-1.76-1.34-1.76-1.09-.75.08-.73.08-.73 1.2.08 1.84 1.24 1.84 1.24 1.07 1.83 2.8 1.3 3.48.99.11-.78.42-1.3.76-1.6-2.67-.3-5.47-1.33-5.47-5.93 0-1.31.47-2.38 1.24-3.22-.13-.3-.54-1.53.12-3.18 0 0 1.01-.32 3.3 1.23a11.5 11.5 0 0 1 6 0c2.29-1.55 3.3-1.23 3.3-1.23.66 1.65.25 2.88.12 3.18.77.84 1.24 1.91 1.24 3.22 0 4.61-2.81 5.63-5.48 5.93.43.37.81 1.1.81 2.22v3.29c0 .32.22.7.83.58A12 12 0 0 0 12 .5Z" />
      </svg>
    );
  }

  if (type === 'linkedin') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M4.98 3.5a2.5 2.5 0 1 1 0 5 2.5 2.5 0 0 1 0-5ZM3 9h4v12H3V9Zm7 0h3.8v1.64h.05c.53-1 1.83-2.05 3.76-2.05 4.02 0 4.76 2.65 4.76 6.09V21h-4v-5.6c0-1.33-.02-3.05-1.86-3.05-1.86 0-2.14 1.45-2.14 2.95V21h-4V9Z" />
      </svg>
    );
  }

  if (type === 'huggingface') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M7.2 7.4a1.6 1.6 0 1 0 0-3.2 1.6 1.6 0 0 0 0 3.2Zm9.6 0a1.6 1.6 0 1 0 0-3.2 1.6 1.6 0 0 0 0 3.2ZM12 22c5.2 0 9-3.6 9-8.5 0-2.4-.9-4.6-2.5-6.1-.2 1.8-1.6 3.1-3.4 3.1-1.2 0-2.3-.6-3-1.5-.7.9-1.8 1.5-3 1.5-1.8 0-3.2-1.3-3.4-3.1A8.4 8.4 0 0 0 3 13.5C3 18.4 6.8 22 12 22Zm-3.8-7.5c.8 1.4 2.1 2.1 3.8 2.1s3-.7 3.8-2.1c.3-.5 1-.6 1.5-.3s.6 1 .3 1.5c-1.2 2-3.1 3-5.6 3s-4.4-1-5.6-3c-.3-.5-.1-1.2.3-1.5.5-.3 1.2-.2 1.5.3Z" />
      </svg>
    );
  }

  if (type === 'kaggle') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M5 3h4v8.1L16.2 3H21l-7.8 8.5L21.5 21h-5.1L9 12.4V21H5V3Z" />
      </svg>
    );
  }

  if (type === 'email') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M3 5h18a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1Zm9 7.4L4.7 7H4v.8l8 5.9 8-5.9V7h-.7L12 12.4Z" />
      </svg>
    );
  }

  if (type === 'phone') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M6.6 10.8c1.4 2.8 3.7 5.1 6.6 6.6l2.2-2.2c.3-.3.8-.4 1.2-.3 1.3.4 2.6.6 4 .6.7 0 1.2.5 1.2 1.2v3.6c0 .7-.5 1.2-1.2 1.2C10.4 22 2 13.6 2 3.2 2 2.5 2.5 2 3.2 2h3.6C7.5 2 8 2.5 8 3.2c0 1.4.2 2.8.6 4 .1.4 0 .9-.3 1.2l-1.7 2.4Z" />
      </svg>
    );
  }

  if (type === 'instagram') {
    return (
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M7 2h10a5 5 0 0 1 5 5v10a5 5 0 0 1-5 5H7a5 5 0 0 1-5-5V7a5 5 0 0 1 5-5Zm0 2a3 3 0 0 0-3 3v10a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3V7a3 3 0 0 0-3-3H7Zm5 4a4 4 0 1 1 0 8 4 4 0 0 1 0-8Zm0 2a2 2 0 1 0 0 4 2 2 0 0 0 0-4Zm5.2-3.2a1 1 0 1 1 0 2 1 1 0 0 1 0-2Z" />
      </svg>
    );
  }

  return (
    <svg viewBox="0 0 24 24" aria-hidden="true">
      <path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20Zm6.9 9h-3.1a15.9 15.9 0 0 0-1.1-5 8.1 8.1 0 0 1 4.2 5ZM12 4.1c.7 1 1.3 2.8 1.6 4.9h-3.2C10.7 6.9 11.3 5.1 12 4.1ZM4.3 13h3.9c.1 1.7.4 3.3.9 4.6A8 8 0 0 1 4.3 13Zm3.9-2H4.3a8 8 0 0 1 4.8-4.6A17 17 0 0 0 8.2 11Zm3.8 8.9c-.7-1-1.3-2.8-1.6-4.9h3.2c-.3 2.1-.9 3.9-1.6 4.9ZM14 13h-4v-2h4v2Zm.9 4.6c.5-1.3.8-2.9.9-4.6h3.9a8 8 0 0 1-4.8 4.6Z" />
    </svg>
  );
}

function SectionCounter({ count, label }: { count: number; label: string }) {
  return (
    <div className="section-counter">
      <div>
        {padCount(count)}
        <small>{label}</small>
      </div>
    </div>
  );
}

function Profile({ profile }: ProfileProps) {
  const skills = profile.skills ?? [];
  const projects = profile.projects ?? [];
  const certificates = profile.certificates ?? [];
  const education = profile.education ?? [];
  const socialLinks = socialItems(profile.social_links);
  const resumeItem = socialLinks.find((item) => item.key === 'resume');
  const profileSocialLinks = socialLinks.filter((item) => item.key !== 'resume');
  const displayName = profile.display_name ?? profile.name ?? 'Chamira Hashan';
  const role = profile.role ?? '';
  const tagline = profile.tagline ?? 'Building practical backend, AI/ML, and full-stack software projects.';
  const profileImage = mediaSource(profile.profile_image_path);

  const [activeSection, setActiveSection] = useState('home');
  const activeSectionRef = useRef('home');
  const [activeProjectIndex, setActiveProjectIndex] = useState(0);
  const [typedDescription, setTypedDescription] = useState('');
  const [projectImageIndex, setProjectImageIndex] = useState(0);
  const [activeCertIndex, setActiveCertIndex] = useState(0);
  const [leavingCertIndex, setLeavingCertIndex] = useState<number | null>(null);

  const typeTimerRef = useRef<number | null>(null);
  const moveTimerRef = useRef<number | null>(null);
  const certTimerRef = useRef<number | null>(null);

  const activeProject = projects[activeProjectIndex];
  const activeProjectImages = useMemo(() => projectImages(activeProject), [activeProject]);
  const activeProjectImage = activeProjectImages[projectImageIndex % Math.max(activeProjectImages.length, 1)];

  const goToProject = (index: number) => {
    if (projects.length === 0) {
      return;
    }

    setActiveProjectIndex((index + projects.length) % projects.length);
  };

  const goToCertificate = (index: number) => {
    if (certificates.length === 0) {
      return;
    }

    const nextIndex = (index + certificates.length) % certificates.length;

    if (nextIndex === activeCertIndex) {
      return;
    }

    setLeavingCertIndex(activeCertIndex);
    setActiveCertIndex(nextIndex);
    window.setTimeout(() => setLeavingCertIndex(null), 800);
  };

  const restartCertificateTimer = () => {
    if (certTimerRef.current) {
      window.clearInterval(certTimerRef.current);
    }

    if (certificates.length > 1) {
      certTimerRef.current = window.setInterval(() => {
        setActiveCertIndex((previousIndex) => {
          const nextIndex = (previousIndex + 1) % certificates.length;
          setLeavingCertIndex(previousIndex);
          window.setTimeout(() => setLeavingCertIndex(null), 800);
          return nextIndex;
        });
      }, 5000);
    }
  };

  const handleManualCertificateChange = (index: number) => {
    goToCertificate(index);
    restartCertificateTimer();
  };

  useEffect(() => {
    const sectionIds = ['home', 'projects', 'skills', 'certificates', 'education'];

    const getCurrentSection = () => {
      const markerY = window.innerHeight * 0.3;
      let bestId = 'home';
      let bestDistance = Number.POSITIVE_INFINITY;

      for (const id of sectionIds) {
        const section = document.getElementById(id);

        if (!section) {
          continue;
        }

        const rect = section.getBoundingClientRect();

        if (rect.top <= markerY && rect.bottom >= markerY) {
          bestId = id;
          break;
        }

        const distance = Math.abs(rect.top - markerY);

        if (distance < bestDistance) {
          bestDistance = distance;
          bestId = id;
        }
      }

      if (bestId !== activeSectionRef.current) {
        activeSectionRef.current = bestId;
        setActiveSection(bestId);
      }
    };

    let frameId = 0;

    const onScrollOrResize = () => {
      if (frameId) {
        return;
      }

      frameId = window.requestAnimationFrame(() => {
        frameId = 0;
        getCurrentSection();
      });
    };

    getCurrentSection();
    window.addEventListener('scroll', onScrollOrResize, { passive: true });
    window.addEventListener('resize', onScrollOrResize);

    return () => {
      if (frameId) {
        window.cancelAnimationFrame(frameId);
      }

      window.removeEventListener('scroll', onScrollOrResize);
      window.removeEventListener('resize', onScrollOrResize);
    };
  }, []);

  useEffect(() => {
    if (typeTimerRef.current) {
      window.clearTimeout(typeTimerRef.current);
    }

    if (moveTimerRef.current) {
      window.clearTimeout(moveTimerRef.current);
    }

    const description = activeProject?.short_description ?? '';

    if (!description) {
      setTypedDescription('');
      return undefined;
    }

    let currentIndex = 0;
    setTypedDescription('');

    const typeNext = () => {
      currentIndex += 1;
      setTypedDescription(description.slice(0, currentIndex));

      if (currentIndex < description.length) {
        typeTimerRef.current = window.setTimeout(typeNext, 38);
        return;
      }

      moveTimerRef.current = window.setTimeout(() => {
        setActiveProjectIndex((previousIndex) =>
          projects.length > 0 ? (previousIndex + 1) % projects.length : 0,
        );
      }, 7000);
    };

    typeTimerRef.current = window.setTimeout(typeNext, 220);

    return () => {
      if (typeTimerRef.current) {
        window.clearTimeout(typeTimerRef.current);
      }

      if (moveTimerRef.current) {
        window.clearTimeout(moveTimerRef.current);
      }
    };
  }, [activeProject?.short_description, projects.length]);

  useEffect(() => {
    setProjectImageIndex(0);

    if (activeProjectImages.length <= 1) {
      return undefined;
    }

    const imageTimer = window.setInterval(() => {
      setProjectImageIndex((previousIndex) => (previousIndex + 1) % activeProjectImages.length);
    }, 1200);

    return () => window.clearInterval(imageTimer);
  }, [activeProjectImages.length, activeProjectIndex]);

  useEffect(() => {
    if (certTimerRef.current) {
      window.clearInterval(certTimerRef.current);
    }

    if (certificates.length <= 1) {
      return undefined;
    }

    certTimerRef.current = window.setInterval(() => {
      setActiveCertIndex((previousIndex) => {
        const nextIndex = (previousIndex + 1) % certificates.length;
        setLeavingCertIndex(previousIndex);
        window.setTimeout(() => setLeavingCertIndex(null), 800);
        return nextIndex;
      });
    }, 5000);

    return () => {
      if (certTimerRef.current) {
        window.clearInterval(certTimerRef.current);
      }
    };
  }, [certificates.length]);

  const navItems = [
    { id: 'home', label: 'Home' },
    { id: 'projects', label: 'Projects' },
    { id: 'skills', label: 'Skills' },
    { id: 'certificates', label: 'Certificates' },
    { id: 'education', label: 'Education' },
  ];

  return (
    <main className="portfolio-page">
      <div className="top-nav-wrap">
        <nav className="top-nav" aria-label="Portfolio sections">
          {navItems.map((item) => (
            <a
              className={`nav-link ${activeSection === item.id ? 'active' : ''}`}
              href={`#${item.id}`}
              key={item.id}
              onClick={() => {
                activeSectionRef.current = item.id;
                setActiveSection(item.id);
              }}
            >
              {item.label}
            </a>
          ))}
        </nav>
      </div>

      <section className="hero" id="home">
        <div className="hero-mobile-frame">
          <div className="hero-content">
            <div className="hero-kicker">{displayName}</div>

            <h1>{tagline}</h1>

            {role && <p className="hero-role">{role}</p>}
            {profile.location && <p className="profile-location">{profile.location}</p>}
          </div>

          <div className="hero-visual">
            <div className={`portrait-card ${profileImage ? 'has-photo' : ''}`}>
              {profileImage && <img className="profile-photo" src={profileImage} alt={`${displayName} profile`} />}
              <div className="portrait-fallback">H7</div>
              <div className="status-dot">✦</div>
            </div>
          </div>
        </div>

        <div className="hero-mobile-actions">
          {resumeItem && (
            <a
              className="download-cv-button"
              href={resumeItem.href}
              target="_blank"
              rel="noreferrer"
              aria-label="Download CV"
            >
              Download CV
            </a>
          )}

          {profileSocialLinks.length > 0 && (
            <div className="social-only-row">
              {profileSocialLinks.map((item) => (
                <a
                  className="social-only"
                  href={item.href}
                  target={item.href.startsWith('mailto:') || item.href.startsWith('tel:') ? undefined : '_blank'}
                  rel={item.href.startsWith('mailto:') || item.href.startsWith('tel:') ? undefined : 'noreferrer'}
                  aria-label={item.label}
                  title={item.label}
                  key={item.key}
                >
                  <SocialIcon type={item.key} />
                </a>
              ))}
            </div>
          )}
        </div>
      </section>

      <section className="about-snapshot glass" aria-label="About portfolio snapshot">
        <div className="about-copy">
          <p className="eyebrow section-only-title">About</p>
          <h2>{displayName}</h2>
          <p>
            {profile.bio ??
              'A practical software engineering portfolio focused on backend, AI/ML, full-stack, and mobile project work.'}
          </p>
        </div>

        <div className="about-stats" aria-label="Portfolio quick counts">
          <div>
            <strong>{padCount(projects.length)}</strong>
            <span>Projects</span>
          </div>
          <div>
            <strong>{padCount(skills.length)}</strong>
            <span>Tech Count</span>
          </div>
        </div>
      </section>

      {projects.length > 0 && activeProject && (
        <section className="section glass" id="projects">
          <div className="section-header compact">
            <div className="section-title-wrap">
              <SectionCounter count={projects.length} label="WORK" />
              <p className="eyebrow section-only-title">Projects</p>
            </div>
          </div>

          <div className="project-carousel">
            <div className="project-track" style={{ transform: `translateX(-${activeProjectIndex * 100}%)` }}>
              {projects.map((project, index) => {
                const images = projectImages(project);
                const visibleImage =
                  index === activeProjectIndex ? activeProjectImage : images.length > 0 ? images[0] : undefined;
                const videoSource = mediaSource(project.video_path);

                return (
                  <article className="project-slide" key={`${project.title ?? 'project'}-${index}`}>
                    <div className="project-card">
                      <div className="project-info">
                        <span className="project-index">{padCount(index + 1)}</span>
                        {project.title && <h3>{project.title}</h3>}

                        <p className="project-description">
                          {index === activeProjectIndex ? typedDescription : project.short_description}
                          {index === activeProjectIndex && typedDescription.length < (project.short_description ?? '').length && (
                            <span className="cursor">|</span>
                          )}
                        </p>

                        {project.tech_stack.length > 0 && (
                          <div className="project-tags">
                            {project.tech_stack.slice(0, 6).map((tech) => (
                              <span key={`${project.title}-${tech}`}>{tech}</span>
                            ))}
                          </div>
                        )}

                        <div className="project-links">
                          {project.github_link && (
                            <a className="project-link" href={project.github_link} target="_blank" rel="noreferrer">
                              GitHub ↗
                            </a>
                          )}

                          {project.hf_link && (
                            <a className="project-link" href={project.hf_link} target="_blank" rel="noreferrer">
                              HF Space ↗
                            </a>
                          )}

                          {project.live_demo_link && (
                            <a className="project-link" href={project.live_demo_link} target="_blank" rel="noreferrer">
                              Live Demo ↗
                            </a>
                          )}
                        </div>
                      </div>

                      <div className="project-visual">
                        {videoSource ? (
                          <video src={videoSource} autoPlay muted loop playsInline />
                        ) : visibleImage ? (
                          <img src={visibleImage} alt={project.title ?? 'Project preview'} />
                        ) : (
                          <div className="project-shot active" />
                        )}
                      </div>
                    </div>
                  </article>
                );
              })}
            </div>
          </div>

          <div className="project-controls">
            <div className="dots">
              {projects.map((project, index) => (
                <button
                  className={`dot ${activeProjectIndex === index ? 'active' : ''}`}
                  type="button"
                  aria-label={`Open project ${index + 1}`}
                  onClick={() => goToProject(index)}
                  key={`${project.title ?? 'project-dot'}-${index}`}
                />
              ))}
            </div>

            <div className="circle-buttons">
              <button className="circle-btn" type="button" onClick={() => goToProject(activeProjectIndex - 1)}>
                ‹
              </button>
              <button className="circle-btn" type="button" onClick={() => goToProject(activeProjectIndex + 1)}>
                ›
              </button>
            </div>
          </div>
        </section>
      )}

      {skills.length > 0 && (
        <section className="section glass" id="skills">
          <div className="section-header compact">
            <p className="eyebrow section-only-title">Skills</p>
          </div>

          <div className="skills-marquee">
            <div className="skills-track">
              {[...skills, ...skills].map((skill, index) => (
                <span
                  className={`skill-pill ${index % skills.length < 3 ? 'featured' : ''}`}
                  key={`${skill}-${index}`}
                >
                  {skill}
                </span>
              ))}
            </div>
          </div>
        </section>
      )}

      {certificates.length > 0 && (
        <section className="section glass" id="certificates">
          <div className="section-header compact">
            <div className="section-title-wrap">
              <SectionCounter count={certificates.length} label="CERT" />
              <p className="eyebrow section-only-title">Certificates</p>
            </div>
          </div>

          <div className="certificate-book">
            <div className="cert-stage">
              {certificates.map((certificate: Certificate, index: number) => (
                <article
                  className={`cert-page ${activeCertIndex === index ? 'active' : ''} ${
                    leavingCertIndex === index ? 'leaving' : ''
                  }`}
                  key={`${certificate.name ?? 'certificate'}-${index}`}
                >
                  <div className="cert-no">{padCount(index + 1)}</div>

                  <div>
                    {certificate.name && <h3>{certificate.name}</h3>}

                    <div className="cert-meta">
                      {certificate.issuer && <span>{certificate.issuer}</span>}
                      {(certificate.date ?? certificate.year) && <span>{certificate.date ?? certificate.year}</span>}
                    </div>

                    {certificate.link && (
                      <a className="cert-view" href={certificate.link} target="_blank" rel="noreferrer">
                        View Certificate ↗
                      </a>
                    )}
                  </div>
                </article>
              ))}
            </div>

            <div className="cert-controls">
              <div className="cert-dots">
                {certificates.map((certificate, index) => (
                  <button
                    className={`cert-dot ${activeCertIndex === index ? 'active' : ''}`}
                    type="button"
                    aria-label={`Open certificate ${index + 1}`}
                    onClick={() => handleManualCertificateChange(index)}
                    key={`${certificate.name ?? 'cert-dot'}-${index}`}
                  />
                ))}
              </div>

              <div className="circle-buttons">
                <button className="circle-btn" type="button" onClick={() => handleManualCertificateChange(activeCertIndex - 1)}>
                  ‹
                </button>
                <button className="circle-btn" type="button" onClick={() => handleManualCertificateChange(activeCertIndex + 1)}>
                  ›
                </button>
              </div>
            </div>
          </div>
        </section>
      )}

      {education.length > 0 && (
        <section className="section glass" id="education">
          <div className="section-header compact">
            <div className="section-title-wrap">
              <SectionCounter count={education.length} label="EDU" />
              <p className="eyebrow section-only-title">Education</p>
            </div>
          </div>

          <div className="education-stack">
            {education.map((edu: Education, index: number) => (
              <article className="edu-card" key={`${edu.institution ?? 'edu'}-${index}`}>
                <span>{edu.duration ?? edu.year ?? `Education ${index + 1}`}</span>
                <h3>{edu.degree ?? edu.institution ?? `Education ${index + 1}`}</h3>
                <p>{[edu.institution, edu.grade, edu.status].filter(Boolean).join(' • ')}</p>

                {edu.link && (
                  <a className="cert-view" href={edu.link} target="_blank" rel="noreferrer">
                    View ↗
                  </a>
                )}
              </article>
            ))}
          </div>
        </section>
      )}

      <footer className="portfolio-footer">
        © {new Date().getFullYear()} {displayName}. All rights reserved.
      </footer>
    </main>
  );
}

export default Profile;
