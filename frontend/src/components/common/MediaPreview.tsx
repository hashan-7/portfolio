import { isVideoPath } from '../../utils/media';

interface MediaPreviewProps {
  src?: string;
  alt?: string;
}

function MediaPreview({ src, alt = 'Media preview' }: MediaPreviewProps) {
  if (!src) {
    return null;
  }

  return (
    <div className="card-media">
      {isVideoPath(src) ? (
        <video src={src} autoPlay muted loop playsInline />
      ) : (
        <img src={src} alt={alt} />
      )}
    </div>
  );
}

export default MediaPreview;