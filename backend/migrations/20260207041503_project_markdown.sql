-- Add migration script here
CREATE TABLE project_markdown(
  project_id UUID PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
  content_md TEXT NOT NULL
);
