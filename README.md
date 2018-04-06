# Statically Linked React-Rust App

A statically linked app built with React and Rust. Based on [this blog post](https://anderspitman.net/2018/04/04/static-react-rust-webapp/) by Anders Pitman.

## Build steps

React development
```sh
# Just the React app: use webpack-dev-server
cd ui
npm run start

# Access at http://localhost:5000
```

Rust development
```
# Build the React app first
cd ui
npm run build

# Build and run the Rust app
cd ..
cargo run

# Access at http://localhost:5000
```

Production
```
# Build the React app
cd ui
npm run build

# Build the Rust app
cd ..
cargo build --release

# Run the binary
./target/release/react-rust

# Access at http://localhost:5000
```
