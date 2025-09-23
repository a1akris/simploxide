# simploxide-client

SimpleX-Chat WebSocket client

## 🤖Try out SimplOxide bots right now!

1. [Install Rust](https://www.rust-lang.org/learn/get-started) if you haven't already.
1. [Get SimpleX chat CLI](https://github.com/simplex-chat/simplex-chat/blob/stable/docs/CLI.md#%F0%9F%9A%80-installation)
1. Run SimpleX Chat CLI with the following flags to prepare the bot account:
   ```bash
   simplex-chat --create-bot-display-name 'SimplOxide Examples' --create-bot-allow-files
   ```
    1. Alternatively, you can run this command from the SimpleX CLI itself:
       ```bash
       /create bot files=on 'SimplOxide Examples'
       ```
1. Run SimpleX Chat CLI as an API server listening on port 5225(all example
   bots are connecting to this port by default):
   ```bash
   simplex-chat -p 5225
   ```
1. Run the squaring bot:
   ```
   cargo run --example squaring_bot --all-features
   ```
   You can run any bot from [examples/](./examples) directory by changing the
   `--example` argument.
    1. All example bots output their address when they start. Use this address
       to connect to your bot in SimpleX app(always prefer the shorter one):
       ![Addresses Screenshot](./screenshots/addresses.png)
    1. Interact with the bot:
       ![Interactions Screenshot](./screenshots/interactions.png)

----

Explore [examples/](./examples) sources or read [crate
docs](https://docs.rs/simploxide-client/latest/simploxide_client/) to learn
how to write your own bots.

### LICENSE

#### DISCLAIMER

SIMPLEX CLI HAS AN OPEN WEBSOCKET API, AND IT CAN BE ACCESSED FROM ANY LIBRARY
OR APPLICATION, WHETHER OPEN-SOURCE OR NOT. THE SIMPLOXIDE LIBRARIES PROVIDE A
WEBSOCKET CLIENT AND API CODECS FOR SIMPLEX CLI THAT DON'T DEPEND ON ANY CODE
FROM THE SIMPLEX PROJECT AND THEREFORE ARE DUAL LICENSED UNDER APACHE-2.0/MIT
AS STATED BELOW. APACHE-2.0/MIT TERMS AND CONDITIONS ARE APPLICABLE TO YOUR
PROJECTS AS LONG AS THEY DON'T:

- SHIP SIMPLEX-CLI OR ANY OTHER SIMPLEX COMPONENTS AS PART OF AN APPLICATION
- USE FFI BINDINGS TO THE SIMPLEX-CORE OR OTHER SIMPLEX LIBRARIES
- DEPEND ON OR INCLUDE ANYTHING ELSE LICENSED UNDER AGPL

OTHERWISE, YOUR PROJECTS MUST ADHERE TO [SIMPLEX
AGPL-3.0](https://github.com/simplex-chat/simplex-chat/blob/stable/LICENSE).


THIS IS NOT A LEGAL ADVICE BUT RATHER A FRIENDLY REMAINDER.

SIMPLOXIDE LIBRARIES AUTHORS DISCLAIM ALL RESPONSIBILITY AND LIABILITY FOR ANY
FAILURE BY SIMPLOXIDE USERS TO COMPLY WITH THE AGPL-3.0.

---

Licensed under either of [Apache License, Version 2.0](../LICENSE-APACHE) or [MIT
license](../LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

