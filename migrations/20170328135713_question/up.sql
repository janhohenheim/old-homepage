CREATE TABLE category (
  id SERIAL PRIMARY KEY,
  text TEXT
);

CREATE TABLE question (
  id SERIAL PRIMARY KEY,
  category INT NOT NULL REFERENCES category,
  question TEXT NOT NULL
);

ALTER TABLE answer ADD question INT NOT NULL REFERENCES question;