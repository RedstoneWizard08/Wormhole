use anyhow::{anyhow, Result};

use crate::sources_enum;

sources_enum!(Ckan, CurseForge, Modrinth, Nexus, SpaceDock, Thunderstore);
