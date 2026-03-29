ALTER TABLE members
    ALTER COLUMN name TYPE varchar(64),
    ALTER COLUMN email TYPE varchar(64),
    ALTER COLUMN role TYPE varchar(32);

ALTER TABLE organizations
    ALTER COLUMN name TYPE varchar(64),
    ALTER COLUMN description TYPE varchar(256);

ALTER TABLE divisions
    ALTER COLUMN name TYPE varchar(64),
    ALTER COLUMN description TYPE varchar(256);

AlTER TABLE groups
    ALTER COLUMN name TYPE varchar(64),
    ALTER COLUMN description TYPE varchar(256);

