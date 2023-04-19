<script lang="ts">
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
        manage = $page.url.pathname.endsWith("/manage");

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

<style lang="scss">
    @mixin tooltip-container {
        .tooltip {
            opacity: 0;
            background-color: #5f6160;
            color: #dcdcec;
            text-align: center;
            padding: 10% 40%;
            border-radius: 6px;
            pointer-events: none;
            width: 10rem;

            position: absolute;
            z-index: 1;

            left: 130%;

            transition: opacity 0.5s ease;

            &::after {
                content: " ";
                position: absolute;
                top: 50%;
                right: 100%;
                margin-top: -10px;
                border-width: 10px;
                border-style: solid;
                border-color: transparent #5f6160 transparent transparent;
            }
        }

        &:hover {
            .tooltip {
                opacity: 1;
            }
        }
    }

    .sidebar {
        width: 2rem;
        height: 100%;

        user-select: none;

        padding: 0 1%;

        display: inline-flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;

        background-color: #2f2f2f;

        .logo {
            width: 64px;
            height: 64px;
            padding: 0;
            margin: 0;

            object-fit: cover;
        }

        .divider {
            width: 90%;
            margin: 0 5%;
            height: 0;
            border: 0;
            padding: 0;

            margin-bottom: 0.5rem;
            border-bottom: 1px solid #aeaebe;
        }

        .link {
            color: #aeaebe;
            margin: 10% 0;
            height: 2.75rem;
            padding: 0 20%;
            border: 1px solid transparent;
            border-radius: 6px;
            font-size: 16pt;
            outline: none;
            width: 100%;

            position: relative;
            background-color: transparent;

            display: inline-flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;

            text-decoration: none;

            transition: color 0.5s ease, background-color 0.5s ease;

            &.active {
                color: white;
                background-color: #4f4f4f;
            }

            .icon {
                margin-right: 4%;
            }

            img {
                width: 100%;
            }

            &:hover {
                color: #dcdcec;
                background-color: #4f4f4f;
            }

            @include tooltip-container;
        }

        .group {
            width: 100%;
            padding: 0 30%;
            margin: 0;

            max-height: 0;
            position: relative;

            transition: max-height 0.5s ease-in-out;

            display: inline-flex;
            flex-direction: column;
            align-items: center;
            justify-content: space-evenly;

            background-color: #3f3f3f;
            border-radius: 8px;

            .link {
                opacity: 0;
                pointer-events: none;

                transition: opacity 0.5s ease, color 0.5s ease, background-color 0.5s ease;

                &:hover,
                &.active {
                    background-color: #5f5f5f;
                }
            }

            &.active {
                margin: 10% 0;
                max-height: 100%;

                .link {
                    opacity: 1;
                    pointer-events: unset;
                }
            }
        }
    }
</style>
