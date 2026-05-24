import type { FullProfile, SocialLinks } from '../../types';

interface ContactFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
}

const emptyLinks: SocialLinks = {
  github: '',
  linkedin: '',
  email: '',
  phone: '',
  website: '',
  huggingface: '',
  kaggle: '',
  resume: '',
  instagram: '',
};

function ContactForm({ profile, onChange }: ContactFormProps) {
  const links = profile.social_links ?? emptyLinks;

  const updateLinks = (nextLinks: SocialLinks) => {
    onChange({
      ...profile,
      social_links: nextLinks,
    });
  };

  return (
    <section className="admin-section clean">
      <div className="admin-form-toolbar">
        <div>
          <h2>Contact & Links</h2>
          <p className="admin-muted">
            Add only the links you want available through the portfolio UI or chatbot.
          </p>
        </div>
      </div>

      <div className="admin-form-grid">
        <label>
          Email
          <input
            type="email"
            value={links.email ?? ''}
            onChange={(event) =>
              updateLinks({
                ...links,
                email: event.target.value,
              })
            }
            placeholder="example@email.com"
          />
        </label>

        <label>
          Phone
          <input
            type="text"
            value={links.phone ?? ''}
            onChange={(event) =>
              updateLinks({
                ...links,
                phone: event.target.value,
              })
            }
            placeholder="076..."
          />
        </label>

        <label>
          GitHub
          <input
            type="url"
            value={links.github ?? ''}
            onChange={(event) =>
              updateLinks({
                ...links,
                github: event.target.value,
              })
            }
            placeholder="https://github.com/..."
          />
        </label>

        <label>
          LinkedIn
          <input
            type="url"
            value={links.linkedin ?? ''}
            onChange={(event) =>
              updateLinks({
                ...links,
                linkedin: event.target.value,
              })
            }
            placeholder="https://linkedin.com/in/..."
          />
        </label>

        <label>
          Hugging Face
          <input
            type="url"
            value={links.huggingface ?? ''}
            onChange={(event) =>
              updateLinks({
                ...links,
                huggingface: event.target.value,
              })
            }
            placeholder="https://huggingface.co/..."
          />
        </label>

        <label>
          Kaggle
          <input
            type="url"
            value={links.kaggle ?? ''}
            onChange={(event) =>
              updateLinks({
                ...links,
                kaggle: event.target.value,
              })
            }
            placeholder="https://www.kaggle.com/..."
          />
        </label>

        <label>
          Resume
          <input
            type="url"
            value={links.resume ?? ''}
            onChange={(event) =>
              updateLinks({
                ...links,
                resume: event.target.value,
              })
            }
            placeholder="https://drive.google.com/..."
          />
        </label>

        <label>
          Instagram
          <input
            type="url"
            value={links.instagram ?? ''}
            onChange={(event) =>
              updateLinks({
                ...links,
                instagram: event.target.value,
              })
            }
            placeholder="https://instagram.com/..."
          />
        </label>

        <label className="wide">
          Website / Portfolio URL
          <input
            type="url"
            value={links.website ?? ''}
            onChange={(event) =>
              updateLinks({
                ...links,
                website: event.target.value,
              })
            }
            placeholder="https://..."
          />
        </label>
      </div>
    </section>
  );
}

export default ContactForm;