CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(64) NOT NULL UNIQUE,
    password VARCHAR(64) NOT NULL,
    deleted_at TIMESTAMPTZ DEFAULT NULL,
    token TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    priority VARCHAR(4) DEFAULT NULL,
    title VARCHAR(255) NOT NULL,
    completed_at TIMESTAMPTZ DEFAULT NULL,
    description TEXT DEFAULT NULL,
    deleted_at TIMESTAMPTZ DEFAULT NULL,
    user_id INTEGER DEFAULT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT INTO users (username, password) 
VALUES ('deleteuser', '$2@12$x3jergkjergjflkgjldfkjgdf.ixJAX9Cj');

INSERT INTO tasks (title, deleted_at, user_id) 
VALUES ('my deleted task', NOW(), (SELECT id FROM users WHERE username = 'deleteuser'));

INSERT INTO tasks (priority, title, description, is_default) VALUES 
('A', 'I am a task, you can complete me by checking the box', 'This is my description', true),
('B', 'See my details for by clicking me', 'My descirption can be changed', true);