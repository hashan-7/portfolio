import { useState, type ChangeEvent } from 'react';
import { uploadMedia } from '../../services/api';
import type { FullProfile } from '../../types';

interface BasicProfileFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
}

function BasicProfileForm({ profile, onChange }: BasicProfileFormProps) {
  const [isUploading, setIsUploading] = useState(false);
  const [uploadMessage, setUploadMessage] = useState('');
  const [uploadError, setUploadError] = useState('');

  const updateField = (field: keyof FullProfile, value: string | undefined) => {
    onChange({
      ...profile,
      [field]: value,
    });
  };

  const handleProfileImageUpload = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    event.target.value = '';

    if (!file) return;

    setIsUploading(true);
    setUploadMessage('');
    setUploadError('');

    try {
      const paths = await uploadMedia(file);
      const imagePath = paths.find((path) => /\.(png|jpg|jpeg|webp|gif)$/i.test(path));

      if (!imagePath) {
        setUploadError('Upload completed, but no image path was returned.');
        return;
      }

      updateField('profile_image_path', imagePath);
      setUploadMessage('Profile image uploaded and selected.');
    } catch (error) {
      setUploadError(error instanceof Error ? error.message : 'Failed to upload profile image.');
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <section className="admin-section clean">
      <div className="admin-form-toolbar">
        <div>
          <h2>Basic Profile</h2>
          <p className="admin-muted">
            Edit the public identity, profile image, title, location, tagline, and bio.
          </p>
        </div>
      </div>

      <div className="admin-profile-image-panel">
        <div>
          <h3>Profile Image</h3>
          <p className="admin-muted">
            Upload a portrait image. The saved path appears in the public hero section.
          </p>

          <div className="admin-profile-image-actions">
            <label className="admin-file-button">
              {isUploading ? 'Uploading...' : 'Upload Profile Image'}
              <input
                type="file"
                accept="image/png,image/jpeg,image/webp,image/gif"
                onChange={handleProfileImageUpload}
                disabled={isUploading}
              />
            </label>

            <button
              className="admin-secondary-button"
              type="button"
              onClick={() => updateField('profile_image_path', '')}
              disabled={!profile.profile_image_path}
            >
              Remove Image
            </button>
          </div>

          <label className="admin-image-path-field">
            Profile Image Path
            <input
              type="text"
              value={profile.profile_image_path ?? ''}
              onChange={(event) => updateField('profile_image_path', event.target.value)}
              placeholder="/media/projects/images/profile.webp"
            />
          </label>
        </div>

        <div className="admin-profile-image-preview">
          {profile.profile_image_path ? <img src={profile.profile_image_path} alt="Profile preview" /> : <span>H7</span>}
        </div>
      </div>

      {uploadMessage && <p className="admin-success">{uploadMessage}</p>}
      {uploadError && <p className="admin-error">{uploadError}</p>}

      <div className="admin-form-grid">
        <label>
          Internal Full Name
          <input
            type="text"
            value={profile.name ?? ''}
            onChange={(event) => updateField('name', event.target.value)}
            placeholder="H. K. Chamira Hashan"
          />
        </label>

        <label>
          Public Display Name
          <input
            type="text"
            value={profile.display_name ?? ''}
            onChange={(event) => updateField('display_name', event.target.value)}
            placeholder="Chamira Hashan"
          />
        </label>

        <label className="wide">
          Role / Title
          <input
            type="text"
            value={profile.role ?? ''}
            onChange={(event) => updateField('role', event.target.value)}
            placeholder="Aspiring Backend & AI/ML Developer | Software Engineering Background"
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
          Tagline
          <input
            type="text"
            value={profile.tagline ?? ''}
            onChange={(event) => updateField('tagline', event.target.value)}
            placeholder="Building practical backend, AI/ML, and full-stack software projects."
          />
        </label>

        <label className="wide">
          Bio
          <textarea
            className="admin-json-editor compact tall"
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
