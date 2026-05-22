export interface Project {
  title?: string;
  description?: string;
  tech_stack?: string[];
  link?: string;
}

export interface Certificate {
  name?: string;
  issuer?: string;
  year?: string;
  link?: string;
}

export interface SocialLinks {
  github?: string;
  linkedin?: string;
  email?: string;
  website?: string;
}

export interface Profile {
  name?: string;
  role?: string;
  bio?: string;
  skills?: string[];
  projects?: Project[];
  certificates?: Certificate[];
  social_links?: SocialLinks;
}

export interface ChatMessage {
  role: 'user' | 'assistant';
  content: string;
}