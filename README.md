# ChilloutVR API in rust

<img align="right" width="256" height="256" src="https://github.com/onlivfe/chilloutvr_rs/raw/main/logo.png"/>

[![License](https://img.shields.io/crates/l/chilloutvr.svg)](https://github.com/onlivfe/chilloutvr_rs/src/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/chilloutvr.svg)](https://crates.io/crates/chilloutvr)
[![Docs](https://docs.rs/chilloutvr/badge.svg)](https://docs.rs/crate/chilloutvr/)

WIP predicted rust models of [ChilloutVR](https://store.steampowered.com/app/661130/ChilloutVR/)'s upcoming API.

This is fully unofficial and in no way affiliated, endorsed, supported, or created by Alpha Blend Interactive, the creators of ChilloutVR.

Note that there is no official API documentation yet and usage of the API is frowned upon as of writing.
If it wasn't clear enough: purpose of this crate isn't to connect to the API currently.
The purpose is to be able to model clients to use the models before the API goes live.
Though note that the models will most likely change in breaking ways.

Once the API is stabilized a bit more and it's usage allowed, an API client is planned to be implemented.

## Testing

The integration tests contact the live API.
That's why they are ignored by default.

Some of them also require authentication.

Sadly not all the things can even be reliably tested without creating a mock API.
Which in turn defeats the purpose of the tests in the first place.

### Creating a user session manually

You can generate a `user-sesion.json` file with logging in via curl for example:

```shell
curl --request POST \
  --url https://api.abinteractive.net/1/users/auth \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data '{
  "username": "email@Address",
  "password": "pa$$word",
  "authType": "LoginCredentials"
}' > user-session.json
```

### Running ignored tests

Make sure that you've got:

- an internet connection
- a valid `user-sesion.json`

Then just run the tests;

```sh
# A specific test with output logging
cargo test --all-features friend_requests -- --exact --ignored --nocapture
# All tests
cargo test --all-features -- --ignored
```

## License

Note that the license is `MPL-2.0` instead of the more common `MIT OR Apache-2.0`.
A license change however can be negotiated if the ABI team wants to use this crate or adopt this crate into a more official one with a different license.
