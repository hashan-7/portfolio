import { useMemo, useState } from 'react';
import { uploadMedia } from '../../services/api';
import type { FullProfile, Project } from '../../types';

interface ProjectsFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
  requestConfirm: (title: string, message: string, onConfirm: () => void) => void;
}

const emptyProject: Project = {
  title: '',
  short_description: '',
  category: '',
  tech_stack: [],
  github_link: '',
  hf_link: '',
  live_demo_link: '',
  image_path: '',
  image_paths: [],
  video_path: '',
  featured: false,
  public_display: true,
  chatbot_visible: true,
  internal_chatbot_notes: '',
  safe_notes: '',
};

function textToList(value: string): string[] {
  return value
    .split('\n')
    .map((item) => item.trim())
    .filter(Boolean);
}

function ProjectsForm({ profile, onChange, requestConfirm }: ProjectsFormProps) {
  const projects = profile.projects ?? [];
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [uploadMessage, setUploadMessage] = useState('');
  const [uploadError, setUploadError] = useState('');
  const [isUploading, setIsUploading] = useState(false);

  const safeSelectedIndex =
    projects.length === 0 ? -1 : Math.min(selectedIndex, projects.length - 1);

  const selectedProject = safeSelectedIndex >= 0 ? projects[safeSelectedIndex] : null;

  const selectedImages = useMemo(() => {
    if (!selectedProject) {
      return [];
    }

    const imagePaths = selectedProject.image_paths ?? [];

    if (imagePaths.length > 0) {
      return imagePaths.slice(0, 3);
    }

    return selectedProject.image_path ? [selectedProject.image_path] : [];
  }, [selectedProject]);

  const updateProjects = (nextProjects: Project[]) => {
    onChange({
      ...profile,
      projects: nextProjects,
    });
  };

  const updateSelectedProject = (nextProject: Project) => {
    if (safeSelectedIndex < 0) {
      return;
    }

    const nextProjects = [...projects];
    nextProjects[safeSelectedIndex] = nextProject;
    updateProjects(nextProjects);
  };

  const addProject = () => {
    const nextProjects = [...projects, { ...emptyProject }];
    updateProjects(nextProjects);
    setSelectedIndex(nextProjects.length - 1);
    setUploadMessage('');
    setUploadError('');
  };

  const moveSelectedProject = (direction: 'up' | 'down') => {
    if (safeSelectedIndex < 0) {
      return;
    }

    const targetIndex = direction === 'up' ? safeSelectedIndex - 1 : safeSelectedIndex + 1;

    if (targetIndex < 0 || targetIndex >= projects.length) {
      return;
    }

    const nextProjects = [...projects];
    [nextProjects[safeSelectedIndex], nextProjects[targetIndex]] = [
      nextProjects[targetIndex],
      nextProjects[safeSelectedIndex],
    ];

    updateProjects(nextProjects);
    setSelectedIndex(targetIndex);
    setUploadMessage('');
    setUploadError('');
  };

  const deleteSelectedProject = () => {
    if (!selectedProject || safeSelectedIndex < 0) {
      return;
    }

    const projectTitle = selectedProject.title || `Project ${safeSelectedIndex + 1}`;

    requestConfirm(
      'Delete project?',
      `Are you sure you want to delete "${projectTitle}"? This action cannot be undone after saving.`,
      () => {
        const nextProjects = projects.filter((_, index) => index !== safeSelectedIndex);
        updateProjects(nextProjects);
        setSelectedIndex(Math.max(0, safeSelectedIndex - 1));
        setUploadMessage('');
        setUploadError('');
      },
    );
  };

  const handleImageUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    if (!selectedProject) {
      return;
    }

    const files = Array.from(event.target.files ?? []);
    event.target.value = '';

    if (files.length === 0) {
      return;
    }

    const remainingSlots = 3 - selectedImages.length;

    if (remainingSlots <= 0) {
      setUploadError('Maximum 3 images are allowed for one project.');
      return;
    }

    if (files.length > remainingSlots) {
      setUploadError(`You can upload only ${remainingSlots} more image(s) for this project.`);
      return;
    }

    setIsUploading(true);
    setUploadMessage('');
    setUploadError('');

    try {
      const uploadedPaths: string[] = [];

      for (const file of files) {
        const paths = await uploadMedia(file);
        uploadedPaths.push(...paths);
      }

      const imageOnlyPaths = uploadedPaths.filter((path) =>
        /\.(png|jpg|jpeg|webp|gif)$/i.test(path),
      );

      if (imageOnlyPaths.length === 0) {
        setUploadError('No image path was returned from the upload.');
        return;
      }

      const nextImages = [...selectedImages, ...imageOnlyPaths].slice(0, 3);

      updateSelectedProject({
        ...selectedProject,
        image_paths: nextImages,
        image_path: nextImages[0] ?? '',
      });

      setUploadMessage(`Uploaded ${imageOnlyPaths.length} image(s).`);
    } catch (error) {
      setUploadError(error instanceof Error ? error.message : 'Failed to upload image.');
    } finally {
      setIsUploading(false);
    }
  };

  const removeImage = (imageIndex: number) => {
    if (!selectedProject) {
      return;
    }

    requestConfirm(
      'Remove image?',
      'Are you sure you want to remove this image path from the selected project?',
      () => {
        const nextImages = selectedImages.filter((_, index) => index !== imageIndex);

        updateSelectedProject({
          ...selectedProject,
          image_paths: nextImages,
          image_path: nextImages[0] ?? '',
        });
      },
    );
  };

  return (
    <section className="admin-section clean">
      <div className="admin-form-toolbar">
        <div>
          <h2>Project Editor</h2>
          <p className="admin-muted">
            Select one project, update it, reorder it, add a new project, or delete the selected project.
          </p>
        </div>

        <div className="admin-actions">
          <button type="button" onClick={addProject}>
            Add Project
          </button>
          <button
            className="admin-secondary-button"
            type="button"
            onClick={() => moveSelectedProject('up')}
            disabled={safeSelectedIndex <= 0}
          >
            Move Up
          </button>
          <button
            className="admin-secondary-button"
            type="button"
            onClick={() => moveSelectedProject('down')}
            disabled={safeSelectedIndex < 0 || safeSelectedIndex >= projects.length - 1}
          >
            Move Down
          </button>
          <button
            className="admin-danger-button"
            type="button"
            onClick={deleteSelectedProject}
            disabled={!selectedProject}
          >
            Delete Selected
          </button>
        </div>
      </div>

      {projects.length === 0 && (
        <div className="admin-empty-box">
          <p>No projects yet. Click Add Project to create one.</p>
        </div>
      )}

      {projects.length > 0 && (
        <div className="admin-form">
          <label>
            Select Project
            <select
              value={safeSelectedIndex}
              onChange={(event) => {
                setSelectedIndex(Number(event.target.value));
                setUploadMessage('');
                setUploadError('');
              }}
            >
              {projects.map((project, index) => (
                <option key={`${project.title || 'project'}-${index}`} value={index}>
                  {index + 1}. {project.title || `Untitled Project ${index + 1}`}
                </option>
              ))}
            </select>
          </label>

          {selectedProject && (
            <>
              <div className="admin-form-grid">
                <label>
                  Title
                  <input
                    type="text"
                    value={selectedProject.title ?? ''}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        title: event.target.value,
                      })
                    }
                    placeholder="Project title"
                  />
                </label>

                <label>
                  Category
                  <input
                    type="text"
                    value={selectedProject.category ?? ''}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        category: event.target.value,
                      })
                    }
                    placeholder="Project category"
                  />
                </label>

                <label className="wide">
                  Short Description
                  <textarea
                    className="admin-json-editor compact"
                    value={selectedProject.short_description ?? ''}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        short_description: event.target.value,
                      })
                    }
                    placeholder="Short public project description"
                  />
                </label>

                <label>
                  GitHub Link
                  <input
                    type="url"
                    value={selectedProject.github_link ?? ''}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        github_link: event.target.value,
                      })
                    }
                    placeholder="https://github.com/..."
                  />
                </label>

                <label>
                  Hugging Face Link
                  <input
                    type="url"
                    value={selectedProject.hf_link ?? ''}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        hf_link: event.target.value,
                      })
                    }
                    placeholder="https://huggingface.co/..."
                  />
                </label>

                <label>
                  Live Demo Link
                  <input
                    type="url"
                    value={selectedProject.live_demo_link ?? ''}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        live_demo_link: event.target.value,
                      })
                    }
                    placeholder="https://..."
                  />
                </label>

                <label>
                  Video Path
                  <input
                    type="text"
                    value={selectedProject.video_path ?? ''}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        video_path: event.target.value,
                      })
                    }
                    placeholder="/media/projects/videos/demo.mp4"
                  />
                </label>

                <label className="wide">
                  Tech Stack — one item per line
                  <textarea
                    className="admin-json-editor compact"
                    value={(selectedProject.tech_stack ?? []).join('\n')}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        tech_stack: textToList(event.target.value),
                      })
                    }
                    placeholder="Java&#10;Spring Boot&#10;MySQL"
                  />
                </label>

                <label className="wide">
                  Internal Chatbot Notes
                  <textarea
                    className="admin-json-editor compact tall"
                    value={selectedProject.internal_chatbot_notes ?? ''}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        internal_chatbot_notes: event.target.value,
                      })
                    }
                    placeholder="Detailed verified notes for chatbot answers only"
                  />
                </label>

                <label className="wide">
                  Safe Notes
                  <textarea
                    className="admin-json-editor compact"
                    value={selectedProject.safe_notes ?? ''}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        safe_notes: event.target.value,
                      })
                    }
                    placeholder="Private admin safety notes"
                  />
                </label>
              </div>

              <div className="admin-switch-grid">
                <label className="admin-checkbox-row">
                  <input
                    type="checkbox"
                    checked={selectedProject.public_display}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        public_display: event.target.checked,
                      })
                    }
                  />
                  Show on public portfolio
                </label>

                <label className="admin-checkbox-row">
                  <input
                    type="checkbox"
                    checked={selectedProject.chatbot_visible ?? true}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        chatbot_visible: event.target.checked,
                      })
                    }
                  />
                  Visible to chatbot
                </label>

                <label className="admin-checkbox-row">
                  <input
                    type="checkbox"
                    checked={selectedProject.featured}
                    onChange={(event) =>
                      updateSelectedProject({
                        ...selectedProject,
                        featured: event.target.checked,
                      })
                    }
                  />
                  Featured project
                </label>
              </div>

              <div className="project-image-manager">
                <div className="admin-form-toolbar">
                  <div>
                    <h3>Project Images</h3>
                    <p className="admin-muted">
                      Upload up to 3 images. The first image is also saved as image_path.
                    </p>
                  </div>

                  <label className="admin-file-button">
                    {isUploading ? 'Uploading...' : 'Upload Image'}
                    <input
                      type="file"
                      accept="image/png,image/jpeg,image/webp,image/gif"
                      multiple
                      onChange={handleImageUpload}
                      disabled={isUploading || selectedImages.length >= 3}
                    />
                  </label>
                </div>

                {selectedImages.length === 0 && (
                  <div className="admin-empty-box">
                    <p>No images added for this project.</p>
                  </div>
                )}

                {selectedImages.length > 0 && (
                  <div className="project-image-grid">
                    {selectedImages.map((imagePath, index) => (
                      <div className="project-image-card" key={`${imagePath}-${index}`}>
                        <img src={imagePath} alt={`Project image ${index + 1}`} />
                        <div>
                          <span>Image {index + 1}</span>
                          <code>{imagePath}</code>
                        </div>
                        <button
                          className="admin-danger-button small"
                          type="button"
                          onClick={() => removeImage(index)}
                        >
                          Remove
                        </button>
                      </div>
                    ))}
                  </div>
                )}

                {uploadMessage && <p className="admin-success">{uploadMessage}</p>}
                {uploadError && <p className="admin-error">{uploadError}</p>}
              </div>
            </>
          )}
        </div>
      )}
    </section>
  );
}

export default ProjectsForm;