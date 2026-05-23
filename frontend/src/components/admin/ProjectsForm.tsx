import { useState } from 'react';
import type { FullProfile, Project } from '../../types';

interface ProjectsFormProps {
  profile: FullProfile;
  onChange: (profile: FullProfile) => void;
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
  video_path: '',
  featured: false,
  public_display: true,
  internal_chatbot_notes: '',
  safe_notes: '',
};

function textToList(value: string): string[] {
  return value
    .split('\n')
    .map((item) => item.trim())
    .filter(Boolean);
}

function ProjectsForm({ profile, onChange }: ProjectsFormProps) {
  const projects = profile.projects ?? [];
  const [selectedIndex, setSelectedIndex] = useState(0);
  const selectedProject = projects[selectedIndex];

  const updateProjects = (nextProjects: Project[]) => {
    onChange({
      ...profile,
      projects: nextProjects,
    });
  };

  const updateSelectedProject = (project: Project) => {
    const nextProjects = [...projects];
    nextProjects[selectedIndex] = project;
    updateProjects(nextProjects);
  };

  const addProject = () => {
    updateProjects([...projects, { ...emptyProject }]);
    setSelectedIndex(projects.length);
  };

  const removeProject = () => {
    if (!selectedProject) {
      return;
    }

    const nextProjects = projects.filter((_, index) => index !== selectedIndex);
    updateProjects(nextProjects);
    setSelectedIndex(Math.max(0, selectedIndex - 1));
  };

  return (
    <section className="admin-section">
      <h2>Projects</h2>
      <p className="admin-muted">Manage public project cards and internal chatbot notes.</p>

      <div className="admin-actions">
        <button type="button" onClick={addProject}>
          Add Project
        </button>
        <button
          className="admin-secondary-button"
          type="button"
          onClick={removeProject}
          disabled={!selectedProject}
        >
          Remove Selected
        </button>
      </div>

      {projects.length > 0 && (
        <div className="admin-form">
          <label>
            Select Project
            <select
              value={selectedIndex}
              onChange={(event) => setSelectedIndex(Number(event.target.value))}
            >
              {projects.map((project, index) => (
                <option key={`${project.title ?? 'project'}-${index}`} value={index}>
                  {project.title || `Project ${index + 1}`}
                </option>
              ))}
            </select>
          </label>

          {selectedProject && (
            <>
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
                />
              </label>

              <label>
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
                />
              </label>

              <label>
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
                />
              </label>

              <label>
                Image Path
                <input
                  type="text"
                  value={selectedProject.image_path ?? ''}
                  onChange={(event) =>
                    updateSelectedProject({
                      ...selectedProject,
                      image_path: event.target.value,
                    })
                  }
                  placeholder="/media/projects/images/example.png"
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
                  placeholder="/media/projects/videos/example.mp4"
                />
              </label>

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

              <label>
                Internal Chatbot Notes
                <textarea
                  className="admin-json-editor compact"
                  value={selectedProject.internal_chatbot_notes ?? ''}
                  onChange={(event) =>
                    updateSelectedProject({
                      ...selectedProject,
                      internal_chatbot_notes: event.target.value,
                    })
                  }
                />
              </label>

              <label>
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
                />
              </label>
            </>
          )}
        </div>
      )}
    </section>
  );
}

export default ProjectsForm;