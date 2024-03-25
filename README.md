### Install Rust WASM toolchain and Trunk (WASM packaging tool):

`rustup target add wasm32-unknown-unknown`
`cargo install --locked trunk`

Now you can run the app with the trunk development server:

`trunk serve --open`