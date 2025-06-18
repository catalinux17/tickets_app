# Docs

A nice mix of Rust + Svelte to help my parents with their job

## How it works?

1. Select Date
1. Select Boarding station
1. Select Destination
1. Type a valid ticket number ex.: `111-111111` or `11111111` (box outline should turn green and the "Send" button should be active now)
1. Press "*Send*" or `Enter`
1. A directory should came up on your *Desktop* (regardless of OS) with a *CSV* file with the tickets

![demo](/demo.gif)

## Prerequisites

1. `yarn`

## How to run?

1. `yarn` - install packages
1. `yarn tauri dev` - launch the dev server OR `yarn tauri build` - to install it as a MacOS/Windows app
