INSERT INTO testing.account(user_name, email, full_name, password)
VALUES ($1, $2, $3, $4)
RETURNING account_id;
