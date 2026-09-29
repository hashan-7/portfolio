import { useEffect, useMemo, useRef, useState } from 'react';
import type { PublicProject } from '../../types';
import { normalizeMediaPath, normalizeMediaPaths } from '../../utils/media';

interface ProjectMediaProps {
  project: PublicProject;
}

interface MediaItem {
  kind: 'image' | 'video';
  src: string;
  label: string;
}

function ProjectMedia({ project }: ProjectMediaProps) {
  const videoRef = useRef<HTMLVideoElement | null>(null);
  const frameRef = useRef<HTMLDivElement | null>(null);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [failedSources, setFailedSources] = useState<string[]>([]);

  const mediaItems = useMemo<MediaItem[]>(() => {
    const imagePaths = normalizeMediaPaths(
      project.image_paths?.length ? project.image_paths : project.image_path ? [project.image_path] : [],
    ).slice(0, 3);
    const videoPath = normalizeMediaPath(project.video_path);

    return [
      ...imagePaths.map((src, index) => ({ kind: 'image' as const, src, label: `Image ${index + 1}` })),
      ...(videoPath ? [{ kind: 'video' as const, src: videoPath, label: 'Video demo' }] : []),
    ];
  }, [project.image_path, project.image_paths, project.video_path]);

  const selectedItem = mediaItems[Math.min(selectedIndex, Math.max(0, mediaItems.length - 1))];
  const poster = mediaItems.find((item) => item.kind === 'image')?.src;

  useEffect(() => {
    const video = videoRef.current;
    const frame = frameRef.current;

    if (!video || !frame) return undefined;

    const pauseIfHidden = () => {
      if (document.visibilityState !== 'visible') video.pause();
    };
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (!entry?.isIntersecting) video.pause();
      },
      { threshold: 0.2 },
    );

    observer.observe(frame);
    document.addEventListener('visibilitychange', pauseIfHidden);
    return () => {
      observer.disconnect();

document.removeEventListener('visibilitychange', pauseIfHidden);
      video.pause();
    };
  }, [selectedItem?.kind, selectedItem?.src]);

  if (!selectedItem) {
    return (
      <div className="project-media project-media-empty" aria-label="No project preview available">
        <span>H7 / PROJECT</span>
      </div>
    );
  }

  return (
    <div className="project-media" ref={frameRef}>
      <div className="project-media-stage">
        {failedSources.includes(selectedItem.src) ? (
          <div className="project-media-unavailable" role="status">
            <span>Preview unavailable</span>
            {mediaItems.length > 1 && <small>Select another preview below.</small>}
          </div>
        ) : selectedItem.kind === 'video' ? (
          <video
            ref={videoRef}
            src={selectedItem.src}
            poster={poster}
            controls
            muted
            playsInline
            preload="metadata"
            aria-label={`${project.title ?? 'Project'} video demo`}
            onError={() =>
              setFailedSources((sources) =>
                sources.includes(selectedItem.src) ? sources : [...sources, selectedItem.src],
              )
            }
          />
        ) : (
          <img
            src={selectedItem.src}
            alt={`${project.title ?? 'Project'} preview - ${selectedItem.label}`}
            loading="lazy"
            decoding="async"
            onError={() =>
              setFailedSources((sources) =>
                sources.includes(selectedItem.src) ? sources : [...sources, selectedItem.src],
              )
            }
          />
        )}
      </div>

      {mediaItems.length > 1 && (
        <div className="project-media-controls" aria-label={`${project.title ?? 'Project'} media`}>
          {mediaItems.map((item, index) => (
            <button
              type="button"
              className={index === selectedIndex ? 'active' : ''}
              aria-pressed={index === selectedIndex}
              onClick={() => {
                videoRef.current?.pause();
                setSelectedIndex(index);
              }}
              key={`${item.kind}-${item.src}`}
            >
              {item.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

export default ProjectMedia;
