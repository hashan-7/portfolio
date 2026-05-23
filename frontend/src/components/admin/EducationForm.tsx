import type { Education, FullProfile } from '../../types';

interface EducationFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
}

const emptyEducation: Education = {
  institution: '',
  degree: '',
  duration: '',
  grade: '',
  status: '',
  link: '',
};

function EducationForm({ profile, onChange }: EducationFormProps) {
  const education = profile.education ?? [];

  const updateEducationList = (nextEducation: Education[]) => {
    onChange({
      ...profile,
      education: nextEducation,
    });
  };

  const updateEducation = (index: number, item: Education) => {
    const nextEducation = [...education];
    nextEducation[index] = item;
    updateEducationList(nextEducation);
  };

  return (
    <section className="admin-section">
      <h2>Education</h2>
      <p className="admin-muted">Keep this short for public display.</p>

      <div className="admin-actions">
        <button type="button" onClick={() => updateEducationList([...education, emptyEducation])}>
          Add Education
        </button>
      </div>

      <div className="admin-form">
        {education.map((item, index) => (
          <div className="admin-nested-card" key={`${item.institution ?? 'education'}-${index}`}>
            <label>
              Institution
              <input
                type="text"
                value={item.institution ?? ''}
                onChange={(event) =>
                  updateEducation(index, {
                    ...item,
                    institution: event.target.value,
                  })
                }
              />
            </label>

            <label>
              Degree / Program
              <input
                type="text"
                value={item.degree ?? ''}
                onChange={(event) =>
                  updateEducation(index, {
                    ...item,
                    degree: event.target.value,
                  })
                }
              />
            </label>

            <label>
              Duration
              <input
                type="text"
                value={item.duration ?? item.year ?? ''}
                onChange={(event) =>
                  updateEducation(index, {
                    ...item,
                    duration: event.target.value,
                    year: undefined,
                  })
                }
              />
            </label>

            <label>
              Grade
              <input
                type="text"
                value={item.grade ?? ''}
                onChange={(event) =>
                  updateEducation(index, {
                    ...item,
                    grade: event.target.value,
                  })
                }
              />
            </label>

            <label>
              Status
              <input
                type="text"
                value={item.status ?? ''}
                onChange={(event) =>
                  updateEducation(index, {
                    ...item,
                    status: event.target.value,
                  })
                }
              />
            </label>

            <label>
              Link
              <input
                type="url"
                value={item.link ?? ''}
                onChange={(event) =>
                  updateEducation(index, {
                    ...item,
                    link: event.target.value,
                  })
                }
              />
            </label>

            <button
              className="admin-secondary-button"
              type="button"
              onClick={() => updateEducationList(education.filter((_, itemIndex) => itemIndex !== index))}
            >
              Remove
            </button>
          </div>
        ))}
      </div>
    </section>
  );
}

export default EducationForm;