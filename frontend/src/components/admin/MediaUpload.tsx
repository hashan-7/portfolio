import { useState, type ChangeEvent } from 'react';
import { uploadMedia } from '../../services/api';

interface MediaUploadProps {
  onUploaded?: (paths: string[]) => void;
}

function MediaUpload({ onUploaded }: MediaUploadProps) {
  const [isUploading, setIsUploading] = useState(false);
  const [statusMessage, setStatusMessage] = useState('');
  const [errorMessage, setErrorMessage] = useState('');

  const handleFileChange = async (event: ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(event.target.files ?? []);
    event.target.value = '';

    if (files.length === 0) return;

    setIsUploading(true);
    setStatusMessage('');
    setErrorMessage('');

    try {
      const uploadedPaths: string[] = [];

      for (const file of files) {
        const paths = await uploadMedia(file);
        uploadedPaths.push(...paths);
      }

      setStatusMessage(`Uploaded: ${uploadedPaths.join(', ')}`);
      onUploaded?.(uploadedPaths);
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : 'Upload failed.');
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <section className="admin-section clean">
      <h2>Media Upload</h2>
      <p className="admin-muted">
        Upload project images, project videos, or profile images. Admin forms can save the returned media paths.
      </p>

      <label className="admin-file-button">
        {isUploading ? 'Uploading...' : 'Upload Media'}
        <input
          type="file"
          accept="image/png,image/jpeg,image/webp,image/gif,video/mp4,video/webm,video/quicktime"
          onChange={handleFileChange}
          disabled={isUploading}
          multiple
        />
      </label>

      {statusMessage && <p className="admin-success">{statusMessage}</p>}
      {errorMessage && <p className="admin-error">{errorMessage}</p>}
    </section>
  );
}

export default MediaUpload;
