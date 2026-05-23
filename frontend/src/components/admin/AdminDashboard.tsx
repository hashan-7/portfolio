import { useEffect, useMemo, useState } from 'react';
import { getAdminProfile, updateAdminProfile } from '../../services/api';
import type { FullProfile } from '../../types';
import BasicProfileForm from './BasicProfileForm';
import CertificatesForm from './CertificatesForm';
import ContactForm from './ContactForm';
import EducationForm from './EducationForm';
import MediaUpload from './MediaUpload';
import ProjectsForm from './ProjectsForm';
import SkillsForm from './SkillsForm';

function AdminDashboard() {
  const [profile, setProfile] = useState<FullProfile | null>(null);
  const [jsonText, setJsonText] = useState('');
  const [statusMessage, setStatusMessage] = useState('');
  const [errorMessage, setErrorMessage] = useState('');
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

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

  const handleLogout = () => {
    localStorage.removeItem('admin_token');
    window.location.href = '/h7-admin';
  };

  const handleSaveProfile = async () => {
    if (!profile) {
      setErrorMessage('Profile data is not loaded.');
      return;
    }

    setIsSaving(true);
    setErrorMessage('');
    setStatusMessage('');

    try {
      await updateAdminProfile(profile);
      setStatusMessage('Profile saved successfully.');
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
      setStatusMessage('JSON applied to form data. Click Save Profile to store it.');
    } catch {
      setErrorMessage('Invalid JSON. Please fix the JSON before applying.');
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

  return (
    <main className="admin-page">
      <section className="admin-card">
        <div className="admin-topbar">
          <div>
            <p className="eyebrow">Admin Dashboard</p>
            <h1>Portfolio Data Manager</h1>
            <p className="admin-muted">
              Manage public portfolio data, chatbot context, and project media.
            </p>
          </div>

          <button className="admin-secondary-button" type="button" onClick={handleLogout}>
            Logout
          </button>
        </div>

        {isLoading && <p className="admin-muted">Loading admin data...</p>}

        {!isLoading && profile && (
          <>
            <div className="admin-stats">
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
                <span>Public name</span>
                <strong>{profile.display_name ?? profile.name ?? 'Not set'}</strong>
              </div>
            </div>

            <MediaUpload />

            <BasicProfileForm profile={profile} onChange={setProfileAndJson} />

            <ContactForm profile={profile} onChange={setProfileAndJson} />

            <SkillsForm profile={profile} onChange={setProfileAndJson} />

            <ProjectsForm profile={profile} onChange={setProfileAndJson} />

            <CertificatesForm profile={profile} onChange={setProfileAndJson} />

            <EducationForm profile={profile} onChange={setProfileAndJson} />

            <section className="admin-section">
              <h2>Chatbot Rules</h2>
              <p className="admin-muted">
                Internal notes for the chatbot. These are not sent to the public profile endpoint.
              </p>

              <textarea
                className="admin-json-editor compact"
                value={profile.chatbot_rules ?? ''}
                onChange={(event) =>
                  setProfileAndJson({
                    ...profile,
                    chatbot_rules: event.target.value || undefined,
                  })
                }
                spellCheck={false}
                placeholder="Chatbot rules and safe response notes..."
              />
            </section>

            <section className="admin-section">
              <h2>Full JSON Editor</h2>
              <p className="admin-muted">
                Advanced editor for the complete protected profile JSON. Use carefully.
              </p>

              <textarea
                className="admin-json-editor"
                value={jsonText}
                onChange={(event) => setJsonText(event.target.value)}
                spellCheck={false}
              />

              <div className="admin-actions">
                <button type="button" onClick={handleApplyJson}>
                  Apply JSON to Forms
                </button>

                <button type="button" onClick={handleSaveJsonDirectly} disabled={isSaving}>
                  {isSaving ? 'Saving JSON...' : 'Save JSON Directly'}
                </button>

                <button className="admin-secondary-button" type="button" onClick={loadProfile}>
                  Reload
                </button>
              </div>
            </section>

            <div className="admin-actions">
              <button type="button" onClick={handleSaveProfile} disabled={isSaving}>
                {isSaving ? 'Saving...' : 'Save Profile'}
              </button>
            </div>
          </>
        )}

        {statusMessage && <p className="admin-success">{statusMessage}</p>}
        {errorMessage && <p className="admin-error">{errorMessage}</p>}
      </section>
    </main>
  );
}

export default AdminDashboard;