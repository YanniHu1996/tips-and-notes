CREATE TABLE IF NOT Exists users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50),
    email VARCHAR(50)
);


CREATE OR REPLACE FUNCTION insert_users(num_users INTEGER) RETURNS void AS $$
BEGIN
    INSERT INTO users (name, email)
    SELECT
      'User_' || s.i,
      'user_' || s.i || '@example.com'
    FROM generate_series(1, num_users) AS s(i);
END;
$$ LANGUAGE plpgsql;






