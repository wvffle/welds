SELECT
    info.TABLE_SCHEMA,
    info.TABLE_NAME,
    info.COLUMN_NAME,
    info.REFERENCED_TABLE_SCHEMA,
    info.REFERENCED_TABLE_NAME,
    info.REFERENCED_COLUMN_NAME
FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE info
JOIN INFORMATION_SCHEMA.TABLE_CONSTRAINTS cons 
ON cons.CONSTRAINT_NAME = info.CONSTRAINT_NAME AND cons.CONSTRAINT_SCHEMA = info.CONSTRAINT_SCHEMA AND cons.CONSTRAINT_TYPE = 'FOREIGN KEY'
