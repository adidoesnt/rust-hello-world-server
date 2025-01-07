# Rust "Hello World" Server

## About

This is a simple server written in [Rust](https://www.rust-lang.org/) using the [Axum](https://github.com/tokio-rs/axum) web framework.

## Getting Started

### Prerequisites

- Rust
- PostgreSQL

### Installation

1. Clone the repository:

```bash
git clone https://github.com/adidoesnt/rust-hello-world-server.git
```

2. Navigate to the project directory:

```bash
cd rust-hello-world-server
```

3. Install the dependencies:

```bash
cargo install --path .
```

4. Create a `.env` file in the project directory and set the required environment variables:

```bash
cp .env.template .env
```

5. Start the server:

```bash
cargo run
```

## Usage

To access the server, open your web browser and navigate to `http://localhost:3000`.

## License

This project is licensed under the MIT License.
