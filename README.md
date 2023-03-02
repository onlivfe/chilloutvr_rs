# ChilloutVR API in rust

<img align="right" width="256" height="256" src="https://github.com/onlivfe/chilloutvr_rs/raw/main/logo.png"/>

[![License](https://img.shields.io/crates/l/chilloutvr.svg)](https://github.com/onlivfe/chilloutvr_rs/src/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/chilloutvr.svg)](https://crates.io/crates/chilloutvr)
[![Docs](https://docs.rs/chilloutvr/badge.svg)](https://docs.rs/crate/chilloutvr/)

A rust crate for [ChilloutVR](https://store.steampowered.com/app/661130/ChilloutVR/)'s API.

This is fully unofficial and in no way affiliated, endorsed, supported, or created by Alpha Blend Interactive, the creators of ChilloutVR.

The crate has models of the responses, with proper serde support.
It also definitions for the requests, using [`racal`](https://docs.rs/racal/latest/racal/) for the HTTP parts and big request/response structs for WebSockets, meaning that there's no lock-in to a single API client.
An example API client using [`reqwest`](https://crates.io/crates/reqwest) is provided for convenience though.

The API technically isn't public yet, so proceed with your own discretion.
That also means there is no official API documentation.
Which means it's possible that some things are wrong and/or will change a lot in the future.

## Testing

The integration tests contact the live API.
While a mock API could be created, it'd defeat the purpose of the tests.
Which is to see that the client can actually use the real current API.
While the requests & responses could be saved for a mock API, ensuring it stays up to date and behaves exactly like the real one is infeasible.

The integration tests are ignored by default for this reason.
A lot of the tests also require actual authentication with an account, which you can read more about below.

### Creating a user session manually

You can generate a `user-auth.json` file with logging in via curl for example:

```shell
curl --request POST \
  --url https://api.abinteractive.net/1/users/auth \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data '{
  "username": "email@Address",
  "password": "pa$$word",
  "authType": "LoginCredentials"
}' > user-auth.json
```

The resulting file should look something like the following:

```json
{
 "message": "Successfully logged in as ljoonal",
 "data": {
  "username": "ljoonal",
  "accessKey": "long-string",
  "userId": "uuid",
  "more fields...": "...and their values"
 }
}
```

### Running live API tests

Make sure that you've got an internet connection & a valid `user-auth.json`.

```sh
# A specific test with output logging
cargo test --all-features friend_requests -- --exact --ignored --nocapture
# All tests
cargo test --all-features -- --ignored
```

## License

Note that the license is `MPL-2.0` instead of the more common `MIT OR Apache-2.0`.
A license change however can be negotiated if the ABI team wants to use this crate or adopt this crate into a more official one with a different license.
