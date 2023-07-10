-- up.sql

CREATE TABLE msgs (
  id SERIAL PRIMARY KEY,
  roomId VARCHAR NOT NULL,
  senderId TEXT NOT NULL,
  body TEXT NOT NULL
)