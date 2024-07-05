<script lang="ts">
import DOMPurify from "dompurify";
import { page } from "$app/stores";
import LoadingPage from "$components/LoadingPage.svelte";
import { listen, unwrap } from "$api/util";
import type { SourceMapping, Mod, ModVersion } from "$bindings";
import { marked } from "marked";
import { onMount } from "svelte";
import Dropdown from "$components/Dropdown.svelte";
import type { DropdownItem } from "$api/dropdown";

const modId = $page.params.mod;
const source = $page.params.source;
const gameId = Number.parseInt($page.params.game);

let modInfo: Mod | null = null;
let isLoading = true;
let mods = false;
let instanceId = Number.parseInt($page.url.searchParams.get("instance") || "-1");
let downloading = false;
let total = 0;
let progress = 0;
let icon: string | null | undefined = null;
let versions: ModVersion[] = [];
let latest: ModVersion | null = null;
let selected: DropdownItem = { id: "", text: "" };
let installed = false;

$: items = versions.map((v) => ({ id: v.id, text: v.name }) as DropdownItem);

const fmt = new Intl.NumberFormat("en-US", {
    notation: "compact",
    compactDisplay: "short",
});

onMount(async () => {
    if (modId || $page.url) {
        // const instance = unwrap(await commands.getInstance(instanceId, null));

        mods = /\/mods?(\/\d+)?/i.test($page.url.pathname);
        // modInfo = unwrap(await commands.getMod(gameId, source as SourceMapping, modId, null));
        // versions = unwrap(
        //     await commands.getModVersions(gameId, source as SourceMapping, instance, modId, null)
        // );
        // latest = unwrap(
        //     await commands.getLatestVersion(gameId, source as SourceMapping, instance, modId, null)
        // );
        isLoading = false;

        selected = { id: latest.id, text: latest.name! };

        icon = import.meta.env.DEV
            ? modInfo.icon?.replace("https://cdn.modrinth.com/", "/__mr_cdn/")
            : modInfo.icon;

        // const modsList = unwrap(await commands.getMods(instanceId, null));

        // installed = modsList.find((v) => v.mod_id === modId) != null;
    }

    listen("progress_callback", (data) => {
        downloading = true;

        const payload = data.payload as any;

        total = payload.total;
        progress = payload.current;

        if (total === progress) {
            downloading = false;
        }
    });
});

const linkFix = (html: string) => {
    const linkRegex = /(<a\s+(?!.*\btarget=)[^>]*)(href="https?:\/\/)(.*?")/gi;

    const matches = html.matchAll(linkRegex);

    for (const match of matches) {
        const startTag = match[1];
        const href = match[2] + match[3];
        const newStartTag = `${startTag} target="_blank"`;
        const newLink = `${newStartTag} ${href}`;
        html = html.replace(match[0], newLink);
    }

    return html;
};

const imageFix = (html: string) => {
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, "text/html");
    const images = doc.querySelectorAll("img");

    for (const img of images) {
        img.style.maxWidth = "50%";
    }

    return doc.body.innerHTML;
};

const handleHtml = (html: string) => {
    const processes = [
        (html: string) => marked(html) as string,
        DOMPurify.sanitize,
        linkFix,
        imageFix,
    ];

    html = processes.reduce((html, process) => process(html), html);

    if (import.meta.env.DEV) {
        html = html.replace("https://cdn.modrinth.com/", "/__mr_cdn/");
    }

    return html;
};

const install = async () => {
    downloading = true;

    const version = versions.find((v) => v.id === selected.id)!;
    // const instance = unwrap(await commands.getInstance(instanceId, null));

    // unwrap(await commands.installMod(gameId, modInfo!, version, instance, null));

    // const mods = unwrap(await commands.getMods(instance.id!, null));

    // installed = mods.find((v) => v.mod_id === modId) != null;

    downloading = false;
};

const uninstall = async () => {
    downloading = true;

    // const instance = unwrap(await commands.getInstance(instanceId, null));
    // const modsNow = unwrap(await commands.getMods(instance.id!, null));
    // const me = modsNow.find((v) => v.mod_id === modId)!;

    // unwrap(await commands.uninstallMod(gameId, me, instance, null));

    // const mods = unwrap(await commands.getMods(instance.id!, null));

    // installed = mods.find((v) => v.mod_id === modId) != null;

    downloading = false;
};
</script>

{#if isLoading}
    <LoadingPage />
{:else}
    <div class="full-mod-container">
        <a class={`link ${mods ? "active" : ""}`} href={`/${gameId}/mods?instance=${instanceId}`}>
            <div class="return-container">
                <div class="return-arrow">
                    <i class="fa-solid fa-long-arrow-left" />
                </div>
                <div class="return-circle" />
            </div>
        </a>

        <div class="mod">
            <div class="infos">
                {#if icon}
                    <!-- svelte-ignore a11y-img-redundant-alt -->
                    <img src={icon} class="icon" alt="{modInfo?.name} Icon" />
                {/if}

                <div class="texts">
                    <div class="left">
                        {#if modInfo?.url}
                            <a class="name" href={modInfo?.url}>{modInfo?.name}</a>
                        {:else}
                            <p class="name">{modInfo?.name}</p>
                        {/if}

                        {#if modInfo?.author}
                            <p class="author">by {modInfo?.author}</p>
                        {/if}
                    </div>

                    <div class="right">
                        {#if modInfo?.downloads}
                            <p class="downloads">
                                <i class="fa-solid fa-circle-down" />
                                &nbsp;&nbsp;
                                {fmt.format(modInfo?.downloads)}
                            </p>
                        {/if}

                        {#if modInfo?.followers}
                            <p class="followers">
                                <i class="fa-solid fa-eye" />
                                &nbsp;&nbsp;
                                {fmt.format(modInfo?.followers)}
                            </p>
                        {/if}
                    </div>
                </div>
            </div>

            <p class="description">
                {@html handleHtml(modInfo?.desc || "Loading description...")}
            </p>
        </div>

        <div class="actions">
            {#if downloading}
                <button type="button" class="action progress">
                    <i class="icon fa-solid fa-spin fa-spinner" />
                    &nbsp; Working...
                </button>
            {:else if installed}
                <button type="button" class="action uninstall" on:click={uninstall}>
                    <i class="icon fa-regular fa-trash-can" />
                    &nbsp; Uninstall
                </button>
            {:else}
                <button type="button" class="action" on:click={install}>
                    <i class="icon fa-regular fa-circle-down" />
                    &nbsp; Install
                </button>
            {/if}

            <Dropdown {items} bind:val={selected.id} bind:valText={selected.text} thin up />

            <!-- TODO: This thing looks terrible, fix it. -->
            <!-- {#if downloading}
                <progress max={total} value={progress} class="progress-bar" />
            {/if} -->
        </div>
    </div>
{/if}

<style scoped lang="scss">
    .full-mod-container {
        width: 100%;
        height: 100%;

        background-color: var(--base-color);

        margin: 0;
        padding: 0;

        .return-container {
            position: relative;

            .return-button {
                position: absolute;
                top: 10px;
                left: 10px;
                user-select: none;
                cursor: pointer;
            }

            .return-arrow {
                position: absolute;
                user-select: none;
                top: 10px;
                left: 10px;
                width: 40px;
                height: 40px;
                display: flex;
                justify-content: center;
                align-items: center;
                border-radius: 50%;
                background-color: green;
                z-index: 2;
                cursor: pointer;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #00a000;
                }
            }

            .return-arrow i {
                font-size: 24px;
                color: white;
            }

            .return-circle {
                position: absolute;
                user-select: none;
                top: 10px;
                left: 10px;
                width: 40px;
                height: 40px;
                border-radius: 50%;
                background-color: green;
                z-index: 1;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #00a000;
                }
            }
        }

        .mod {
            width: 100%;
            height: 92%;

            overflow-y: scroll;

            .infos {
                width: 97%;
                user-select: none;
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: space-between;

                margin: 0.5rem 0;
                height: 4rem;
                padding: 0.25% 1.5%;

                .icon {
                    user-select: none;
                    height: 100%;
                    object-fit: cover;
                    object-position: center center;
                    margin-right: 1rem;
                    margin-left: 2.5rem;
                }

                .texts {
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: space-between;
                    width: 100%;

                    .left,
                    .right {
                        display: flex;
                        flex-direction: column;
                        justify-content: center;

                        & > p {
                            margin: 0.5rem;
                        }
                    }

                    .left {
                        align-items: flex-start;

                        & > a {
                            color: inherit;
                            margin: 0.5rem;
                        }

                        .author {
                            color: #bcbebc;
                        }
                    }

                    .right {
                        align-items: flex-end;

                        .downloads {
                            color: lightgreen;
                        }

                        .followers {
                            color: lightskyblue;
                        }
                    }
                }
            }

            .description {
                width: 97%;
                padding: 0 1.5%;
                margin: 0;
                margin-top: 2%;

                font-family: "Ubuntu Mono", monospace;

                :global(a) {
                    color: inherit;
                }
            }
        }

        .actions {
            width: 100%;
            height: 6%;
            margin: 0;
            padding: 1%;

            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: flex-start;

            .action {
                background-color: transparent;
                color: lightgreen;
                border: 1px solid lightgreen;
                border-radius: 8px;
                padding: 0.5% 1%;

                font-size: 12pt;
                cursor: pointer;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;

                transition:
                    color 0.5s ease,
                    background-color 0.5s ease;

                &:hover {
                    background-color: lightgreen;
                    color: black;
                }

                .icon {
                    font-size: 14pt;
                }

                &.uninstall {
                    color: #ac2c2c;
                    border-color: #ac2c2c;

                    &:hover {
                        color: black;
                        background-color: #ac2c2c;
                    }
                }

                &.progress {
                    color: #ac2cac;
                    border-color: #ac2cac;
                    cursor: progress;

                    &:hover {
                        color: #ac2cac;
                        background-color: transparent;
                    }
                }
            }

            .progress-bar {
                margin-left: 5%;
                border-radius: 20px;
                width: 30%;
                height: 12px;
            }

            .progress-bar::-webkit-progress-value {
                background-color: #2c5cba;
                border-radius: 7px;
            }

            .progress-bar::-webkit-progress-bar {
                background-color: #c5c5c5;
                border-radius: 7px;
            }
        }
    }
</style>
