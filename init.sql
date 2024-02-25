CREATE DATABASE IF NOT EXISTS crud;

CREATE TABLE IF NOT EXISTS crud.tasks (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    priority VARCHAR(50),
    create_at TIMESTAMP NOT NULL DEFAULT NOW(),
);

DO $$
BEGIN
	INSERT INTO crud.tasks (name, priority)
	VALUES
		('Tarefa 1', 'Alta'),
        ('Tarefa 2', 'Média'),
        ('Tarefa 3', 'Baixa');
END;
$$;