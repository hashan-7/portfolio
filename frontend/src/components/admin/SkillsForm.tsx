import type { FullProfile } from '../../types';

interface SkillsFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
}

function textToList(value: string): string[] {
  return value
    .split('\n')
    .map((item) => item.trim())
    .filter(Boolean);
}

function SkillsForm({ profile, onChange }: SkillsFormProps) {
  return (
    <section className="admin-section clean">
      <div className="admin-form-toolbar">
        <div>
          <h2>Skills & Focus Areas</h2>
          <p className="admin-muted">
            Use one item per line. JSON updates automatically while typing.
          </p>
        </div>
      </div>

      <div className="admin-form-grid">
        <label>
          Skills
          <textarea
            className="admin-json-editor compact tall"
            value={(profile.skills ?? []).join('\n')}
            onChange={(event) =>
              onChange({
                ...profile,
                skills: textToList(event.target.value),
              })
            }
            placeholder="Java&#10;Python&#10;Spring Boot"
          />
          <span className="admin-field-help">One skill per line.</span>
        </label>

        <label>
          Focus Areas
          <textarea
            className="admin-json-editor compact tall"
            value={(profile.focus_areas ?? []).join('\n')}
            onChange={(event) =>
              onChange({
                ...profile,
                focus_areas: textToList(event.target.value),
              })
            }
            placeholder="Backend / API Development&#10;AI API Integration"
          />
          <span className="admin-field-help">One focus area per line.</span>
        </label>
      </div>
    </section>
  );
}

export default SkillsForm;