<script lang="ts">
    import { page } from "$app/stores";
    import logo from "../assets/icon.png";

    let instances = false;
    let mods = false;
    let manage = false;
    let spacewarp = false;
    let settings = false;

    $: {
        mods = /\/mods?(\/\d+)?/i.test($page.url.pathname);
        instances = /\/instances?(\/\d+)?/i.test($page.url.pathname);

        manage = $page.url.pathname == "/manage";
        spacewarp = $page.url.pathname == "/spacewarp" || $page.url.pathname == "/install";
        settings = $page.url.pathname == "/settings";
    }
</script>

<div class="header">
    <a class="logo-link" href="/">
        <img class="logo" src={logo} alt="insert wormhole logo here" />
    </a>

    <a class="link" class:active={spacewarp} href="/spacewarp">
        <i class="icon fa-solid fa-rocket" />
        SpaceWarp
    </a>

    <a class="link" class:active={instances} href="/instances">
        <i class="icon fa-solid fa-download" />
        Instances
    </a>

    <a class="link" class:active={mods} href="/mods">
        <i class="icon fa-solid fa-search" />
        Browse Mods
    </a>

    <a class="link" class:active={manage} href="/manage">
        <i class="icon fa-solid fa-sliders" />
        Manage Mods
    </a>

    <a class="link" class:active={settings} href="/settings">
        <i class="icon fa-solid fa-gear" />
        Settings
    </a>
</div>

<style lang="scss">
    .header {
        width: 100%;
        height: 2rem;

        user-select: none;

        padding: 1% 0;
        position: fixed;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;

        background-color: #2f2f2f;

        .logo {
            width: 64px;
            object-fit: cover;
            margin: 0;
        }

        .link {
            color: #aeaebe;
            margin: 0 1%;
            width: 9rem;
            padding: 0.5%;
            border: 1px solid transparent;
            border-radius: 6px;

            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;

            text-decoration: none;

            transition: color 0.5s ease, border 0.5s ease;

            &.active {
                color: white;
                border: 1px solid #dcdcec;
            }

            .icon {
                margin-right: 4%;
            }

            &:hover {
                color: #dcdcec;
            }
        }
    }
</style>
