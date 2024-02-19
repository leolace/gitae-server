create table if not exists sessions (
  id uuid DEFAULT uuid_generate_v4() primary key,
  token text not null,
  user_id uuid not null unique,

  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
