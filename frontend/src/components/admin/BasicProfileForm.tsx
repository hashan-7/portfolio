import type { FullProfile } from '../../types';

interface BasicProfileFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
}

function BasicProfileForm({ profile, onChange }: BasicProfileFormProps) {
  return (
    <section className="admin-section clean">
      <div className="admin-form-toolbar">
        <div>
          <h2>Basic Profile</h2>
          <p className="admin-muted">
            Edit the public identity, title, location, tagline, and bio.
          </p>
        </div>
      </div>

      <div className="admin-form-grid">
        <label>
          Internal Full Name
          <input
            type="text"
            value={profile.name ?? ''}
            onChange={(event) =>
              onChange({
                ...profile,
                name: event.target.value,
              })
            }
            placeholder="H. K. Chamira Hashan"
          />
        </label>

        <label>
          Public Display Name
          <input
            type="text"
            value={profile.display_name ?? ''}
            onChange={(event) =>
              onChange({
                ...profile,
                display_name: event.target.value,
              })
            }
            placeholder="Chamira Hashan"
          />
        </label>

        <label className="wide">
          Role / Title
          <input
            type="text"
            value={profile.role ?? ''}
            onChange={(event) =>
              onChange({
                ...profile,
                role: event.target.value,
              })
            }
            placeholder="Aspiring Backend & AI/ML Developer | Software Engineering Background"
          />
        </label>

        <label>
          Location
          <input
            type="text"
            value={profile.location ?? ''}
            onChange={(event) =>
              onChange({
                ...profile,
                location: event.target.value,
              })
            }
            placeholder="Sri Lanka"
          />
        </label>

        <label>
          Tagline
          <input
            type="text"
            value={profile.tagline ?? ''}
            onChange={(event) =>
              onChange({
                ...profile,
                tagline: event.target.value,
              })
            }
            placeholder="Building practical backend, AI/ML, and full-stack software projects."
          />
        </label>

        <label className="wide">
          Bio
          <textarea
            className="admin-json-editor compact tall"
            value={profile.bio ?? ''}
            onChange={(event) =>
              onChange({
                ...profile,
                bio: event.target.value,
              })
            }
            placeholder="Short professional bio..."
          />
        </label>
      </div>
    </section>
  );
}

export default BasicProfileForm;