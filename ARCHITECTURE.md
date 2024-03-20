# Architecture

Wormhole's architecture will be as follows:

                |---> [Frontend](#frontend)
                |
[Backend](#backend) ------|---> [Query Service](#query-service)
                |
                |---> [Database](#database)

## Backend

The backend acts as a sort of control plane. It orchestrates the rest of the components and runs the app.
The backend will also house the main API which can be used by [plugins](#plugins).

## Frontend

The frontend will either be the CLI or the GUI.

The GUI is powered by Tauri, and uses the SvelteKit framework to do its rendering logic.
The basic flow will be:

- Home
    - Game list
        - [Game]
            - Instances
                - [Instance]
                    - Manage mods:
                        - Browse & add mods
                        - Remove mods based on a list
                            - List contains:
                                - Mod name
                                - File name
                                - File size
                                - Modified date
                        - Update description
                        - View log outputs
                        - Manage instance-specific settings
                - Create
                    - Instance created
                    - Redirect to its page
            - Settings
                - Change game path
                - Log in to game accounts
    - Settings

For the CLI, there will be a few commands:

NOTE: `wh` is an alias for `wormhole`.

- `wh version` - Print version information
- `wh gui` - Open the GUI version
- `wh game list` - List installed games
- `wh game detect` - Detect installed games
- `wh game remote`- List supported games
- `wh game install [game]` - Install a game (using the proper launcher)
- `wh game uninstall [game]` - Uninstall a game (using the proper launcher)
- `wh game info [game]` - Print information about a game
- `wh inst(ance) [game] (list)` - List instances
- `wh inst [game] add` - Open a prompt interface to create a new instance
- `wh inst [game] del [name]` - Remove an instance
- `wh inst [game] mods [name]` - List mods for an instance
- `wh inst [game] mods [name] add [mod]` - Add a mod to an instance
- `wh inst [game] mods [name] del [mod]` - Remove a mod from an instance
- `wh inst [game] mods [name] update ([mod]...)` - Update all mods for an instance
- `wh update` - Check for updates
- `wh cache info` - Get information about cache usage
- `wh cache prune` - Prune cached files
- `wh reset` - Reset and remove all instances and installed games, clearing all data.

The CLI will be installed alongside the GUI.

## Query Service

The Query Service manages mod sources and indexing. This handles searching for mods, getting metadata,
retaining metadata, and much more.

Mod sources:
- Spacedock (supported)
- CKAN (WIP) ([crate](https://crates.io/crates/ckandex))
- CurseForge (planned)
- Modrinth (planned)
- Thunderstore (planned)
- Nexus Mods (planned*)
- More?**

\* Nexus mods is dependent on the developer friendliness and capability of its API.
\*\* More can be suggested! Go to the repository discussions tab and suggest there!

The Query Service will also cache some results into the database to aid in not spamming these services.
These cached results will mostly include mod version-specific metadata, and **NOT** dynamic data like version lists,
general mod metadata, and search results.

## Database

The database will be powered by sqlite (rusqlite) and will provide metadata storage about installed games,
instances, mod lists, modpacks, plugins, settings, and cache information.

## Modpacks

Modpacks will be supported! A special `.whmp` (**W**orm**H**ole **M**od**P**ack) format will be used to share modpacks.
These can then be uploaded to Wormhole's custom registry and shared publicly.

## Plugins

Plugins will be implemented by using a runtime WASI plugin loader and will be written in Rust. These will have full access
to Wormhole's API and system details. BE CAREFUL!

The implementation will be like this.

1. Load the WASI (`.wasm`) module.
2. Dynamically execute the contained initializer.