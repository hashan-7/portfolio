export interface Certificate {
  name?: string;
  issuer?: string;
  year?: string;
  date?: string;
  link?: string;
}

export interface SocialLinks {
  github?: string;
  linkedin?: string;
  email?: string;
  phone?: string;
  website?: string;
  huggingface?: string;
  kaggle?: string;
  resume?: string;
  instagram?: string;
}

export interface Education {
  institution?: string;
  degree?: string;
  duration?: string;
  year?: string;
  grade?: string;
  status?: string;
  link?: string;
}

export interface PublicProject {
  title?: string;
  short_description?: string;
  category?: string;
  tech_stack: string[];
  github_link?: string;
  hf_link?: string;
  live_demo_link?: string;
  image_path?: string;
  image_paths?: string[];
  video_path?: string;
  featured: boolean;
}

export interface PublicProfile {
  name?: string;
  display_name?: string;
  role?: string;
  tagline?: string;
  location?: string;
  bio?: string;
  profile_image_path?: string;
  skills: string[];
  focus_areas: string[];
  projects: PublicProject[];
  certificates: Certificate[];
  education: Education[];
  social_links?: SocialLinks;
}

export interface Project extends PublicProject {
  public_display: boolean;
  chatbot_visible?: boolean;
  internal_chatbot_notes?: string;
  safe_notes?: string;
}

export interface FullProfile {
  name?: string;
  display_name?: string;
  role?: string;
  tagline?: string;
  location?: string;
  bio?: string;
  profile_image_path?: string;
  skills: string[];
  focus_areas: string[];
  projects: Project[];
  certificates: Certificate[];
  education: Education[];
  social_links?: SocialLinks;
  chatbot_rules?: string;
}

export interface ChatMessage {
  role: 'user' | 'assistant';
  content: string;
}
