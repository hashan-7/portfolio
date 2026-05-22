import type { Profile as ProfileData } from '../types';

interface ProfileProps {
  profile: ProfileData;
}

function Profile({ profile }: ProfileProps) {
  const skills = profile.skills ?? [];
  const projects = profile.projects ?? [];
  const certificates = profile.certificates ?? [];
  const socialLinks = profile.social_links;

  return (
    <main className="portfolio">
      <section className="hero-section">
        <p className="eyebrow">AI Powered Portfolio</p>
        <h1>{profile.name ?? 'Portfolio Owner'}</h1>
        {profile.role && <h2>{profile.role}</h2>}
        {profile.bio && <p className="bio">{profile.bio}</p>}

        {socialLinks && (
          <div className="social-links">
            {socialLinks.github && (
              <a href={socialLinks.github} target="_blank" rel="noreferrer">
                GitHub
              </a>
            )}
            {socialLinks.linkedin && (
              <a href={socialLinks.linkedin} target="_blank" rel="noreferrer">
                LinkedIn
              </a>
            )}
            {socialLinks.email && <a href={`mailto:${socialLinks.email}`}>Email</a>}
            {socialLinks.website && (
              <a href={socialLinks.website} target="_blank" rel="noreferrer">
                Website
              </a>
            )}
          </div>
        )}
      </section>

      {skills.length > 0 && (
        <section className="section">
          <div className="section-header">
            <p className="eyebrow">Skills</p>
            <h2>Technical Skills</h2>
          </div>
          <div className="chip-list">
            {skills.map((skill) => (
              <span className="chip" key={skill}>
                {skill}
              </span>
            ))}
          </div>
        </section>
      )}

      {projects.length > 0 && (
        <section className="section">
          <div className="section-header">
            <p className="eyebrow">Projects</p>
            <h2>Featured Projects</h2>
            <p>{projects.length} project{projects.length === 1 ? '' : 's'} listed</p>
          </div>

          <div className="card-grid">
            {projects.map((project, index) => (
              <article className="card" key={`${project.title ?? 'project'}-${index}`}>
                {project.title && <h3>{project.title}</h3>}
                {project.description && <p>{project.description}</p>}

                {project.tech_stack && project.tech_stack.length > 0 && (
                  <div className="chip-list small">
                    {project.tech_stack.map((tech) => (
                      <span className="chip" key={tech}>
                        {tech}
                      </span>
                    ))}
                  </div>
                )}

                {project.link && (
                  <a className="card-link" href={project.link} target="_blank" rel="noreferrer">
                    View Project
                  </a>
                )}
              </article>
            ))}
          </div>
        </section>
      )}

      {certificates.length > 0 && (
        <section className="section">
          <div className="section-header">
            <p className="eyebrow">Certificates</p>
            <h2>Certificates</h2>
            <p>{certificates.length} certificate{certificates.length === 1 ? '' : 's'} listed</p>
          </div>

          <div className="card-grid">
            {certificates.map((certificate, index) => (
              <article className="card" key={`${certificate.name ?? 'certificate'}-${index}`}>
                {certificate.name && <h3>{certificate.name}</h3>}
                {(certificate.issuer || certificate.year) && (
                  <p>
                    {[certificate.issuer, certificate.year].filter(Boolean).join(' • ')}
                  </p>
                )}
                {certificate.link && (
                  <a className="card-link" href={certificate.link} target="_blank" rel="noreferrer">
                    View Certificate
                  </a>
                )}
              </article>
            ))}
          </div>
        </section>
      )}
    </main>
  );
}

export default Profile;