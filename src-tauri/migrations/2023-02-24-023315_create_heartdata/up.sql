CREATE TABLE heartdata (
  id SERIAL PRIMARY KEY,
  date_time TEXT UNIQUE NOT NULL,
  resting_rate INTEGER NOT NULL
)