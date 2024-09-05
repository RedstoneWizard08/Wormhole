# Modpacks

## Pack Info (pack.toml)

```toml
[meta]
name = "My Modpack"
version = "0.1.0" # Adheres to SemVer
authors = ["Me!"]

[versions]
minecraft = "1.21.x" # SemVer range

[[export]]
side = "client"
format = "modrinth"

[[export]]
side = "client"
format = "curseforge"

[[export]]
side = "server"
format = "zip"

[[export]]
side = "server"
format = "tar+gz"

[[export]]
side = "both"
format = "wormhole"

[[loaders]]
id = "fabric"
version = "0.15.11"

[[loaders]]
id = "quilt"
version = "0.26.1-beta.1"

[[loaders]]
id = "forge"
version = "51.0.16"

[[loaders]]
id = "neoforge"
version = "21.0.21-beta"
```

## Version Handling Example

```rs
/*
[dependencies]
anyhow = "1.0.86"
semver = "1.0.23"
lenient_semver = "0.4.2"
*/

use anyhow::Result;
use semver::VersionReq;

fn main() -> Result<()> {
    let v = VersionReq::parse("1.21.x")?;
    let n = lenient_semver::parse("1.21")?;
    let n2 = lenient_semver::parse("1.20.1")?;
    let n3 = lenient_semver::parse("1.21.1")?;
    let n4 = lenient_semver::parse("1.22")?;

    println!("{} in {}: {}", n, v, v.matches(&n));
    println!("{} in {}: {}", n2, v, v.matches(&n2));
    println!("{} in {}: {}", n3, v, v.matches(&n3));
    println!("{} in {}: {}", n4, v, v.matches(&n4));

    Ok(())
}
```

## Commands

-   `wh pack init`: Initialize a modpack project (interactively or non-interactively)
-   `wh pack add [slug|id|url] (--file [file_id] --source [source])`: Add a mod
-   `wh pack export (--format [format] --side [side])`: Export the pack
-   `wh pack refresh`: Refresh the index lock
-   `wh pack list`: List all mods in a modpack
-   `wh pack update (--all)`: Update mods

## Mod Metadata (mods/[mod_id].toml)

```toml
# mods/sodium.toml
[mod]
id = "sodium"
source = "modrinth"
version = "mc1.21-0.5.9"
side = "client"
loaders = ["fabric", "quilt"]

[file]
hash-format = "sha1"
hash = "<sha1>"
url = "<jar_url>"
```

## Index File (index.lock)

```rs
// Binary Serialized
// THIS IS PSEUDOCODE! NOT AN ACTUAL MACRO!

define_binary_format! {
    gzip = true,

    input = array({
        path: str,
        hash: [char; 40],
    }),

    body = {
        // Header
        ascii("INDX");
        NUL;
        literal(1 => u8);
        NUL;

        // Body
        ascii("DATA");
        NUL

        array({
            ascii(F);
            NUL;
            input(path => str + '\0'); // Yes there are two null characters after the string.
            NUL;
            input(hash => [char; 40]); // sha-1 is always 40 chars long
            NUL;
        });

        // End
        EOF;
    }
}
```

## Wormhole Pack Format

### manifest.json

```json
{
    "$schema": "TODO",
    "version": 1,

    "pack": {
        "name": "My Modpack",
        "version": "0.1.0",

        "authors": [
            "Me!"
        ]
    },

    "management": {
        "game_id": "mc", // Wormhole Plugin ID
        "game_version": "1.21.x", // Optional for anything but Minecraft

        "mod_loaders": [
            {
                "id": "fabric",
                "version": "0.15.11"
            },

            {
                "id": "quilt",
                "version": "0.26.1-beta.1"
            },

            {
                "id": "forge",
                "version": "51.0.16"
            },

            {
                "id": "neoforge",
                "version": "21.0.21-beta"
            }
        ]
    },

    "files": [
        {
            "path": "options.txt",
            "hash": {
                "format": "sha1",
                "value": "<sha1>"
            }
        }

        ...
    ],

    "mods": [
        {
            "id": "sodium",
            "source": "modrinth",
            "version": "mc1.21-0.5.9",
            "loaders": ["fabric", "quilt"],
            "url": "<jar_url>",

            "hash": {
                "format": "sha1",
                "value": "<sha1>"
            }
        },

        {
            "id": "embeddium",
            "source": "modrinth",
            "version": "1.0.0-beta.1+mc1.21",
            "loaders": ["forge", "neoforge"],
            "url": "<jar_url>",

            "hash": {
                "format": "sha1",
                "value": "<sha1>"
            }
        }
    ]
}
```
