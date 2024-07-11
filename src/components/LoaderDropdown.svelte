<script lang="ts">
    import { RPC, unwrap, type ModLoader, type ModLoaderType } from "$api/bindings/app";
import { getLoader, getLoaderVersion, getMinecraft } from "$api/loader";
import { isRelease, isSnapshot } from "$api/mc";
import { isChildOf } from "$api/util";
import { createEventDispatcher, onMount } from "svelte";

export let loader: ModLoader | undefined;
export let loading = false;

let dropdown: HTMLDivElement;
let button: HTMLButtonElement;
let open = false;
let snapshots = false;
let openLoader: ModLoaderType = "Vanilla";
let vanillaVer: string | undefined;
let vanilla: ModLoader[] = [];
let forge: ModLoader[] = [];
let neoforge: ModLoader[] = [];
let fabric: ModLoader[] = [];
let quilt: ModLoader[] = [];

const emitter = createEventDispatcher();

// For some reason, I have to repeat the call to `isSnapshot` here,
// otherwise the `isRelease` check won't work for snapshots. Why?
// Who knows.
$: filteredVanilla = snapshots
    ? vanilla
    : vanilla.filter((v) => isRelease(getMinecraft(v)) && !isSnapshot(getMinecraft(v)));

$: compatibleForge = forge.filter((v) => getMinecraft(v) === vanillaVer);
$: compatibleNeoForge = neoforge.filter((v) => getMinecraft(v) === vanillaVer);

$: openLoaders = (() => {
    switch (openLoader as ModLoaderType) {
        case "Forge":
            return compatibleForge;
        case "NeoForge":
            return compatibleNeoForge;
        case "Fabric":
            return fabric.reverse();
        case "Quilt":
            return quilt.reverse();
        default:
            return [];
    }
})().sort();

onMount(async () => {
    if (loader) {
        vanillaVer = getMinecraft(loader);
    } else {
        vanillaVer = getMinecraft(unwrap(await RPC.latestLoader.read("Vanilla")));
    }

    vanilla = unwrap(await RPC.loaders.read("Vanilla"));

    window.addEventListener("click", (ev) => {
        if (
            ev.target !== dropdown &&
            !isChildOf(ev.target as Node, dropdown) &&
            ev.target !== button &&
            !isChildOf(ev.target as Node, button)
        ) {
            open = false;
        }
    });

    window.addEventListener("keydown", (ev) => {
        if (ev.code === "Escape") {
            open = false;
        }
    });
});

const updater = (type: ModLoaderType) => {
    return async () => {
        if (type === openLoader) {
            openLoader = "Vanilla";

            return;
        }

        switch (type) {
            case "Vanilla":
                // vanilla = unwrap(await commands.getLoaders("Vanilla", null));
                break;
            case "Forge":
                // forge = unwrap(await commands.getLoaders("Forge", null));
                break;
            case "NeoForge":
                // neoforge = unwrap(await commands.getLoaders("NeoForge", null));
                break;
            case "Fabric":
                // fabric = unwrap(await commands.getLoaders("Fabric", null));
                break;
            case "Quilt":
                // quilt = unwrap(await commands.getLoaders("Quilt", null));
                break;
            case "None":
                loader = { Vanilla: vanillaVer || "" };
                emitter("change");
                break;
        }

        openLoader = type;
    };
};

const changer = (type: ModLoaderType, ver: string) => {
    return () => {
        switch (type) {
            case "Vanilla":
                vanillaVer = ver;

                if (loader && getLoader(loader) === "Vanilla") {
                    loader = { Vanilla: ver };
                } else if (loader && getLoader(loader) !== "None") {
                    (loader as any)[getLoader(loader)][0] = ver;
                }

                break;
            case "Forge":
                loader = { Forge: [vanillaVer!, ver] };
                break;
            case "NeoForge":
                loader = { NeoForge: [vanillaVer!, ver] };
                break;
            case "Fabric":
                loader = { Fabric: [vanillaVer!, ver] };
                break;
            case "Quilt":
                loader = { Quilt: [vanillaVer!, ver] };
                break;
        }

        emitter("change");
    };
};

const toggleOpen = () => {
    if (loading) return;

    open = !open;
};
</script>

{#if loader}
    {@const name = getLoader(loader)}
    {@const ver = getLoaderVersion(loader)}

    <button
        type="button"
        class="button"
        class:loading
        on:click={toggleOpen}
        bind:this={button}
    >
        {name === "None" ? "Vanilla" : name}
        {ver && ver != "" && ver != "." ? ver + "/" : ""}{vanillaVer}
        &nbsp;&nbsp;&nbsp;

        {#if loading}
            <i class="fa-solid fa-refresh fa-spin" />
        {:else if open}
            <i class="fa-solid fa-chevron-up" />
        {:else}
            <i class="fa-solid fa-chevron-down" />
        {/if}
    </button>
{/if}

<div
    class="dropdown"
    class:loader={openLoader !== "Vanilla" && openLoader !== "None"}
    class:open={open && !loading}
    bind:this={dropdown}
>
    <div class="left list">
        {#each filteredVanilla as v}
            {@const ver = getMinecraft(v)}

            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div
                class="item"
                on:click={changer("Vanilla", ver)}
                class:selected={vanillaVer === ver}
            >
                {ver}

                {#if isRelease(ver) && !isSnapshot(ver)}
                    <i class="fa-solid fa-star" />
                {/if}
            </div>
        {/each}
    </div>

    <div class="middle list">
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
            class="item"
            on:click={updater("Forge")}
            class:selected={openLoader === "Forge"}
        >
            Forge
            <i class="fa-solid fa-chevron-right" />
        </div>

        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
            class="item"
            on:click={updater("NeoForge")}
            class:selected={openLoader === "NeoForge"}
        >
            NeoForge
            <i class="fa-solid fa-chevron-right" />
        </div>

        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
            class="item"
            on:click={updater("Fabric")}
            class:selected={openLoader === "Fabric"}
        >
            Fabric
            <i class="fa-solid fa-chevron-right" />
        </div>

        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
            class="item"
            on:click={updater("Quilt")}
            class:selected={openLoader === "Quilt"}
        >
            Quilt
            <i class="fa-solid fa-chevron-right" />
        </div>

        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
            class="item"
            on:click={updater("None")}
            class:selected={openLoader === "None"}
        >
            None
        </div>
    </div>

    <div class="right list">
        {#if openLoaders.length > 0}
            {#each openLoaders as v}
                {@const ver = getLoaderVersion(v)}

                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <div
                    class="item"
                    on:click={changer(openLoader, ver || "")}
                    class:selected={getLoaderVersion(
                        loader || { Vanilla: "" },
                    ) === ver}
                >
                    {ver}
                </div>
            {/each}
        {/if}
    </div>
</div>

<style scoped lang="scss">
    .button {
        color: lightskyblue;
        background-color: transparent;

        border: 1px solid lightskyblue;
        padding: 1% 1.5%;
        margin: 0 3%;
        font-family: Ubuntu;
        font-size: 12pt;

        border-radius: 8px;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;

        cursor: pointer;
        outline: none;
        transition:
            color 0.5s ease,
            background-color 0.5s ease;

        &:hover {
            color: black;
            background-color: lightskyblue;
        }

        &.loading {
            cursor: not-allowed;

            &:hover {
                color: lightskyblue;
                background-color: transparent;
            }
        }
    }

    .dropdown {
        position: absolute;
        display: none;
        flex-direction: row;
        align-items: center;
        justify-content: flex-start;
        left: 60%;
        top: 57%;
        height: 40%;
        z-index: 30;
        width: 25%;
        background-color: var(--secondary-highlight-color);
        border-radius: 6px;
        padding: 0.5%;

        .left {
            border-right: 1px solid var(--fg-color-light);
        }

        .left,
        .middle,
        .right {
            overflow: scroll;
            height: 100%;
            width: 50%;
            display: flex;
            flex-direction: column;
            align-items: flex-start;
            justify-content: flex-start;

            .item {
                width: calc(100% - 1rem);
                height: 1.5rem;
                padding: 0.5rem;
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: space-between;
                cursor: pointer;

                transition: background-color 0.25s ease;

                &:hover {
                    background-color: var(--secondary-color);
                }

                &.selected {
                    background-color: var(--secondary-selected-color);
                }
            }
        }

        .left {
            border-top-left-radius: 6px;
            border-bottom-left-radius: 6px;
        }

        .right {
            display: none;
        }

        .middle {
            overflow: hidden;
        }

        &.loader {
            width: 37.5%;

            .left,
            .middle,
            .right {
                width: 33%;
            }

            .middle {
                border-right: 1px solid var(--fg-color-light);
            }

            .right {
                display: flex;
            }
        }

        &.open {
            display: flex;
        }
    }
</style>
