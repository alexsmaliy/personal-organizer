BEGIN TRANSACTION;
   INSERT INTO user VALUES
      ('user-123', 'admin', 'admin@example.com', '$2a$12$z5GYBePHK0ZSjXBLPex78u', '$2a$12$z5GYBePHK0ZSjXBLPex78uYTTdbuyffkysSN5XERQmQndkBxCQpAi'),  -- siberian
      ('user-456', 'meow', 'meow@example.com', '$2a$12$fsfL0t8TFeRHa.dG6TCH.O', '$2a$12$fsfL0t8TFeRHa.dG6TCH.Ompx6PLlQ.1tLnS542qqvGsOj2kGRq.O')    -- husky
   ;

   INSERT INTO bookmark (id, user_id, title, url, about) VALUES
      ('bookmark-1', 'user-123', 'Reddit', 'reddit.com', 'A social aggregation website for news, comedy, and web content.' ),
      ('bookmark-2', 'user-123', 'CNN Lite', 'cnn.com/lite', 'A minimalist version of CNN.'),
      ('bookmark-3', 'user-123', 'Fly.io', 'fly.io', 'An edge platform for apps.'),
      ('bookmark-4', 'user-123', 'Solid', 'solidjs.com', 'A compilation-based JSX component framework.'),
      ('bookmark-5', 'user-123', 'Remix', 'remix.run', 'An SSR framework for React.'),
      ('bookmark-6', 'user-123', 'Astro', 'astro.build', 'An MPA framework with SSR/SSG and an agnostic approach to components.'),
      ('bookmark-7', 'user-123', 'Render', 'render.com', 'Heroku competitor with managed PostgreSQL, Redis, queues, etc.'),
      ('bookmark-8', 'user-123', 'Fresh', 'fresh.deno.com', 'A Preact SSR framework for Deno emphasizing minimal toolchain.'),
      ('bookmark-9', 'user-123', 'qwik', 'qwik.builder.io', 'A component framework with SSR focused on minimal runtime and fast initial load.')
   ;

   INSERT INTO tag VALUES
      ('tag-1', 'news'), ('tag-2', 'social'), ('tag-3', 'design'), ('tag-4', 'minimalism'),
      ('tag-5', 'sample'), ('tag-6', 'edge'), ('tag-7', 'hosting'), ('tag-8', 'framework'),
      ('tag-9', 'solid'), ('tag-10', 'ssr'), ('tag-11', 'spa'), ('tag-12', 'jsx'),
      ('tag-13', 'mpa'), ('tag-14', 'ssg'), ('tag-15', 'deno'), ('tag-16', 'preact'),
      ('tag-17', 'resumable'), ('tag-18', 'react')
   ;

   INSERT INTO bookmark_tag_link VALUES
      ('bookmark-1', 'tag-1'), ('bookmark-1', 'tag-2'),
      ('bookmark-2', 'tag-3'), ('bookmark-2', 'tag-4'), ('bookmark-2', 'tag-5'),
      ('bookmark-3', 'tag-6'), ('bookmark-3', 'tag-7'),
      ('bookmark-4', 'tag-8'), ('bookmark-4', 'tag-9'), ('bookmark-4', 'tag-10'), ('bookmark-4', 'tag-14'), ('bookmark-4', 'tag-12'),
      ('bookmark-5', 'tag-8'), ('bookmark-5', 'tag-18'), ('bookmark-5', 'tag-10'), ('bookmark-5', 'tag-11'), ('bookmark-5', 'tag-12'),
      ('bookmark-6', 'tag-8'), ('bookmark-6', 'tag-13'), ('bookmark-6', 'tag-10'), ('bookmark-6', 'tag-14'),
      ('bookmark-7', 'tag-7'),
      ('bookmark-8', 'tag-8'), ('bookmark-8', 'tag-15'), ('bookmark-8', 'tag-10'), ('bookmark-8', 'tag-11'), ('bookmark-8', 'tag-16'), ('bookmark-8', 'tag-12'),
      ('bookmark-9', 'tag-8'), ('bookmark-9', 'tag-12'), ('bookmark-9', 'tag-10'), ('bookmark-9', 'tag-17')
   ;

   INSERT INTO settings (user_id, theme, home) VALUES
      ('user-123', false, 'all'),
      ('user-456', false, 'inbox')
   ;
COMMIT;
