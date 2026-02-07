-- Add migration script here
CREATE TABLE projects(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  slug TEXT UNIQUE NOT NULL,
  title TEXT NOT NULL,
  short_description TEXT,
  cover_image_url TEXT,
  repo_url TEXT,
  live_url TEXT,
  tech_stack TEXT[],
  is_published BOOLEAN DEFAULT true,
  created_at TIMESTAMP DEFAULT now(),
  updated_at TIMESTAMP DEFAULT now()
);
