import { useState } from 'react';
import type { Certificate, FullProfile } from '../../types';

interface CertificatesFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
  requestConfirm: (title: string, message: string, onConfirm: () => void) => void;
}

const emptyCertificate: Certificate = {
  name: '',
  issuer: '',
  year: '',
  date: '',
  link: '',
};

function CertificatesForm({ profile, onChange, requestConfirm }: CertificatesFormProps) {
  const certificates = profile.certificates ?? [];
  const [selectedIndex, setSelectedIndex] = useState(0);

  const safeSelectedIndex =
    certificates.length === 0 ? -1 : Math.min(selectedIndex, certificates.length - 1);

  const selectedCertificate =
    safeSelectedIndex >= 0 ? certificates[safeSelectedIndex] : null;

  const updateCertificates = (nextCertificates: Certificate[]) => {
    onChange({
      ...profile,
      certificates: nextCertificates,
    });
  };

  const updateSelectedCertificate = (certificate: Certificate) => {
    if (safeSelectedIndex < 0) {
      return;
    }

    const nextCertificates = [...certificates];
    nextCertificates[safeSelectedIndex] = certificate;
    updateCertificates(nextCertificates);
  };

  const addCertificate = () => {
    const nextCertificates = [...certificates, { ...emptyCertificate }];
    updateCertificates(nextCertificates);
    setSelectedIndex(nextCertificates.length - 1);
  };

  const moveSelectedCertificate = (direction: 'up' | 'down') => {
    if (safeSelectedIndex < 0) {
      return;
    }

    const targetIndex = direction === 'up' ? safeSelectedIndex - 1 : safeSelectedIndex + 1;

    if (targetIndex < 0 || targetIndex >= certificates.length) {
      return;
    }

    const nextCertificates = [...certificates];
    [nextCertificates[safeSelectedIndex], nextCertificates[targetIndex]] = [
      nextCertificates[targetIndex],
      nextCertificates[safeSelectedIndex],
    ];

    updateCertificates(nextCertificates);
    setSelectedIndex(targetIndex);
  };

  const deleteSelectedCertificate = () => {
    if (!selectedCertificate || safeSelectedIndex < 0) {
      return;
    }

    const certificateName =
      selectedCertificate.name || `Certificate ${safeSelectedIndex + 1}`;

    requestConfirm(
      'Delete certificate?',
      `Are you sure you want to delete "${certificateName}"? This action cannot be undone after saving.`,
      () => {
        const nextCertificates = certificates.filter((_, index) => index !== safeSelectedIndex);
        updateCertificates(nextCertificates);
        setSelectedIndex(Math.max(0, safeSelectedIndex - 1));
      },
    );
  };

  return (
    <section className="admin-section clean">
      <div className="admin-form-toolbar">
        <div>
          <h2>Certificate Editor</h2>
          <p className="admin-muted">
            Select one certificate, update it, reorder it, add a new one, or delete the selected certificate.
          </p>
        </div>

        <div className="admin-actions">
          <button type="button" onClick={addCertificate}>
            Add Certificate
          </button>
          <button
            className="admin-secondary-button"
            type="button"
            onClick={() => moveSelectedCertificate('up')}
            disabled={safeSelectedIndex <= 0}
          >
            Move Up
          </button>
          <button
            className="admin-secondary-button"
            type="button"
            onClick={() => moveSelectedCertificate('down')}
            disabled={safeSelectedIndex < 0 || safeSelectedIndex >= certificates.length - 1}
          >
            Move Down
          </button>
          <button
            className="admin-danger-button"
            type="button"
            onClick={deleteSelectedCertificate}
            disabled={!selectedCertificate}
          >
            Delete Selected
          </button>
        </div>
      </div>

      {certificates.length === 0 && (
        <div className="admin-empty-box">
          <p>No certificates yet. Click Add Certificate to create one.</p>
        </div>
      )}

      {certificates.length > 0 && (
        <div className="admin-form">
          <label>
            Select Certificate
            <select
              value={safeSelectedIndex}
              onChange={(event) => setSelectedIndex(Number(event.target.value))}
            >
              {certificates.map((certificate, index) => (
                <option key={`${certificate.name || 'certificate'}-${index}`} value={index}>
                  {index + 1}. {certificate.name || `Untitled Certificate ${index + 1}`}
                </option>
              ))}
            </select>
          </label>

          {selectedCertificate && (
            <div className="admin-form-grid">
              <label>
                Name
                <input
                  type="text"
                  value={selectedCertificate.name ?? ''}
                  onChange={(event) =>
                    updateSelectedCertificate({
                      ...selectedCertificate,
                      name: event.target.value,
                    })
                  }
                  placeholder="Certificate name"
                />
              </label>

              <label>
                Issuer
                <input
                  type="text"
                  value={selectedCertificate.issuer ?? ''}
                  onChange={(event) =>
                    updateSelectedCertificate({
                      ...selectedCertificate,
                      issuer: event.target.value,
                    })
                  }
                  placeholder="Issuer"
                />
              </label>

              <label>
                Date
                <input
                  type="text"
                  value={selectedCertificate.date ?? selectedCertificate.year ?? ''}
                  onChange={(event) =>
                    updateSelectedCertificate({
                      ...selectedCertificate,
                      date: event.target.value,
                      year: undefined,
                    })
                  }
                  placeholder="May 20, 2026"
                />
              </label>

              <label>
                Link
                <input
                  type="url"
                  value={selectedCertificate.link ?? ''}
                  onChange={(event) =>
                    updateSelectedCertificate({
                      ...selectedCertificate,
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

export default CertificatesForm;