import type { FullProfile } from '../../types';

interface BasicProfileFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
}

function BasicProfileForm({ profile, onChange }: BasicProfileFormProps) {
  const updateField = (field: keyof FullProfile, value: string) => {
    onChange({
      ...profile,
      [field]: value || undefined,
    });
  };

  return (
    <section className="admin-section">
      <h2>Basic Profile</h2>
      <p className="admin-muted">Public profile details shown on the portfolio UI.</p>

      <div className="admin-form">
        <label>
          Full Name
          <input
            type="text"
            value={profile.name ?? ''}
            onChange={(event) => updateField('name', event.target.value)}
            placeholder="Chamira Hashan"
          />
        </label>

        <label>
          Display Name
          <input
            type="text"
            value={profile.display_name ?? ''}
            onChange={(event) => updateField('display_name', event.target.value)}
            placeholder="Chamira Hashan"
          />
        </label>

        <label>
          Role / Title
          <input
            type="text"
            value={profile.role ?? ''}
            onChange={(event) => updateField('role', event.target.value)}
            placeholder="Aspiring Backend & AI/ML Developer"
          />
        </label>

        <label>
          Tagline
          <input
            type="text"
            value={profile.tagline ?? ''}
            onChange={(event) => updateField('tagline', event.target.value)}
            placeholder="Building practical backend, AI/ML, and full-stack software projects."
          />
        </label>

        <label>
          Location
          <input
            type="text"
            value={profile.location ?? ''}
            onChange={(event) => updateField('location', event.target.value)}
            placeholder="Sri Lanka"
          />
        </label>

        <label>
          Bio
          <textarea
            className="admin-json-editor compact"
            value={profile.bio ?? ''}
            onChange={(event) => updateField('bio', event.target.value)}
            placeholder="Short professional bio..."
          />
        </label>
      </div>
    </section>
  );
}

export default BasicProfileForm;