import type { FullProfile, SocialLinks } from '../../types';

interface ContactFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
}

function ContactForm({ profile, onChange }: ContactFormProps) {
  const socialLinks = profile.social_links ?? {};

  const updateSocial = (field: keyof SocialLinks, value: string) => {
    onChange({
      ...profile,
      social_links: {
        ...socialLinks,
        [field]: value || undefined,
      },
    });
  };

  return (
    <section className="admin-section">
      <h2>Contact & Links</h2>
      <p className="admin-muted">These links are shown as public contact/link buttons.</p>

      <div className="admin-form">
        <label>
          Email
          <input
            type="email"
            value={socialLinks.email ?? ''}
            onChange={(event) => updateSocial('email', event.target.value)}
            placeholder="email@example.com"
          />
        </label>

        <label>
          Phone
          <input
            type="text"
            value={socialLinks.phone ?? ''}
            onChange={(event) => updateSocial('phone', event.target.value)}
            placeholder="07XXXXXXXX"
          />
        </label>

        <label>
          GitHub
          <input
            type="url"
            value={socialLinks.github ?? ''}
            onChange={(event) => updateSocial('github', event.target.value)}
            placeholder="https://github.com/..."
          />
        </label>

        <label>
          LinkedIn
          <input
            type="url"
            value={socialLinks.linkedin ?? ''}
            onChange={(event) => updateSocial('linkedin', event.target.value)}
            placeholder="https://linkedin.com/in/..."
          />
        </label>

        <label>
          Hugging Face
          <input
            type="url"
            value={socialLinks.huggingface ?? ''}
            onChange={(event) => updateSocial('huggingface', event.target.value)}
            placeholder="https://huggingface.co/..."
          />
        </label>

        <label>
          Kaggle
          <input
            type="url"
            value={socialLinks.kaggle ?? ''}
            onChange={(event) => updateSocial('kaggle', event.target.value)}
            placeholder="https://www.kaggle.com/..."
          />
        </label>

        <label>
          Resume
          <input
            type="url"
            value={socialLinks.resume ?? ''}
            onChange={(event) => updateSocial('resume', event.target.value)}
            placeholder="https://..."
          />
        </label>

        <label>
          Instagram
          <input
            type="url"
            value={socialLinks.instagram ?? ''}
            onChange={(event) => updateSocial('instagram', event.target.value)}
            placeholder="https://instagram.com/..."
          />
        </label>

        <label>
          Website
          <input
            type="url"
            value={socialLinks.website ?? ''}
            onChange={(event) => updateSocial('website', event.target.value)}
            placeholder="https://..."
          />
        </label>
      </div>
    </section>
  );
}

export default ContactForm;