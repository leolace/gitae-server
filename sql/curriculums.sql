create table if not exists curriculums (
  id uuid default uuid_generate_v4() primary key,
  user_id uuid,
  github_user_id varchar(12) not null,
  name varchar(50) not null,
  job_title varchar(50) not null, 
  about text not null default '',
  skills varchar(20)[],

  foreign key (user_id) references users(id) on delete cascade
);

create table if not exists user_projects (
  id uuid default uuid_generate_v4() primary key,
  curriculum_id uuid not null,
  name varchar(50) not null,
  url text not null,
  repo_url text not null,

  foreign key (curriculum_id) references curriculums(id) on delete cascade
);

create table if not exists user_experiences (
 id uuid default uuid_generate_v4() primary key,
  curriculum_id uuid not null,
  title varchar(100) not null,
  sub_title varchar(100) not null,
  description text default '',

  foreign key (curriculum_id) references curriculums(id) on delete cascade
);
