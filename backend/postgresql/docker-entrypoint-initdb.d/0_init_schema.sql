\c finances;

CREATE TABLE IF NOT EXISTS companies (
    cik bigint PRIMARY KEY,
    ticker text UNIQUE NOT NULL,
    title text NOT NULL
);

