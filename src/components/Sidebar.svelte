<script lang="ts">
    import "./sass/Sidebar.scss";
    import { page } from "$app/stores";
    import { KSPGame } from "../api/instance";
    import logo from "../assets/icon.png";
    import ksp1_logo from "../assets/ksp-square.png";
    import ksp2_logo from "../assets/ksp2-square.png";

    let instances = false;
    let mods = false;
    let manage = false;
    let spacewarp = false;
    let settings = false;
    let ksp1 = false;
    let ksp2 = false;

    $: {
        mods = /\/mods?(\/\d+)?/i.test($page.url.pathname);
        instances = /\/instances?(\/\d+)?/i.test($page.url.pathname);

        manage = $page.url.pathname == "/manage";
        spacewarp = $page.url.pathname == "/spacewarp" || $page.url.pathname == "/install";
        settings = $page.url.pathname == "/settings";

        ksp1 = $page.url.pathname.startsWith(`/${KSPGame.KSP1}`);
        ksp2 = $page.url.pathname.startsWith(`/${KSPGame.KSP2}`);
    }
</script>

<div class="sidebar">
    <a class="logo-link" href="/">
        <img class="logo" src={logo} alt="insert wormhole logo here" />
    </a>

    <hr class="divider" />

    <a class="link" class:active={spacewarp} href="/spacewarp">
        <i class="icon fa-solid fa-rocket" />

        <span class="tooltip">SpaceWarp</span>
    </a>

    <a class="link" class:active={ksp1} href="/{KSPGame.KSP1}">
        <img src={ksp1_logo} alt="KSP 1" />

        <span class="tooltip">KSP 1</span>
    </a>

    <div class="group" class:active={ksp1}>
        <a class="link" class:active={instances} href="/{KSPGame.KSP1}/instances">
            <i class="icon fa-solid fa-rocket" />

            <span class="tooltip">Instances</span>
        </a>

        <a class="link" class:active={mods} href="/{KSPGame.KSP1}/mods">
            <i class="icon fa-solid fa-search" />

            <span class="tooltip">Browse Mods</span>
        </a>

        <a class="link" class:active={manage} href="/{KSPGame.KSP1}/manage">
            <i class="icon fa-solid fa-sliders" />

            <span class="tooltip">Manage Mods</span>
        </a>
    </div>

    <a class="link" class:active={ksp2} href="/{KSPGame.KSP2}">
        <img src={ksp2_logo} alt="KSP 2" />

        <span class="tooltip">KSP 2</span>
    </a>

    <div class="group" class:active={ksp2}>
        <a class="link" class:active={instances} href="/{KSPGame.KSP2}/instances">
            <i class="icon fa-solid fa-rocket" />

            <span class="tooltip">Instances</span>
        </a>

        <a class="link" class:active={mods} href="/{KSPGame.KSP2}/mods">
            <i class="icon fa-solid fa-search" />

            <span class="tooltip">Browse Mods</span>
        </a>

        <a class="link" class:active={manage} href="/{KSPGame.KSP2}/manage">
            <i class="icon fa-solid fa-sliders" />

            <span class="tooltip">Manage Mods</span>
        </a>
    </div>

    <a class="link" class:active={settings} href="/settings">
        <i class="icon fa-solid fa-gear" />

        <span class="tooltip">Settings</span>
    </a>
</div>
