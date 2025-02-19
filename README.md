# Age

**Age** is a simple tool to check the installation time of your Arch Linux system.

## Installation

You can install **age** from the AUR (Arch User Repository) using `yay` or any other AUR helper:

```bash
yay -S system-age
```

Alternatively, you can manually clone the AUR repository and build it:

```bash
git clone https://aur.archlinux.org/system-age.git
cd system-age
makepkg -si
```

## Dependencies

- `glibc`
- `cargo` (for building the package)

## Usage

Once installed, you can check your system's installation time by running:

```bash
age
```

### Example Output:

```bash
canmi@xyy ~> age
Archlinux@canmi 6.13.2-arch1-1 already installed 7d 5h 53m 25s.
canmi@xyy ~>
```

The output will display the name of your system, the kernel version, and how long the system has been installed.

## Links

- [AUR Package](https://aur.archlinux.org/packages/system-age)
- [GitHub Repository](https://github.com/canmi21/age)
