<script lang="ts">
    import SearchBar from "$components/SearchBar.svelte";
    import LoadingPage from "$components/LoadingPage.svelte";
    import Pagination from "$components/Pagination.svelte";
    import Mod from "$components/Mod.svelte";
    import { page } from "$app/stores";
    import { commands, type PluginInfo, type SourceMapping, type Mod as ModItem } from "$bindings";
    import { unwrap } from "$api/util";
    import Dropdown from "$components/Dropdown.svelte";
    import { onMount } from "svelte";
    import Back from "$components/Back.svelte";

    let results: ModItem[] = [];
    let perPage = 30;
    let pages = 0;
    let pageId = 0;
    let loading = true;
    let initialLoad = true;
    let gameId = parseInt($page.params.game);
    let sources: SourceMapping[] = [];
    let source = sources[0];
    let last: string | null = null;
    let instanceId = parseInt($page.url.searchParams.get("instance") || "-1");

    onMount(async () => {
        loading = true;
        pages = 0;

        sources = unwrap(await commands.sources(gameId, null)) as SourceMapping[];
        source = source || sources[0];

        handleSearch("");

        loading = false;
        initialLoad = false;
    });

    const handleSearch = async (query: string, force = false) => {
        if (last == query && !force) return;

        console.log("Searching for:", query);

        try {
            loading = true;

            const data = unwrap(
                await commands.searchMods(
                    gameId,
                    source,
                    query,
                    { page: pageId, count: perPage },
                    null
                )
            );

            console.log("Got data:", data);

            results = data.data;
            pageId = data.page || pageId;
            perPage = data.per_page || perPage;
            pages = data.pages || pages;

            if (pageId > pages) pageId = Math.max(pages - 1, 0);
        } catch (e) {
            console.log(e);
        }

        last = query;
        loading = false;
    };

    const onChange = async () => {
        await handleSearch(last || "", true);
    };
</script>

<div class="browse-container">
    <div class="top">
        <a class="link" href="/{gameId}/instance/{instanceId}">
            <div class="return-container">
                <div class="return-arrow">
                    <i class="fa-solid fa-long-arrow-left" />
                </div>
                <div class="return-circle" />
            </div>
        </a>

        <div class="search-bar">
            <SearchBar onSearch={handleSearch} />
        </div>

        <Dropdown
            on:change={onChange}
            bind:val={source}
            valText={source}
            items={sources.map((v) => ({ id: v, text: v }))} />
    </div>

    {#if loading}
        <LoadingPage />
    {:else}
        <div class="list">
            {#if results}
                {#each results as mod}
                    <Mod {mod} game={gameId} instance={instanceId} />
                {/each}
            {/if}
        </div>
    {/if}

    {#if !initialLoad && pages > 0}
        <Pagination {pages} bind:page={pageId} on:change={onChange} />
    {/if}
</div>

<style lang="scss">
    .browse-container {
        width: 100%;
        height: 93%;

        background-color: var(--base-color);
        padding: 0;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;

        .return-container {
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

        .list {
            height: calc(98% - 3rem);
            width: 100%;
            margin-left: 1.75%;

            display: flex;
            flex-direction: column;
            align-items: flex-start;
            justify-content: flex-start;

            overflow-y: scroll;
        }
    }

    .top {
        width: 100%;
        height: 3.5rem;
        padding: 0 1rem;

        display: grid;
        grid-column-gap: 2%;
        grid-template-rows: 100%;
        grid-template-columns: repeat(3, calc(((100% - 2rem) - 4%) / 3));

        align-items: center;
        align-content: center;

        justify-items: center;
        justify-content: center;

        .search-bar {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;

            border-radius: 0.25rem;
            overflow: hidden;

            width: 70%;
            margin: 0 0 0.75rem;
            padding: 0.5rem 0;
            margin-top: 0.5rem;

            border: 1px solid #4c4c4a;
            transition: background-color 0.5s ease;

            &:has(input:focus) {
                background-color: #4c4c4a;
            }
        }
    }
</style>
