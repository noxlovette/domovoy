# domovoy

Terminal UI for controlling Yandex Smart Home lights.

## Setup

```
cargo install --path domovoy
```

Authenticate with your Yandex account:

```
domovoy auth
```

To clear saved credentials:

```
domovoy reset
```

## Usage

```
domovoy tui
```

The list shows all colour-capable lights and light groups from your account. Groups are prefixed with `[G]`. The `●` / `○` indicator shows the last known power state.

### Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `Enter` | Open colour picker |
| `o` | Turn selected light or group **on** |
| `O` | Turn selected light or group **off** |
| `q` / `Esc` | Quit |

**In the colour picker:**

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `Enter` | Apply colour |
| `Esc` | Cancel |
| `q` | Quit |

## Logs

Logged to `/tmp/domovoy.log`.
