# Turing Game

Turing Game is an educational browser-based game designed to teach the fundamentals of computer science, starting from the transistor level and progressing through layers of abstraction to build a basic computer. The game is built using Rust and WebAssembly.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Features
- Interactive tutorials on transistors, logic gates, adders, multiplexers, and more.
- Visually demonstrates how basic components work together to build complex circuits.
- Supports various logic operations and arithmetic functions.
- Educational content for beginners and enthusiasts.

## Installation
To run Turing Game locally, follow these steps:

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [npm](https://www.npmjs.com/get-npm) (Node Package Manager)

### Steps
1. Clone the repository:
    ```sh
    git clone https://github.com/your-username/turing-game.git
    cd turing-game
    ```

2. Build the WebAssembly package:
    ```sh
    wasm-pack build --target web --out-dir ./static/pkg
    ```

3. Install the HTTP server if you don't have it installed:
    ```sh
    npm install -g http-server
    ```

4. Start the server:
    ```sh
    http-server static
    ```

5. Open your browser and navigate to `http://localhost:8080`.

## Usage
Select a level from the menu to begin learning about transistors and logic gates. Each level provides an interactive circuit diagram that allows you to toggle inputs and observe the outputs.

## Development
To contribute to the development of Turing Game, follow these steps:

### Prerequisites
- Rust
- wasm-pack
- npm

### Steps
1. Fork the repository.
2. Create a new branch:
    ```sh
    git checkout -b feature/your-feature-name
    ```

3. Make your changes and commit them:
    ```sh
    git commit -m "Description of your changes"
    ```

4. Push to the branch:
    ```sh
    git push origin feature/your-feature-name
    ```

5. Open a pull request.
