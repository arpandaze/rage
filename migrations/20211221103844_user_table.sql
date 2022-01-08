CREATE TABLE users (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  first_name VARCHAR(15) NOT NULL,
  middle_name VARCHAR(15),
  last_name VARCHAR(15) NOT NULL,
  email VARCHAR(50) NOT NULL,
  phone VARCHAR(10),
  two_fa_secret VARCHAR,
  hashed_password VARCHAR,
  is_verified BOOLEAN DEFAULT(False),
  is_active BOOLEAN DEFAULT(True),
  user_type SMALLINT NOT NULL DEFAULT(0),
  access_zones SMALLINT[],
  created_on TIMESTAMP without time zone DEFAULT NOW()
);

