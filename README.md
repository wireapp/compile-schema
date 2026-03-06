# `compile-schema`: Compile a SQL schema from a set of migrations

It is convenient to maintain a schema as a set of migrations. However, it's also nice to have a source of truth for the actual current state of the schema, and that's hard to accomplish when looking only at the migrations.

This project uses an in-memory SQLite database to evaluate the effects of a set of migrations, and then emit the overall resulting schema.

**Note**: as this always runs on an fresh, empty database, it _cannot_ validate the effects of data migrations; this only demonstrates the overall resulting shape of the database.

## Migrations Format

The path passed to this tool must be to a directory containing migrations. Migrations are SQL files named in the format `V{1}__{2}.sql`, where `{1}` is the migration version number and `{2}` is the migration name. Migrations are applied in ascending order of version number.