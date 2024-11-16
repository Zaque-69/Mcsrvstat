# Mcsrvstat Rust

**Mcsrvstat** is a Text User Interface (TUI) application written in Rust, designed to quickly and efficiently display information about a Minecraft server. With a simple interface accessible directly from the terminal, Mcsrvstat enables users to view real-time data about the server's status, such as the active player count, server version, and more. It is ideal for server administrators or players who want to connect to their favorite server and quickly check its activity and configuration.

## Features

- **Quick Server Check**: Mcsrvstat displays essential Minecraft server information, including version, MOTD, active players, and maximum player count, API version, IP.
- **Intuitive and Responsive Terminal Design**: The TUI interface is designed to be user-friendly and easy to navigate, presenting information in a structured way.
- **Real-Time Data**: The application fetches up-to-date server information for each query, ensuring accuracy.
- **Minimalism and Performance**: Built to run quickly and efficiently, Mcsrvstat is ideal for continuous monitoring.

## Installation

To install Mcsrvstat on your system, you need **Rust** and **Cargo**.

Clone the project:

```sh
git clone https://github.com/Zaque-69/mcsrvstat.git
cd mcsrvstat
```

## Run the App

To run Mcsrvstat on your system, you need to execute the following commands : 

### For Nix users : 
```sh
direnv allow
```

## Showing the data in real-time :
 
```sh
cargo run hypixel.net
```

## Screenshots

<p align = "left">
  <img width="400" alt="webui" src="https://github.com/Zaque-69/mcsrvstat/blob/main/assets/tui.png">
</p>
