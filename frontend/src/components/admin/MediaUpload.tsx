import { useState } from 'react';
import { uploadMedia } from '../../services/api';

interface MediaUploadProps {
  onUploaded?: (paths: string[]) => void;
}

function MediaUpload({ onUploaded }: MediaUploadProps) {
  const [isUploading, setIsUploading] = useState(false);
  const [statusMessage, setStatusMessage] = useState('');
  const [errorMessage, setErrorMessage] = useState('');

  const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];

    if (!file) {
      return;
    }

    setIsUploading(true);
    setStatusMessage('');
    setErrorMessage('');

    try {
      const paths = await uploadMedia(file);
      setStatusMessage(`Uploaded: ${paths.join(', ')}`);
      onUploaded?.(paths);
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : 'Upload failed.');
    } finally {
      setIsUploading(false);
      event.target.value = '';
    }
  };

  return (
    <section className="admin-section">
      <h2>Media Upload</h2>
      <p className="admin-muted">
        Upload project images or videos. Copy the returned path into a project image_path or video_path.
      </p>

      <input
        type="file"
        accept="image/png,image/jpeg,image/webp,image/gif,video/mp4,video/webm,video/quicktime"
        onChange={handleFileChange}
        disabled={isUploading}
      />

      {statusMessage && <p className="admin-success">{statusMessage}</p>}
      {errorMessage && <p className="admin-error">{errorMessage}</p>}
    </section>
  );
}

export default MediaUpload;