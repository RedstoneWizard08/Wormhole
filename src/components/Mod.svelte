<script lang="ts">
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import { commands, type SourceMapping, type Mod } from "$bindings";
    import { unwrap } from "$api/util";

    export let mod: Mod;
    export let game: number;
    export let instance: number;

    let source: SourceMapping | null = null;
    let installed = false;
    let installing = false;

    const img = import.meta.env.DEV
        ? mod.icon?.replace("https://cdn.modrinth.com/", "/__mr_cdn/")
        : mod.icon;

    const fmt = new Intl.NumberFormat("en-US", {
        notation: "compact",
        compactDisplay: "short",
    });

    onMount(async () => {
        source = unwrap(await commands.getSourceId(mod.source, null)) as SourceMapping;

        const mods = unwrap(await commands.getMods(instance, null));

        installed = mods.find((v) => v.mod_id == mod.id) != null;
    });

    const download = async (ev: MouseEvent) => {
        ev.preventDefault();
        ev.stopPropagation();

        installing = true;

        // TODO: Version dropdown
        const resolver = unwrap(await commands.getSourceId(mod.source, null)) as SourceMapping;
        const latest = unwrap(await commands.getLatestVersion(game, resolver, mod.id, null));
        const instanceInfo = unwrap(await commands.getInstance(instance, null));

        if (installed) {
            const mods = unwrap(await commands.getMods(instance, null));
            const me = mods.find((v) => v.mod_id == mod.id);

            unwrap(await commands.uninstallMod(game, me!, instanceInfo, null));
        } else {
            unwrap(await commands.installMod(game, mod, latest, instanceInfo, null));
        }

        const mods = unwrap(await commands.getMods(instance, null));

        installed = mods.find((v) => v.mod_id == mod.id) != null;

        installing = false;
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    class="mod-tile"
    on:click={() => goto(`/${game}/mod/${source}/${mod.id}?instance=${instance}`)}>
    {#if img}
        <!-- svelte-ignore a11y-img-redundant-alt -->
        <img src={img} class="image" alt="mod background image" />
    {/if}

    <div class="after-img">
        <div class="info">
            <p class="title">{mod.name}</p>
            <p class="author">by {mod.author}</p>
        </div>

        <div class="right">
            <div class="stats">
                <div class="stat">
                    <i class="icon fa-solid fa-circle-down" />
                    <p class="text">{fmt.format(mod.downloads)}</p>
                </div>

                <div class="stat">
                    <i class="icon fa-solid fa-eye" />
                    <p class="text">{fmt.format(mod.followers)}</p>
                </div>
            </div>

            <button type="button" class="action" on:click={download}>
                {#if installing}
                    <i class="icon progress fa-solid fa-spinner fa-spin" />
                {:else if installed}
                    <i class="icon delete fa-solid fa-trash-can" />
                {:else}
                    <i class="icon fa-solid fa-circle-down" />
                {/if}
            </button>
        </div>
    </div>
</div>

<style scoped lang="scss">
    .mod-tile {
        width: 96%;
        height: 4rem;

        user-select: none;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;

        margin: 0.5% 0;
        padding: 1% 1.5%;
        border-radius: 4px;

        transition: background-color 0.5s ease;
        cursor: pointer;

        &:hover {
            background-color: #3f4140;
        }

        .image {
            height: 100%;
            object-fit: cover;
            border-radius: 4px;
        }

        .after-img {
            width: 100%;
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: space-between;

            .info {
                display: flex;
                flex-direction: column;
                align-items: flex-start;
                justify-content: center;

                & > p {
                    margin: 0.5rem;
                }

                .author {
                    color: var(--fg-color-light);
                }
            }

            .right {
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: flex-end;

                .stats {
                    display: flex;
                    flex-direction: column;
                    align-items: flex-start;
                    justify-content: center;
                    margin-right: 2rem;

                    .stat {
                        display: flex;
                        flex-direction: row;
                        align-items: center;
                        justify-content: flex-start;

                        margin: 0;
                        width: 4rem;

                        .text {
                            margin: 0.5rem;
                        }
                    }
                }

                .action {
                    border: none;
                    background-color: transparent;

                    .icon {
                        color: #90ee90;
                        font-size: 14pt;
                        cursor: pointer;

                        transition: color 0.5s ease;

                        &:hover {
                            color: #50ae50;
                        }

                        &.delete {
                            color: #ac2c2c;
                            font-size: 14pt;
                            cursor: pointer;

                            transition: color 0.5s ease;

                            &:hover {
                                color: #dc2c2c;
                            }
                        }

                        &.progress {
                            color: #ac2cac;
                            font-size: 14pt;
                            cursor: pointer;

                            transition: color 0.5s ease;

                            &:hover {
                                color: #dc2cdc;
                            }
                        }
                    }
                }
            }
        }
    }
</style>
