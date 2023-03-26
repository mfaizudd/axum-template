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
