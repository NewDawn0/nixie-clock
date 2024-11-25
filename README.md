# Nixie clock

A simple command-line nixie clock which shows the date and time

## Requirements

- A truecolour terminal

## Installation

### Using cargo install

```bash
cargo install --git https://github.com/NewDawn0/nixie-clock.git
```

### Install using Nix

#### Imperatively

```bash
git clone https://github.com/NewDawn0/nixie-clock
nix profile install .
```

#### Declaratively

1. Add it as an input to your system flake as follows

   ```nix
   {
     inputs = {
       # Your other inputs ...
       nixie-clock = {
         url = "github:NewDawn0/nixie-clock";
         inputs.nixpkgs.follows = "nixpkgs";
         # Optional: If you use nix-systems or rust-overlay
         inputs.nix-systems.follows = "nix-systems";
         inputs.rust-overlay.follows = "rust-overlay";
       };
     };
   }
   ```

2. Add the overlay to expose nixie-clock to your pkgs

   ```nix
   overlays = [ inputs.nixie-clock.overlays.default ];
   ```

3. Then you can either install it in your `environment.systemPackages` using
   ```nix
   environment.systemPackages = with pkgs; [ nixie-clock ];
   ```
   or install it to your `home.packages`
   ```nix
   home.packages = with pkgs; [ nixie-clock ];
   ```

## Usage

```bash
$ nixie-clock
```
