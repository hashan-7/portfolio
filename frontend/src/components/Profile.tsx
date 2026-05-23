import type { PublicProfile } from '../types';

interface ProfileProps {
  profile: PublicProfile;
}

function Profile({ profile }: ProfileProps) {
  const skills = profile.skills ?? [];
  const focusAreas = profile.focus_areas ?? [];
  const projects = profile.projects ?? [];
  const certificates = profile.certificates ?? [];
  const education = profile.education ?? [];
  const socialLinks = profile.social_links;
  const displayName = profile.display_name ?? profile.name ?? 'Portfolio Owner';

  return (
    <main className="portfolio">
      <section className="hero-section">
        <p className="eyebrow">AI Powered Portfolio</p>
        <h1>{displayName}</h1>
        {profile.role && <h2>{profile.role}</h2>}
        {profile.tagline && <p className="bio">{profile.tagline}</p>}
        {profile.bio && <p className="bio">{profile.bio}</p>}
        {profile.location && <p className="profile-location">{profile.location}</p>}

        {socialLinks && (
          <div className="social-links">
            {socialLinks.email && <a href={`mailto:${socialLinks.email}`}>Email</a>}
            {socialLinks.phone && <a href={`tel:${socialLinks.phone}`}>Phone</a>}
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
            {socialLinks.huggingface && (
              <a href={socialLinks.huggingface} target="_blank" rel="noreferrer">
                Hugging Face
              </a>
            )}
            {socialLinks.kaggle && (
              <a href={socialLinks.kaggle} target="_blank" rel="noreferrer">
                Kaggle
              </a>
            )}
            {socialLinks.resume && (
              <a href={socialLinks.resume} target="_blank" rel="noreferrer">
                Resume
              </a>
            )}
            {socialLinks.instagram && (
              <a href={socialLinks.instagram} target="_blank" rel="noreferrer">
                Instagram
              </a>
            )}
            {socialLinks.website && (
              <a href={socialLinks.website} target="_blank" rel="noreferrer">
                Website
              </a>
            )}
          </div>
        )}
      </section>

      {focusAreas.length > 0 && (
        <section className="section">
          <div className="section-header">
            <p className="eyebrow">Focus</p>
            <h2>What I Build</h2>
          </div>
          <div className="chip-list">
            {focusAreas.map((area) => (
              <span className="chip focus-chip" key={area}>
                {area}
              </span>
            ))}
          </div>
        </section>
      )}

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
            <p>
              {projects.length} project{projects.length === 1 ? '' : 's'} listed
            </p>
          </div>

          <div className="card-grid">
            {projects.map((project, index) => (
              <article className="card" key={`${project.title ?? 'project'}-${index}`}>
                {project.video_path && (
                  <div className="card-media">
                    <video src={project.video_path} autoPlay muted loop playsInline />
                  </div>
                )}

                {!project.video_path && project.image_path && (
                  <div className="card-media">
                    <img src={project.image_path} alt={project.title ?? 'Project preview'} />
                  </div>
                )}

                {project.category && <p className="card-category">{project.category}</p>}
                {project.title && <h3>{project.title}</h3>}
                {project.short_description && <p>{project.short_description}</p>}

                {project.tech_stack.length > 0 && (
                  <div className="chip-list small">
                    {project.tech_stack.map((tech) => (
                      <span className="chip" key={tech}>
                        {tech}
                      </span>
                    ))}
                  </div>
                )}

                <div className="card-action-links">
                  {project.github_link && (
                    <a className="card-link" href={project.github_link} target="_blank" rel="noreferrer">
                      GitHub
                    </a>
                  )}
                  {project.hf_link && (
                    <a className="card-link" href={project.hf_link} target="_blank" rel="noreferrer">
                      HF Space
                    </a>
                  )}
                  {project.live_demo_link && (
                    <a
                      className="card-link"
                      href={project.live_demo_link}
                      target="_blank"
                      rel="noreferrer"
                    >
                      Live Demo
                    </a>
                  )}
                </div>
              </article>
            ))}
          </div>
        </section>
      )}

      {education.length > 0 && (
        <section className="section">
          <div className="section-header">
            <p className="eyebrow">Education</p>
            <h2>Education</h2>
          </div>

          <div className="card-grid">
            {education.map((edu, index) => (
              <article className="card" key={`${edu.institution ?? 'edu'}-${index}`}>
                {edu.degree && <h3>{edu.degree}</h3>}
                <p>
                  {[edu.institution, edu.duration ?? edu.year, edu.grade, edu.status]
                    .filter(Boolean)
                    .join(' • ')}
                </p>
                {edu.link && (
                  <a className="card-link" href={edu.link} target="_blank" rel="noreferrer">
                    View
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
            <h2>Certificates & Learning</h2>
            <p>
              {certificates.length} certificate{certificates.length === 1 ? '' : 's'} listed
            </p>
          </div>

          <div className="card-grid">
            {certificates.map((certificate, index) => (
              <article className="card" key={`${certificate.name ?? 'certificate'}-${index}`}>
                {certificate.name && <h3>{certificate.name}</h3>}
                <p>
                  {[certificate.issuer, certificate.date ?? certificate.year].filter(Boolean).join(' • ')}
                </p>
                {certificate.link && (
                  <a className="card-link" href={certificate.link} target="_blank" rel="noreferrer">
                    Verify
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