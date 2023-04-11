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

    <hr class="divider" />

    <a class="link" class:active={spacewarp} href="/spacewarp">
        <i class="icon fa-solid fa-rocket" />
        
        <span class="tooltip">SpaceWarp</span>
    </a>

    <a class="link" class:active={instances} href="/instances">
        <i class="icon fa-solid fa-download" />
        
        <span class="tooltip">Instances</span>
    </a>

    <a class="link" class:active={mods} href="/mods">
        <i class="icon fa-solid fa-search" />
        
        <span class="tooltip">Browse Mods</span>
    </a>

    <a class="link" class:active={manage} href="/manage">
        <i class="icon fa-solid fa-sliders" />
        
        <span class="tooltip">Manage Mods</span>
    </a>

    <a class="link" class:active={settings} href="/settings">
        <i class="icon fa-solid fa-gear" />
        
        <span class="tooltip">Settings</span>
    </a>
</div>

<style lang="scss">
    .header {
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
            padding: 0 30%;
            border: 1px solid transparent;
            border-radius: 6px;
            font-size: 16pt;
            outline: none;

            position: relative;

            display: inline-flex;
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

            &:hover .tooltip {
                opacity: 1;
            }
        }
    }
</style>
