const videoExtensions = ['.mp4', '.webm', '.mov'];
const imageExtensions = ['.png', '.jpg', '.jpeg', '.webp', '.gif'];

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
  if (
    trimmed.startsWith('/') ||
    trimmed.startsWith('http://') ||
    trimmed.startsWith('https://') ||
    trimmed.startsWith('data:')
  ) {
    return trimmed;
  }
  return `/${trimmed}`;
}

export function normalizeMediaPaths(paths?: string[]): string[] {
  return (paths ?? [])
    .map(normalizeMediaPath)
    .filter((path): path is string => Boolean(path));
}
