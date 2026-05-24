import { useState } from 'react';
import type { Education, FullProfile } from '../../types';

interface EducationFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
  requestConfirm: (title: string, message: string, onConfirm: () => void) => void;
}

const emptyEducation: Education = {
  institution: '',
  degree: '',
  duration: '',
  year: '',
  grade: '',
  status: '',
  link: '',
};

function EducationForm({ profile, onChange, requestConfirm }: EducationFormProps) {
  const education = profile.education ?? [];
  const [selectedIndex, setSelectedIndex] = useState(0);

  const safeSelectedIndex =
    education.length === 0 ? -1 : Math.min(selectedIndex, education.length - 1);

  const selectedEducation = safeSelectedIndex >= 0 ? education[safeSelectedIndex] : null;

  const updateEducationList = (nextEducation: Education[]) => {
    onChange({
      ...profile,
      education: nextEducation,
    });
  };

  const updateSelectedEducation = (item: Education) => {
    if (safeSelectedIndex < 0) {
      return;
    }

    const nextEducation = [...education];
    nextEducation[safeSelectedIndex] = item;
    updateEducationList(nextEducation);
  };

  const addEducation = () => {
    const nextEducation = [...education, { ...emptyEducation }];
    updateEducationList(nextEducation);
    setSelectedIndex(nextEducation.length - 1);
  };

  const moveSelectedEducation = (direction: 'up' | 'down') => {
    if (safeSelectedIndex < 0) {
      return;
    }

    const targetIndex = direction === 'up' ? safeSelectedIndex - 1 : safeSelectedIndex + 1;

    if (targetIndex < 0 || targetIndex >= education.length) {
      return;
    }

    const nextEducation = [...education];
    [nextEducation[safeSelectedIndex], nextEducation[targetIndex]] = [
      nextEducation[targetIndex],
      nextEducation[safeSelectedIndex],
    ];

    updateEducationList(nextEducation);
    setSelectedIndex(targetIndex);
  };

  const deleteSelectedEducation = () => {
    if (!selectedEducation || safeSelectedIndex < 0) {
      return;
    }

    const educationName =
      selectedEducation.degree ||
      selectedEducation.institution ||
      `Education ${safeSelectedIndex + 1}`;

    requestConfirm(
      'Delete education entry?',
      `Are you sure you want to delete "${educationName}"? This action cannot be undone after saving.`,
      () => {
        const nextEducation = education.filter((_, index) => index !== safeSelectedIndex);
        updateEducationList(nextEducation);
        setSelectedIndex(Math.max(0, safeSelectedIndex - 1));
      },
    );
  };

  return (
    <section className="admin-section clean">
      <div className="admin-form-toolbar">
        <div>
          <h2>Education Editor</h2>
          <p className="admin-muted">
            Select one education entry, update it, reorder it, add a new entry, or delete the selected one.
          </p>
        </div>

        <div className="admin-actions">
          <button type="button" onClick={addEducation}>
            Add Education
          </button>
          <button
            className="admin-secondary-button"
            type="button"
            onClick={() => moveSelectedEducation('up')}
            disabled={safeSelectedIndex <= 0}
          >
            Move Up
          </button>
          <button
            className="admin-secondary-button"
            type="button"
            onClick={() => moveSelectedEducation('down')}
            disabled={safeSelectedIndex < 0 || safeSelectedIndex >= education.length - 1}
          >
            Move Down
          </button>
          <button
            className="admin-danger-button"
            type="button"
            onClick={deleteSelectedEducation}
            disabled={!selectedEducation}
          >
            Delete Selected
          </button>
        </div>
      </div>

      {education.length === 0 && (
        <div className="admin-empty-box">
          <p>No education entries yet. Click Add Education to create one.</p>
        </div>
      )}

      {education.length > 0 && (
        <div className="admin-form">
          <label>
            Select Education
            <select
              value={safeSelectedIndex}
              onChange={(event) => setSelectedIndex(Number(event.target.value))}
            >
              {education.map((item, index) => (
                <option key={`${item.institution || 'education'}-${index}`} value={index}>
                  {index + 1}. {item.degree || item.institution || `Education ${index + 1}`}
                </option>
              ))}
            </select>
          </label>

          {selectedEducation && (
            <div className="admin-form-grid">
              <label>
                Institution
                <input
                  type="text"
                  value={selectedEducation.institution ?? ''}
                  onChange={(event) =>
                    updateSelectedEducation({
                      ...selectedEducation,
                      institution: event.target.value,
                    })
                  }
                  placeholder="Institution name"
                />
              </label>

              <label>
                Program / Degree
                <input
                  type="text"
                  value={selectedEducation.degree ?? ''}
                  onChange={(event) =>
                    updateSelectedEducation({
                      ...selectedEducation,
                      degree: event.target.value,
                    })
                  }
                  placeholder="Diploma in Software Engineering"
                />
              </label>

              <label>
                Duration
                <input
                  type="text"
                  value={selectedEducation.duration ?? selectedEducation.year ?? ''}
                  onChange={(event) =>
                    updateSelectedEducation({
                      ...selectedEducation,
                      duration: event.target.value,
                      year: undefined,
                    })
                  }
                  placeholder="Oct 2023 – Dec 2024"
                />
              </label>

              <label>
                Grade
                <input
                  type="text"
                  value={selectedEducation.grade ?? ''}
                  onChange={(event) =>
                    updateSelectedEducation({
                      ...selectedEducation,
                      grade: event.target.value,
                    })
                  }
                  placeholder="GPA 3.5/4.0"
                />
              </label>

              <label>
                Status
                <input
                  type="text"
                  value={selectedEducation.status ?? ''}
                  onChange={(event) =>
                    updateSelectedEducation({
                      ...selectedEducation,
                      status: event.target.value,
                    })
                  }
                  placeholder="Awarded / Academic work completed"
                />
              </label>

              <label>
                Link
                <input
                  type="url"
                  value={selectedEducation.link ?? ''}
                  onChange={(event) =>
                    updateSelectedEducation({
                      ...selectedEducation,
                      link: event.target.value,
                    })
                  }
                  placeholder="https://..."
                />
              </label>
            </div>
          )}
        </div>
      )}
    </section>
  );
}

export default EducationForm;