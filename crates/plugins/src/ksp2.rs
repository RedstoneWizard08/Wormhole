//! The KSP2 support module.

use crate::unity_plugin;

unity_plugin!(ksp2::Kerbal2Plugin = {
    id: "ksp2",
    name: "Kerbal Space Program 2",
    game: 22407,
    exec: "KSP2_x64.exe",
    dir: "Kerbal Space Program 2",
    icon: "../assets/ksp2/icon.png",
    banner: "../assets/ksp2/banner.png",
    
    resolvers: [
        SpaceDock,
        Ckan
    ]
});
