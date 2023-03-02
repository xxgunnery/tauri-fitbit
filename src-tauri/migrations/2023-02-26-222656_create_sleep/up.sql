CREATE TABLE sleepdata (
    id SERIAL PRIMARY KEY,
    sleep_date TEXT UNIQUE NOT NULL,
    efficiency INTEGER NOT NULL,
    end_time TEXT NOT NULL,
    rem INTEGER NOT NULL,
    light INTEGER NOT NULL,
    deep INTEGER NOT NULL,
    wake INTEGER NOT NULL
)