import { useMemo, useState, type ReactNode } from 'react';
import type { Certificate, Education, PublicProfile, PublicProject } from '../types';
import { formatExternalLink } from '../utils/links';
import { normalizeMediaPath } from '../utils/media';
import { getSocialItems } from '../utils/social';
import { useSectionSpy } from '../hooks/useSectionSpy';
import ProjectMedia from './portfolio/ProjectMedia';
import SocialLinks from './portfolio/SocialLinks';

interface ProfileProps {
  profile: PublicProfile;
}

interface SectionHeadingProps {
  number: string;
  eyebrow: string;
  title: string;
  description?: string;
}

interface CollectionPagerProps<T> {
  items: T[];
  label: string;
  pageClassName: string;
  renderItem: (item: T, absoluteIndex: number) => ReactNode;
  viewportId: string;
}

const COLLECTION_PAGE_SIZE = 4;

function SectionHeading({ number, eyebrow, title, description }: SectionHeadingProps) {
  return (
    <header className="section-heading">
      <div>
        <span className="section-number">{number}</span>
        <p className="eyebrow">{eyebrow}</p>
      </div>
      <div>
        <h2>{title}</h2>
        {description && <p>{description}</p>}
      </div>
    </header>
  );
}

function CollectionPager<T>({
  items,
  label,
  pageClassName,
  renderItem,
  viewportId,
}: CollectionPagerProps<T>) {
  const [page, setPage] = useState(0);
  const [direction, setDirection] = useState<'forward' | 'backward'>('forward');
  const pageCount = Math.max(1, Math.ceil(items.length / COLLECTION_PAGE_SIZE));
  const safePage = Math.min(page, pageCount - 1);
  const startIndex = safePage * COLLECTION_PAGE_SIZE;
  const endIndex = Math.min(startIndex + COLLECTION_PAGE_SIZE, items.length);
  const visibleItems = items.slice(startIndex, endIndex);

  const showPage = (nextPage: number) => {
    const safeNextPage = Math.min(Math.max(nextPage, 0), pageCount - 1);
    if (safeNextPage === safePage) return;

    setDirection(safeNextPage > safePage ? 'forward' : 'backward');
    setPage(safeNextPage);
  };

  return (
    <div className="collection-frame">
      <div className="collection-toolbar">
        <p className="collection-range" aria-live="polite">
          <span>{String(startIndex + 1).padStart(2, '0')}—{String(endIndex).padStart(2, '0')}</span>
          <small>{label} / {String(items.length).padStart(2, '0')}</small>
        </p>

        {pageCount > 1 && (
          <div className="collection-navigation" aria-label={`${label} navigation`}>
            <button
              type="button"
              onClick={() => showPage(safePage - 1)}
              disabled={safePage === 0}
              aria-label={`Show previous ${label.toLowerCase()}`}
              aria-controls={viewportId}
            >
              <span aria-hidden="true">←</span>
            </button>

            <span className="collection-progress" aria-hidden="true">
              {Array.from({ length: pageCount }, (_, index) => (
                <i className={index === safePage ? 'active' : ''} key={index} />
              ))}
            </span>

            <span className="sr-only">Page {safePage + 1} of {pageCount}</span>

            <button
              type="button"
              onClick={() => showPage(safePage + 1)}
              disabled={safePage === pageCount - 1}
              aria-label={`Show next ${label.toLowerCase()}`}
              aria-controls={viewportId}
            >
              <span aria-hidden="true">→</span>
            </button>
          </div>
        )}
      </div>

      <div className="collection-viewport" id={viewportId}>
        <div
          className={`${pageClassName} collection-page`}
          data-direction={direction}
          key={`${label}-${safePage}`}
        >
          {visibleItems.map((item, index) => renderItem(item, startIndex + index))}
        </div>
      </div>
    </div>
  );
}

function ProjectLinks({ project }: { project: PublicProject }) {
  const links = [
    { label: 'GitHub', href: formatExternalLink(project.github_link) },
    { label: 'Hugging Face', href: formatExternalLink(project.hf_link) },
    { label: 'Live demo', href: formatExternalLink(project.live_demo_link) },
  ].filter((link): link is { label: string; href: string } => Boolean(link.href));

  if (links.length === 0) return null;

  return (
    <div className="project-actions">
      {links.map((link) => (
        <a href={link.href} target="_blank" rel="noreferrer" key={link.label}>
          {link.label}
          <span aria-hidden="true">↗</span>
        </a>
      ))}
    </div>
  );
}

function CertificateCard({ certificate, index }: { certificate: Certificate; index: number }) {
  const image = normalizeMediaPath(certificate.image_path);
  const link = formatExternalLink(certificate.link);

  return (
    <article className="credential-card">
      <div className="credential-visual">
        {image ? (
          <img
            src={image}
            alt={`${certificate.name ?? 'Certificate'} preview`}
            loading="lazy"
            decoding="async"
          />
        ) : (
          <span>{String(index + 1).padStart(2, '0')}</span>
        )}
      </div>
      <div className="credential-copy">
        <span className="credential-index">CERT / {String(index + 1).padStart(2, '0')}</span>
        <h3>{certificate.name ?? 'Certificate'}</h3>
        <p>{[certificate.issuer, certificate.date ?? certificate.year].filter(Boolean).join(' · ')}</p>
        {link && (
          <a href={link} target="_blank" rel="noreferrer">
            View credential <span aria-hidden="true">↗</span>
          </a>
        )}
      </div>
    </article>
  );
}

function EducationCard({ education, index }: { education: Education; index: number }) {
  const link = formatExternalLink(education.link);

  return (
    <article className="education-card">
      <span className="education-marker" aria-hidden="true" />
      <div className="education-date">{education.duration ?? education.year ?? `Entry ${index + 1}`}</div>
      <div>
        <h3>{education.degree ?? education.institution ?? `Education ${index + 1}`}</h3>
        <p className="education-institution">{education.institution}</p>
        <p>{[education.grade, education.status].filter(Boolean).join(' · ')}</p>
        {link && (
          <a href={link} target="_blank" rel="noreferrer">
            View details <span aria-hidden="true">↗</span>
          </a>
        )}
      </div>
    </article>
  );
}
function Profile({ profile }: ProfileProps) {
  const displayName = profile.display_name ?? profile.name ?? 'Chamira Hashan';
  const projects = profile.projects ?? [];
  const skills = profile.skills ?? [];
  const focusAreas = profile.focus_areas ?? [];
  const certificates = profile.certificates ?? [];
  const education = profile.education ?? [];
  const socialItems = getSocialItems(profile.social_links);
  const resume = socialItems.find((item) => item.key === 'resume');
  const profileImage = normalizeMediaPath(profile.profile_image_path);
  const displayBio = profile.bio?.replace(/\bgraduate\b/gi, 'undergraduate');

  const navItems = useMemo(
    () => [

{ id: 'home', label: 'Home', visible: true },
      { id: 'about', label: 'Focus', visible: Boolean(profile.bio || focusAreas.length) },
      { id: 'projects', label: 'Work', visible: projects.length > 0 },
      { id: 'skills', label: 'Skills', visible: skills.length > 0 },
      { id: 'certificates', label: 'Credentials', visible: certificates.length > 0 },
      { id: 'education', label: 'Education', visible: education.length > 0 },
      { id: 'contact', label: 'Contact', visible: socialItems.length > 0 },
    ].filter((item) => item.visible),
    [
      certificates.length,
      education.length,
      focusAreas.length,
      profile.bio,
      projects.length,
      skills.length,
      socialItems.length,
    ],
  );

  const sectionIds = useMemo(() => navItems.map((item) => item.id), [navItems]);
  const activeSection = useSectionSpy(sectionIds);

  return (
    <main className="portfolio-page">
      <div className="ambient-signal-field" aria-hidden="true">
        <span className="ambient-orbit ambient-orbit-north" />
        <span className="ambient-orbit ambient-orbit-south" />
        <span className="ambient-scan" />
      </div>

      <a className="skip-link" href="#portfolio-content">Skip to content</a>

      <div className="site-nav-wrap">
        <nav className="site-nav" aria-label="Portfolio sections">
          <a className="site-mark" href="#home" aria-label={`${displayName} home`}>H7</a>
          <div className="site-nav-links">
            {navItems.map((item) => (
              <a
                className={activeSection === item.id ? 'active' : ''}
                href={`#${item.id}`}
                aria-current={activeSection === item.id ? 'location' : undefined}
                key={item.id}
              >
                {item.label}
              </a>
            ))}
          </div>
        </nav>
      </div>

      <div id="portfolio-content">
        <section className="hero-section" id="home">
          <div className="hero-copy">
            <p className="hero-overline">Software engineering / AI &amp; ML</p>
            <h1>
              <span>{displayName}</span>
              {profile.tagline ?? 'Building practical software with disciplined engineering.'}
            </h1>
            {profile.role && <p className="hero-role">{profile.role}</p>}
            {profile.location && <p className="hero-location">Based in {profile.location}</p>}

            {focusAreas.length > 0 && (
              <ul className="focus-list" aria-label="Professional focus areas">
                {focusAreas.slice(0, 4).map((focus) => <li key={focus}>{focus}</li>)}
              </ul>
            )}

            <div className="hero-actions">
              {projects.length > 0 && <a className="primary-action" href="#projects">View selected work</a>}
              {resume && (
                <a className="secondary-action" href={resume.href} target="_blank" rel="noreferrer">

View CV <span aria-hidden="true">↗</span>
                </a>
              )}
            </div>

            <SocialLinks items={socialItems} exclude={['resume']} className="hero-social-links" />
          </div>

          <div className={`portrait-frame ${profileImage ? 'has-image' : ''}`}>
            <div className="portrait-grid" aria-hidden="true" />
            {profileImage ? (
              <img
                src={profileImage}
                alt={`${displayName} portrait`}
                width="720"
                height="900"
                decoding="async"
                fetchPriority="high"
              />
            ) : (
              <span className="portrait-monogram" aria-label={`${displayName} portrait placeholder`}>H7</span>
            )}
          </div>
        </section>
        {(profile.bio || focusAreas.length > 0) && (
          <section className="portfolio-section about-section" id="about">
            <SectionHeading
              number="02"
              eyebrow="Professional focus"
              title="Turning requirements into dependable, usable systems."
              description="A practical engineering profile grounded in clear boundaries, maintainable implementation, evidence-led iteration, and delivery-aware decisions."
            />
            <div className="about-layout">
              <div className="about-narrative">
                <p className="about-label">Engineering profile / current trajectory</p>
                <p className="about-lead">
                  {displayBio ?? 'A practical software engineering portfolio spanning backend, AI/ML, full-stack, and mobile work.'}
                </p>
                <p className="about-approach">
                  My approach connects system design, implementation, validation, and deployment into one deliberate delivery process—keeping the result useful, explainable, and maintainable.
                </p>
              </div>

              <div className="focus-system">
                <header>
                  <span>Current direction</span>
                  <small>Capability signals</small>
                </header>
                <div className="focus-grid">
                  {focusAreas.map((focus, index) => (
                    <article key={focus}>
                      <span>{String(index + 1).padStart(2, '0')}</span>
                      <h3>{focus}</h3>
                      <i aria-hidden="true" />
                    </article>
                  ))}
                </div>
              </div>

              <div className="delivery-principles">
                <p>Delivery principles</p>
                <ul>
                  <li><span>01</span><strong>Clear system boundaries</strong></li>
                  <li><span>02</span><strong>Evidence-led iteration</strong></li>
                  <li><span>03</span><strong>Production-aware delivery</strong></li>
                </ul>
              </div>

              <dl className="portfolio-counts">
                <div><dt>Projects</dt><dd>{String(projects.length).padStart(2, '0')}</dd></div>
                <div><dt>Technologies</dt><dd>{String(skills.length).padStart(2, '0')}</dd></div>
                <div><dt>Credentials</dt><dd>{String(certificates.length).padStart(2, '0')}</dd></div>
              </dl>
            </div>
          </section>
        )}

        {projects.length > 0 && (
          <section className="portfolio-section projects-section" id="projects">
            <SectionHeading
              number="03"
              eyebrow="Selected work"
              title="Projects built to solve, learn, and ship."
              description="Each project is presented from the live portfolio dataset, with its available code, demo, and media evidence."
            />
            <CollectionPager
              items={projects}
              label="Projects"
              pageClassName="project-list project-page"
              viewportId="project-collection"
              renderItem={(project, index) => (
                <article
                  className={`project-story ${project.featured ? 'featured' : ''}`}
                  key={`${project.title ?? 'project'}-${index}`}
                >
                  <div className="project-copy">
                    <div className="project-meta">
                      <span>PROJECT / {String(index + 1).padStart(2, '0')}</span>
                      {project.category && <span>{project.category}</span>}
                      {project.featured && <span className="featured-label">Featured</span>}
                    </div>
                    <h3>{project.title ?? `Project ${index + 1}`}</h3>
                    {project.short_description && <p>{project.short_description}</p>}
                    {project.tech_stack.length > 0 && (
                      <ul className="technology-list" aria-label={`${project.title ?? 'Project'} technologies`}>
                        {project.tech_stack.map((technology) => <li key={technology}>{technology}</li>)}
                      </ul>
                    )}
                    <ProjectLinks project={project} />
                  </div>
                  <ProjectMedia project={project} />
                </article>
              )}
            />
          </section>
        )}

        {skills.length > 0 && (
          <section className="portfolio-section skills-section" id="skills">
            <SectionHeading
              number="04"
              eyebrow="Technical stack"
              title="A capability system, not a checklist."
              description="Technologies connected across software delivery, backend systems, applied AI, data, and production tooling—selected according to the problem and its constraints."
            />
            <div className="skills-system">
              <header className="skills-system-header">
                <div className="skills-orbit" aria-hidden="true">
                  <i />
                  <span>{String(skills.length).padStart(2, '0')}</span>
                </div>
                <div>
                  <p className="eyebrow">Capability map</p>
                  <p>One working system spanning implementation, intelligence, data, and delivery.</p>
                </div>
              </header>

              <ul className="skills-constellation" aria-label="Technical skills">
                {skills.map((skill, index) => (
                  <li key={skill}>
                    <span className="skill-index">{String(index + 1).padStart(2, '0')}</span>
                    <div>
                      <small>Capability</small>
                      <strong>{skill}</strong>
                    </div>
                    <i className="skill-signal" aria-hidden="true" />
                  </li>
                ))}
              </ul>
            </div>
          </section>
        )}

        {certificates.length > 0 && (
          <section className="portfolio-section credentials-section" id="certificates">
            <SectionHeading
              number="05"
              eyebrow="Credentials"
              title="Verified learning, presented without noise."
            />
            <CollectionPager
              items={certificates}
              label="Certificates"
              pageClassName="credential-grid"
              viewportId="certificate-collection"
              renderItem={(certificate, index) => (
                <CertificateCard
                  certificate={certificate}
                  index={index}
                  key={`${certificate.name ?? 'certificate'}-${index}`}
                />
              )}
            />
          </section>
        )}

        {education.length > 0 && (
          <section className="portfolio-section education-section" id="education">
            <SectionHeading number="06" eyebrow="Education" title="A foundation for continuous practice." />
            <div className="education-list">

{education.map((item, index) => (
                <EducationCard
                  education={item}
                  index={index}
                  key={`${item.institution ?? 'education'}-${index}`}
                />
              ))}
            </div>
          </section>
        )}

        {socialItems.length > 0 && (
          <section className="portfolio-section contact-section" id="contact">
            <div>
              <p className="eyebrow">Contact / Connect</p>
              <h2>Continue the conversation.</h2>
              <p>Use any available public channel below to explore the work or get in touch.</p>
            </div>
            <SocialLinks items={socialItems} className="contact-links" />
          </section>
        )}
      </div>

      <footer className="portfolio-footer">
        <span>© {new Date().getFullYear()} {displayName}</span>
      </footer>
    </main>
  );
}

export default Profile;
