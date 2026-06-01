const videoExtensions = ['.mp4', '.webm', '.mov'];
const imageExtensions = ['.png', '.jpg', '.jpeg', '.webp', '.gif'];

const apiBaseUrl = (import.meta.env.VITE_API_BASE_URL ?? '').trim().replace(/\/+$/, '');

function isAbsoluteMediaUrl(path: string): boolean {
  return (
    path.startsWith('http://') ||
    path.startsWith('https://') ||
    path.startsWith('data:') ||
    path.startsWith('blob:')
  );
}

export function isVideoPath(path?: string): boolean {
  if (!path) return false;
  const lowerPath = path.toLowerCase();
  return videoExtensions.some((extension) => lowerPath.endsWith(extension));
}

export function isImagePath(path?: string): boolean {
  if (!path) return false;
  const lowerPath = path.toLowerCase();
  return imageExtensions.some((extension) => lowerPath.endsWith(extension));
}

export function mediaTypeLabel(path?: string): string {
  if (isVideoPath(path)) return 'Video';
  if (isImagePath(path)) return 'Image';
  return 'Media';
}

export function normalizeMediaPath(path?: string): string | undefined {
  const trimmed = path?.trim();
  if (!trimmed) return undefined;

  if (isAbsoluteMediaUrl(trimmed)) {
    return trimmed;
  }

  const normalizedPath = trimmed.startsWith('/') ? trimmed : `/${trimmed}`;

  if (apiBaseUrl) {
    return `${apiBaseUrl}${normalizedPath}`;
  }

  return normalizedPath;
}

export function normalizeMediaPaths(paths?: string[]): string[] {
  return (paths ?? [])
    .map(normalizeMediaPath)
    .filter((path): path is string => Boolean(path));
}