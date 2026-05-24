import { useEffect, useMemo, useState } from 'react';
import { getAdminProfile, updateAdminProfile } from '../../services/api';
import type { FullProfile } from '../../types';
import ConfirmDialog from '../common/ConfirmDialog';
import BasicProfileForm from './BasicProfileForm';
import CertificatesForm from './CertificatesForm';
import ContactForm from './ContactForm';
import EducationForm from './EducationForm';
import ProjectsForm from './ProjectsForm';
import SkillsForm from './SkillsForm';

type AdminTab =
  | 'profile'
  | 'links'
  | 'skills'
  | 'projects'
  | 'certificates'
  | 'education'
  | 'json';

interface ConfirmState {
  open: boolean;
  title: string;
  message: string;
  onConfirm: () => void;
}

const tabMeta: Record<AdminTab, { title: string; description: string }> = {
  profile: {
    title: 'Basic Profile',
    description: 'Edit the public identity and profile summary.',
  },
  links: {
    title: 'Contact & Links',
    description: 'Manage public contact links and profile buttons.',
  },
  skills: {
    title: 'Skills & Focus',
    description: 'Edit skills and focus areas shown in the portfolio.',
  },
  projects: {
    title: 'Projects',
    description: 'Manage project cards, links, images, and chatbot details.',
  },
  certificates: {
    title: 'Certificates',
    description: 'Manage verified certificates and proof links.',
  },
  education: {
    title: 'Education',
    description: 'Manage school and NIBM education entries.',
  },
  json: {
    title: 'Advanced JSON',
    description: 'Advanced full-profile JSON editor.',
  },
};

const tabs: { id: AdminTab; label: string; number: string }[] = [
  { id: 'profile', label: 'Basic Profile', number: '01' },
  { id: 'links', label: 'Contact & Links', number: '02' },
  { id: 'skills', label: 'Skills & Focus', number: '03' },
  { id: 'projects', label: 'Projects', number: '04' },
  { id: 'certificates', label: 'Certificates', number: '05' },
  { id: 'education', label: 'Education', number: '06' },
  { id: 'json', label: 'Advanced JSON', number: '07' },
];

function AdminDashboard() {
  const [profile, setProfile] = useState<FullProfile | null>(null);
  const [jsonText, setJsonText] = useState('');
  const [activeTab, setActiveTab] = useState<AdminTab>('profile');
  const [statusMessage, setStatusMessage] = useState('');
  const [errorMessage, setErrorMessage] = useState('');
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

  const [confirmState, setConfirmState] = useState<ConfirmState>({
    open: false,
    title: '',
    message: '',
    onConfirm: () => {},
  });

  const projectCount = useMemo(() => profile?.projects?.length ?? 0, [profile]);
  const certificateCount = useMemo(() => profile?.certificates?.length ?? 0, [profile]);
  const skillCount = useMemo(() => profile?.skills?.length ?? 0, [profile]);

  const setProfileAndJson = (nextProfile: FullProfile) => {
    setProfile(nextProfile);
    setJsonText(JSON.stringify(nextProfile, null, 2));
  };

  const loadProfile = async () => {
    setIsLoading(true);
    setErrorMessage('');
    setStatusMessage('');

    try {
      const data = await getAdminProfile();
      setProfileAndJson(data);
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : 'Failed to load admin profile.');
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadProfile();
  }, []);

  const requestConfirm = (title: string, message: string, onConfirm: () => void) => {
    setConfirmState({
      open: true,
      title,
      message,
      onConfirm,
    });
  };

  const closeConfirm = () => {
    setConfirmState((previous) => ({
      ...previous,
      open: false,
    }));
  };

  const handleConfirmedAction = () => {
    confirmState.onConfirm();
    closeConfirm();
  };

  const handleSaveCurrentTab = async () => {
    if (!profile) {
      setErrorMessage('Profile data is not loaded.');
      return;
    }

    setIsSaving(true);
    setErrorMessage('');
    setStatusMessage('');

    try {
      await updateAdminProfile(profile);
      setStatusMessage(`${tabMeta[activeTab].title} saved successfully.`);
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : 'Failed to save profile.');
    } finally {
      setIsSaving(false);
    }
  };

  const handleApplyJson = () => {
    setErrorMessage('');
    setStatusMessage('');

    try {
      const parsedProfile = JSON.parse(jsonText) as FullProfile;
      setProfileAndJson(parsedProfile);
      setStatusMessage('JSON applied to form data. Click Save This Tab to store it.');
    } catch {
      setErrorMessage('Invalid JSON. Please fix it before applying.');
    }
  };

  const handleSaveJsonDirectly = async () => {
    setIsSaving(true);
    setErrorMessage('');
    setStatusMessage('');

    try {
      const parsedProfile = JSON.parse(jsonText) as FullProfile;
      await updateAdminProfile(parsedProfile);
      setProfileAndJson(parsedProfile);
      setStatusMessage('JSON saved successfully.');
    } catch (error) {
      setErrorMessage(error instanceof Error ? error.message : 'Failed to save JSON.');
    } finally {
      setIsSaving(false);
    }
  };

  const handleLogout = () => {
    localStorage.removeItem('admin_token');
    window.location.replace('/h7-admin');
  };

  const renderActiveTab = () => {
    if (!profile) {
      return null;
    }

    if (activeTab === 'profile') {
      return <BasicProfileForm profile={profile} onChange={setProfileAndJson} />;
    }

    if (activeTab === 'links') {
      return <ContactForm profile={profile} onChange={setProfileAndJson} />;
    }

    if (activeTab === 'skills') {
      return <SkillsForm profile={profile} onChange={setProfileAndJson} />;
    }

    if (activeTab === 'projects') {
      return (
        <ProjectsForm
          profile={profile}
          onChange={setProfileAndJson}
          requestConfirm={requestConfirm}
        />
      );
    }

    if (activeTab === 'certificates') {
      return (
        <CertificatesForm
          profile={profile}
          onChange={setProfileAndJson}
          requestConfirm={requestConfirm}
        />
      );
    }

    if (activeTab === 'education') {
      return (
        <EducationForm
          profile={profile}
          onChange={setProfileAndJson}
          requestConfirm={requestConfirm}
        />
      );
    }

    return (
      <section className="admin-section clean">
        <h2>Full Profile JSON</h2>
        <p className="admin-muted">
          Advanced editor. Any valid change here will update form state automatically after applying.
        </p>

        <textarea
          className="admin-json-editor large"
          value={jsonText}
          onChange={(event) => setJsonText(event.target.value)}
          spellCheck={false}
        />

        <div className="admin-actions">
          <button className="admin-secondary-button" type="button" onClick={handleApplyJson}>
            Apply JSON to Forms
          </button>
          <button type="button" onClick={handleSaveJsonDirectly} disabled={isSaving}>
            {isSaving ? 'Saving JSON...' : 'Save JSON Directly'}
          </button>
        </div>
      </section>
    );
  };

  return (
    <main className="admin-shell">
      <aside className="admin-sidebar">
        <div className="admin-brand">
          <div className="admin-brand-logo">H7</div>
          <div>
            <h1>H7 Admin</h1>
            <p>Portfolio Manager</p>
          </div>
        </div>

        <p className="admin-nav-label">Content</p>

        <nav className="admin-nav-list">
          {tabs.map((tab) => (
            <button
              className={`admin-nav-item ${activeTab === tab.id ? 'active' : ''}`}
              type="button"
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
            >
              <span>{tab.number}</span>
              {tab.label}
            </button>
          ))}
        </nav>

        <p className="admin-nav-label">Session</p>

        <div className="admin-nav-list">
          <a className="admin-nav-item" href="/" target="_blank" rel="noreferrer">
            <span>↗</span>
            View Public Site
          </a>
          <button className="admin-nav-item" type="button" onClick={handleLogout}>
            <span>⎋</span>
            Logout
          </button>
        </div>

        <div className="admin-sidebar-note">
          <strong>Private route</strong>
          <span>This admin panel is hidden from the public portfolio UI.</span>
        </div>
      </aside>

      <section className="admin-main">
        <header className="admin-topbar">
          <div>
            <p className="eyebrow">Admin Dashboard</p>
            <h1>Portfolio Data Manager</h1>
            <p>
              Manage public profile content, internal chatbot context, project images, certificates,
              education, and safe portfolio data.
            </p>
          </div>

          <div className="admin-actions top">
            <button className="admin-secondary-button" type="button" onClick={loadProfile}>
              Reload
            </button>
            <button type="button" onClick={handleSaveCurrentTab} disabled={isSaving || !profile}>
              {isSaving ? 'Saving...' : 'Save This Tab'}
            </button>
          </div>
        </header>

        <div className="admin-stats-grid">
          <div>
            <span>Projects</span>
            <strong>{projectCount}</strong>
          </div>
          <div>
            <span>Certificates</span>
            <strong>{certificateCount}</strong>
          </div>
          <div>
            <span>Skills</span>
            <strong>{skillCount}</strong>
          </div>
          <div>
            <span>Public Name</span>
            <strong>{profile?.display_name ?? profile?.name ?? 'Not set'}</strong>
          </div>
        </div>

        <div className="admin-content-layout">
          <section className="admin-panel">
            <div className="admin-panel-header">
              <div>
                <h2>{tabMeta[activeTab].title}</h2>
                <p>{tabMeta[activeTab].description}</p>
              </div>
              <span className="admin-badge">Auto JSON Sync</span>
            </div>

            <div className="admin-panel-body">
              {isLoading && <p className="admin-muted">Loading admin data...</p>}
              {!isLoading && profile && renderActiveTab()}
              {!isLoading && !profile && (
                <p className="admin-error">Profile data could not be loaded.</p>
              )}
            </div>

            {activeTab !== 'json' && (
              <div className="admin-panel-footer">
                <button
                  className="admin-secondary-button"
                  type="button"
                  onClick={loadProfile}
                  disabled={isSaving}
                >
                  Reload
                </button>
                <button type="button" onClick={handleSaveCurrentTab} disabled={isSaving || !profile}>
                  {isSaving ? 'Saving...' : `Save ${tabMeta[activeTab].title}`}
                </button>
              </div>
            )}
          </section>

          <aside className="admin-side-panel">
            <section className="admin-panel small">
              <div className="admin-panel-header">
                <div>
                  <h2>System Status</h2>
                  <p>Current admin state</p>
                </div>
              </div>

              <div className="admin-panel-body">
                <div className="admin-status-list">
                  <div>
                    <span>Admin Token</span>
                    <strong>Active</strong>
                  </div>
                  <div>
                    <span>Profile JSON</span>
                    <strong>{profile ? 'Loaded' : 'Missing'}</strong>
                  </div>
                  <div>
                    <span>JSON Sync</span>
                    <strong>Auto</strong>
                  </div>
                  <div>
                    <span>AI Fallback</span>
                    <strong>Optional</strong>
                  </div>
                </div>
              </div>
            </section>

            <section className="admin-panel small">
              <div className="admin-panel-header">
                <div>
                  <h2>Quick Notes</h2>
                  <p>Admin reminders</p>
                </div>
              </div>

              <div className="admin-panel-body">
                <p className="admin-muted">
                  Public UI receives only filtered data. Internal chatbot notes are used only for
                  chatbot context.
                </p>
                <p className="admin-muted">
                  Project images can be uploaded from the project editor. Maximum 3 images per
                  project.
                </p>
                <p className="admin-muted">
                  Do not store secrets in JSON. Use HF Space secrets for keys and admin credentials.
                </p>
              </div>
            </section>
          </aside>
        </div>

        {statusMessage && <p className="admin-success floating">{statusMessage}</p>}
        {errorMessage && <p className="admin-error floating">{errorMessage}</p>}
      </section>

      <ConfirmDialog
        open={confirmState.open}
        title={confirmState.title}
        message={confirmState.message}
        onCancel={closeConfirm}
        onConfirm={handleConfirmedAction}
      />
    </main>
  );
}

export default AdminDashboard;