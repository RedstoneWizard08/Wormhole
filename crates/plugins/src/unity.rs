//! Plugins for unity-based games.

use crate::unity_plugin;

unity_plugin!(ksp1::Kerbal1Plugin = {
    id: "kerbal-1",
    name: "Kerbal Space Program",
    game: 3102,
    exec: "KSP_x64.exe",
    dir: "Kerbal Space Program",
    icon: "../assets/ksp1/icon.png",
    banner: "../assets/ksp1/banner.png",

    resolvers: [
        SpaceDock,
        Ckan
    ]
});

unity_plugin!(ksp2::Kerbal2Plugin = {
    id: "kerbal-2",
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

unity_plugin!(cw::ContentWarningPlugin = {
    id: "content-warning",
    name: "Content Warning",
    game: 2881650,
    exec: "Content Warning.exe",
    dir: "Content Warning",
    icon: "../assets/content/icon.png",
    banner: "../assets/content/banner.png",

    resolvers: [
        Thunderstore
    ]
});

unity_plugin!(lc::LethalCompanyPlugin = {
    id: "lethal-company",
    name: "Lethal Company",
    game: 1966720,
    exec: "Lethal Company.exe",
    dir: "Lethal Company",
    icon: "../assets/lethal/icon.png",
    banner: "../assets/lethal/banner.png",

    resolvers: [
        Thunderstore
    ]
});
