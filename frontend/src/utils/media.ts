const videoExtensions = ['.mp4', '.webm', '.mov'];
const imageExtensions = ['.png', '.jpg', '.jpeg', '.webp', '.gif'];

export function isVideoPath(path?: string): boolean {
  if (!path) {
    return false;
  }

  const lowerPath = path.toLowerCase();
  return videoExtensions.some((extension) => lowerPath.endsWith(extension));
}

export function isImagePath(path?: string): boolean {
  if (!path) {
    return false;
  }

  const lowerPath = path.toLowerCase();
  return imageExtensions.some((extension) => lowerPath.endsWith(extension));
}

export function mediaTypeLabel(path?: string): string {
  if (isVideoPath(path)) {
    return 'Video';
  }

  if (isImagePath(path)) {
    return 'Image';
  }

  return 'Media';
}