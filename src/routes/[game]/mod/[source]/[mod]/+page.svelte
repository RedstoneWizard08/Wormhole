<script lang="ts">
    import DOMPurify from "dompurify";
    import { page } from "$app/stores";
    import LoadingPage from "../../../../../components/LoadingPage.svelte";
    import { unwrap } from "../../../../../api/util";
    import { commands } from "../../../../../api/bindings/app";
    import type { SourceMapping, Mod } from "../../../../../api/wrap";
    import { marked } from "marked";

    const modId = $page.params.mod;
    const source = $page.params.source;
    const gameId = parseInt($page.params.game);

    let modInfo: Mod | null = null;
    let isLoading = true;
    let mods = false;

    $: {
        if (modId || $page.url) {
            mods = /\/mods?(\/\d+)?/i.test($page.url.pathname);

            (async () => {
                modInfo = unwrap(
                    await commands.getMod(gameId, source as SourceMapping, modId, null)
                );

                isLoading = false;
            })();
        }
    }

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

        images.forEach((img) => {
            img.style.maxWidth = "50%";
        });

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

        return html;
    };

    const install = async () => {
        // TODO
    };
</script>

{#if isLoading}
    <LoadingPage />
{:else}
    <div class="full-mod-container">
        <a class={`link ${mods ? "active" : ""}`} href={`/${gameId}/mods`}>
            <div class="return-container">
                <div class="return-arrow">
                    <i class="fa-solid fa-long-arrow-left" />
                </div>
                <div class="return-circle" />
            </div>
        </a>

        <div class="mod">
            {#if modInfo?.banner}
                <!-- svelte-ignore a11y-img-redundant-alt -->
                <img src={modInfo?.banner} class="background" alt="mod-background-image" />
            {/if}

            <div class="infos">
                <div class="left">
                    <p class="name">{modInfo?.name}</p>

                    {#if modInfo?.author}
                        &bull;
                        <p class="author">{modInfo?.author}</p>
                    {/if}
                </div>

                <div class="right">
                    {#if modInfo?.downloads}
                        <p class="downloads">
                            <i class="fa-solid fa-circle-down" />
                            &nbsp;&nbsp;
                            {modInfo?.downloads}
                        </p>
                    {/if}

                    {#if modInfo?.followers}
                        <p class="followers">
                            <i class="fa-solid fa-eye" />
                            &nbsp;&nbsp;
                            {modInfo?.followers}
                        </p>
                    {/if}
                </div>
            </div>

            <p class="description">
                {@html handleHtml(modInfo?.desc || "Loading description...")}
            </p>
        </div>

        <div class="actions">
            <button type="button" class="action" on:click={install}>
                <i class="icon fa-regular fa-circle-down" />
                &nbsp; Install
            </button>
        </div>
    </div>
{/if}

<style lang="scss">
    .full-mod-container {
        width: 100%;
        height: 100%;

        background-color: #1f2120;

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
            }
        }

        .mod {
            width: 100%;
            height: 92%;

            overflow-y: scroll;

            .background {
                user-select: none;
                width: 100%;
                height: 70%;
                object-fit: cover;
                object-position: center center;
            }

            .infos {
                width: 97%;
                user-select: none;
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: space-between;

                padding: 0.25% 1.5%;

                .left,
                .right {
                    display: flex;
                    flex-direction: row;
                    align-items: center;
                }

                .left {
                    width: 50%;
                    justify-content: flex-start;

                    .name {
                        margin-right: 2%;
                    }

                    .author {
                        margin-left: 2%;
                        color: #bcbebc;
                    }
                }

                .right {
                    width: 50%;
                    justify-content: flex-end;

                    .downloads {
                        color: lightgreen;
                        margin-right: 4%;
                    }

                    .followers {
                        color: lightskyblue;
                    }
                }
            }

            .description {
                width: 97%;
                padding: 0 1.5%;
                margin: 0;

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

            .action {
                background-color: transparent;
                color: lightgreen;
                border: 1px solid lightgreen;
                border-radius: 8px;
                padding: 0.5% 1%;

                font-family: "manteka", serif;
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
            }
        }
    }
</style>
