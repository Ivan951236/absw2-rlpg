# Angry Birds Star Wars 2 Roguelike Preset Generator

A TUI application that generates character presets for Angry Birds Star Wars 2 roguelike gameplay.

## Features

- Choose between Bird Side or Pork Side characters
- Select from 6 different worlds with varying level counts:
  - Worlds 1-4: 20 levels each
  - World 5: 16 levels
  - World 6: 12 levels
- Generates up to 8 characters per preset
- Clean and intuitive terminal user interface

## Installation

1. Make sure you have Rust installed on your system. If not, install it from [rust-lang.org](https://www.rust-lang.org/tools/install)
2. Clone or download this repository
3. Navigate to the project directory

## Usage

Run the application with:

```bash
cargo run
```

Follow the on-screen prompts:
1. Select Bird Side (1) or Pork Side (2)
2. Enter the world number (1-6)
3. Enter the level number based on the world's maximum levels
4. View your generated character preset
5. Press Enter or Esc to generate a new preset

## Character Lists

### Bird Side Characters:
- Ezra Bridger
- Sabine Wren
- Chopper
- Hera Syndulla
- Kana Jarrus
- Garazeb "Zeb" Orrelios
- Qui-Gon Jinn
- Obi-Wan Kenobi
- Yoda
- Jar Jar Binks
- Anakin E1
- Captain Panakin
- Padmé
- Anakin E2
- Jedi Younglings
- Made Windu
- Chewbacca
- R2-D2
- C-3PO
- Silver C-3PO
- Han Solo
- Leia
- Pilot Luke
- Endor Luke
- Jedi Luke
- Lando
- Wicket
- Kit Fisto
- Carbonite Han Solo

### Pork Side Characters:
- The Inquisitor
- Agent Kallus
- AT-DP Pilot
- Cikatro Vizago
- Vizago Droid
- Imperial Officer
- Jango Fett
- Darth Sidious
- Battle Droid
- Darth Maul
- Count Dooku
- Droideka
- Zam Wesell
- General Grievous
- Anakin E3
- Biker Scout
- Stormtrooper
- TIE Fighter Pilot
- Darth Vader
- Boba Fett
- Hologram Darth Sidious
- Shadowtrooper
- Red Battle Droid
- Shocktrooper
- Jabba
- Royal Guard
- Tusken Raider

## Build

To build the application without running it:

```bash
cargo build --release
```

The executable will be available in `target/release/`.