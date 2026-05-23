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

function listToText(value: string[]): string {
  return value.join('\n');
}

function SkillsForm({ profile, onChange }: SkillsFormProps) {
  return (
    <section className="admin-section">
      <h2>Skills & Focus Areas</h2>
      <p className="admin-muted">Enter one item per line.</p>

      <div className="admin-form">
        <label>
          Skills
          <textarea
            className="admin-json-editor compact"
            value={listToText(profile.skills ?? [])}
            onChange={(event) =>
              onChange({
                ...profile,
                skills: textToList(event.target.value),
              })
            }
            placeholder="Java&#10;Python&#10;Spring Boot"
          />
        </label>

        <label>
          Focus Areas
          <textarea
            className="admin-json-editor compact"
            value={listToText(profile.focus_areas ?? [])}
            onChange={(event) =>
              onChange({
                ...profile,
                focus_areas: textToList(event.target.value),
              })
            }
            placeholder="Backend / API Development&#10;AI API Integration"
          />
        </label>
      </div>
    </section>
  );
}

export default SkillsForm;