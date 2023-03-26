# Axum Template

Axum + SQLx + Docker + JWT-based OAuth2 Authorization
template to get started using axum faster. This template
is mainly for myself but anyone can use it if it fits
their requirements.

You need to have an oauth2 server/use existing server
to generate the token. Or you can just not use/delete
the `Claims`, `UserInfo`, or `BearerToken` extractor 
and create your own authentication mechanism.

Don't forget to change the Cargo.toml's package name.
And also update main.rs to use the correct package name.

## Usage
After cloning this repository, just search for TODO comment
and make necessary changes

## Project structure
- `dto` Contains all data transfer object used in routes
- `entities` Contains all database model, usually I use active
  record pattern.
- `routes` Contains all the routes this app need, and then tied
  together in the `startup.rs` file.
- `config.rs` Handles loading configuration from the `configuration`
  folder, you can change the app's environment prefix here.
- `startup.rs` Contains route mapping, database connection and cors
  configuration
