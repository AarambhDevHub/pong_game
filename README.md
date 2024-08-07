# Pong Game in Rust with ggez

Welcome to the Pong game project! This repository contains the source code for a classic Pong game built using the Rust programming language and the ggez game library.

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Controls](#controls)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Overview
This project implements a simple Pong game where you control a paddle to bounce a ball against an AI opponent. The game demonstrates basic 2D game development concepts, including rendering graphics, handling user input, and managing game state.

## Features
- Player and AI-controlled paddles
- Ball movement and collision detection
- Scoring system
- Game over and restart functionality
- Adjustable game speed for AI difficulty

## Installation
To build and run this project, you'll need to have the Rust programming language installed. You can install Rust using rustup.

1. Clone the repository:
   ```bash
   git clone https://github.com/SuryodayDevHub/pong_game.git
   cd pong_game

2. Install dependencies:
    This project uses the ggez library. You can install the required dependencies using Cargo:
   ```bash
    cargo build

## Usage
To run the game, use the following command:
    ```bash
    cargo run

## Controls
    - **Spacebar**: Start or restart the game
    - **Up Arrow**: Move the player's paddle up
    - **Down Arrow**: Move the player's paddle down

## Project Structure
    
    ```bash
    pong_game/
    ├── Cargo.toml        # Project configuration and dependencies
    ├── src/
    │   └── main.rs       # Main game logic
    └── README.md         # Project documentation

# Key Components

- **main.rs**: Contains the implementation of the game, including game state management, rendering, and event handling.

# Contributing

Contributions are welcome! If you have suggestions for improvements or new features, feel free to open an issue or create a pull request.

## Donations

If you find this project useful and would like to support its continued development, you can make a donation via [Buy Me a Coffee](https://buymeacoffee.com/aarambhdevhub
).

# License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
