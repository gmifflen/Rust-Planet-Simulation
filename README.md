# Rust Planetary Simulation

This project is a planetary simulation written in Rust, creating an interactive visualization of planets orbiting the Sun. It's designed to demonstrate the principles of celestial mechanics, using real-world physics to simulate gravitational forces and planetary motion.

## Overview

The simulation features a dynamic model of planets orbiting a central star (the Sun), where each planet's motion is influenced by gravitational forces. The project is loosely based on a [Python-based planetary simulation](https://github.com/techwithtim/Python-Planet-Simulation) by Tech With Tim, adapted and expanded into Rust for enhanced performance and additional features.

## Features

- Realistic simulation of gravitational forces between planets and the Sun.
- Interactive visualization of planetary orbits.
- Scalable model accommodating additional celestial bodies.
- Adjustable simulation parameters for customized experiences.
- Informative display of planetary data.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your system. If not, follow the instructions on the [Rustup website](https://rustup.rs/) to install it.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/gmifflen/Rust-Planet-Simulation.git
   ```

2. Navigate to the project directory:
   ```bash
   cd rust-planet-simulation
   ```

3. Compile and run the project:
   ```bash
   cargo run
   or
   cargo run --release
   ```

## Usage

After starting the simulation, you will see planets orbiting the Sun. Each planet's motion is calculated in real-time based on gravitational forces.

## Contributing

I'm not the best at Rust, this is my second Rust project, my first being (Bounce)[https://github.com/gmifflen/Bounce].

Any suggestions on improvements or ways to write it in a better/safer way is much appreciated.

## Acknowledgements

- Special thanks to Tech With Tim for the original Python Planet Simulation, which inspired this project.
- This project is developed using Rust and the minifb library for windowing and buffer management.

## License

This project is open-source and available under the MIT License - see the LICENSE file for details.
