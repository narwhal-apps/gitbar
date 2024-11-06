<h1 align="center">GitBar</h1>
<p align="center"><img src="./app-icon.png" width="60" height="60" alt="app icon" />
</p>

An system tray app for showing your pull requested reviews.

![image](./website/static/screenshot.png)

## Development instructions

### Requirements

[Tauri getting started](https://tauri.app/v1/guides/getting-started/prerequisites)

### Instructions

```sh
# 1 - install dependencies
pnpm i

# 2 - Run the App in development mode
pnpm tauri:dev
```

> note that the first run will take time as tauri download and compile dependencies.

### Production

```sh
pnpm tauri:build
```

## Icons
https://primer.style/foundations/icons/
