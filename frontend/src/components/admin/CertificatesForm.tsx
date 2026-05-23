import type { Certificate, FullProfile } from '../../types';

interface CertificatesFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
}

const emptyCertificate: Certificate = {
  name: '',
  issuer: '',
  year: '',
  date: '',
  link: '',
};

function CertificatesForm({ profile, onChange }: CertificatesFormProps) {
  const certificates = profile.certificates ?? [];

  const updateCertificates = (nextCertificates: Certificate[]) => {
    onChange({
      ...profile,
      certificates: nextCertificates,
    });
  };

  const updateCertificate = (index: number, certificate: Certificate) => {
    const nextCertificates = [...certificates];
    nextCertificates[index] = certificate;
    updateCertificates(nextCertificates);
  };

  return (
    <section className="admin-section">
      <h2>Certificates</h2>
      <p className="admin-muted">Use verified certificates or official proof links only.</p>

      <div className="admin-actions">
        <button type="button" onClick={() => updateCertificates([...certificates, emptyCertificate])}>
          Add Certificate
        </button>
      </div>

      <div className="admin-form">
        {certificates.map((certificate, index) => (
          <div className="admin-nested-card" key={`${certificate.name ?? 'certificate'}-${index}`}>
            <label>
              Name
              <input
                type="text"
                value={certificate.name ?? ''}
                onChange={(event) =>
                  updateCertificate(index, {
                    ...certificate,
                    name: event.target.value,
                  })
                }
              />
            </label>

            <label>
              Issuer
              <input
                type="text"
                value={certificate.issuer ?? ''}
                onChange={(event) =>
                  updateCertificate(index, {
                    ...certificate,
                    issuer: event.target.value,
                  })
                }
              />
            </label>

            <label>
              Date
              <input
                type="text"
                value={certificate.date ?? certificate.year ?? ''}
                onChange={(event) =>
                  updateCertificate(index, {
                    ...certificate,
                    date: event.target.value,
                    year: undefined,
                  })
                }
              />
            </label>

            <label>
              Link
              <input
                type="url"
                value={certificate.link ?? ''}
                onChange={(event) =>
                  updateCertificate(index, {
                    ...certificate,
                    link: event.target.value,
                  })
                }
              />
            </label>

            <button
              className="admin-secondary-button"
              type="button"
              onClick={() => updateCertificates(certificates.filter((_, itemIndex) => itemIndex !== index))}
            >
              Remove
            </button>
          </div>
        ))}
      </div>
    </section>
  );
}

export default CertificatesForm;