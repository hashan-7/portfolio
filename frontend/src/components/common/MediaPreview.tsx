import { isVideoPath, normalizeMediaPath } from '../../utils/media';

interface MediaPreviewProps {
  src?: string;
  alt?: string;
  className?: string;
}

function MediaPreview({ src, alt = 'Media preview', className = 'card-media' }: MediaPreviewProps) {
  const safeSrc = normalizeMediaPath(src);

  if (!safeSrc) return null;

  return (
    <div className={className}>
      {isVideoPath(safeSrc) ? (
        <video src={safeSrc} autoPlay muted loop playsInline />
      ) : (
        <img src={safeSrc} alt={alt} />
      )}
    </div>
  );
}

export default MediaPreview;
