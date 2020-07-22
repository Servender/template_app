Template for Rust app
========================

After starting needing:
1. create `configurations/config.toml` (mv configurations/config.example.toml > configurations/config.toml) and cpnfigure for your machine.
2. change migration dir for your database (***dir name***)
3. initialize database (create .env file formats: `DATABASE_URL=postgres://{server}:{password}@{host}:{port}/{database}` and start command `diesel migration run`)

