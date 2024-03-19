<script lang="ts">
    import type { BrowseModInfo, ModWithDistance } from "../../../api/models/modinfo/browse";
    import { invoke_proxy } from "../../../api/invoke";
    import Dropdown from "../../../components/Dropdown.svelte";
    import SearchBar from "../../../components/SearchBar.svelte";
    import LoadingPage from "../../../components/LoadingPage.svelte";
    import Pagination from "../../../components/Pagination.svelte";
    import Mod from "../../../components/Mod.svelte";
    import { page } from "$app/stores";
    import type { DropdownItem } from "../../../api/dropdown";
    import { gameItems } from "../../../api/browse";

    let results: BrowseModInfo[] = [];
    let perPage = 30;
    let pages = 1;
    let pageId = 1;
    let loading = true;
    let initialLoad = true;

    let gameId = parseInt($page.params.game);

    let instance = -1;
    let instanceText = "Unknown";

    let instances: DropdownItem[] = [];

    $: (async () => {
        await invoke_proxy("set_active_instance", {
            instanceId: instance,
        });
    })();

    $: (async () => {
        instance = -1;
        instanceText = "Unknown";

        loading = true;
        pages = 0;

        const data = await invoke_proxy("get_mods", {
            gameId,
            count: perPage,
            page: pageId,
        });

        results = data.result;
        pages = data.pages;

        if (pageId > data.pages) pageId = data.pages - 1;

        const defaultInstance = await invoke_proxy("get_active_instance", {
            gameId,
        });

        if (defaultInstance) {
            instance = defaultInstance.id;
            instanceText = defaultInstance.name;
        }

        if (initialLoad) {
            instances = (
                await invoke_proxy("get_instances", {
                    gameId,
                })
            ).map((instance) => ({
                id: instance.id,
                text: instance.name,
            }));

            initialLoad = false;
        }

        loading = false;
    })();

    async function searchMods(mods: BrowseModInfo[], query: string): Promise<BrowseModInfo[]> {
        const exactMatches: BrowseModInfo[] = [];
        const closeMatches: ModWithDistance[] = [];

        const regex = new RegExp(`^${query}`, "i");

        for (const mod of mods) {
            if (mod.name.toLowerCase() === query.toLowerCase() || regex.test(mod.name)) {
                exactMatches.push(mod);
            } else {
                const dist = await invoke_proxy("get_distance", {
                    query,
                    modName: mod.name,
                });
                closeMatches.push({ mod, dist });
            }
        }

        closeMatches.sort((a, b) => a.dist! - b.dist!);

        return exactMatches.concat(closeMatches.map((m) => m.mod));
    }

    const handleSearch = async (query: string) => {
        console.log("Searching for:", query);

        const matches = await searchMods(results, query);

        console.log(matches);

        results = matches;
    };
</script>

<div class="browse-container">
    <div class="top">
        <span></span>

        <div class="search-bar">
            <SearchBar onSearch={handleSearch} />
        </div>

        <Dropdown bind:val={instance} bind:valText={instanceText} items={instances} right />
    </div>

    {#if loading}
        <LoadingPage />
    {:else}
        <div class="grid">
            {#each results as mod}
                <Mod {mod} />
            {/each}
        </div>
    {/if}

    {#if !initialLoad && pages > 0}
        <Pagination {pages} bind:page={pageId} />
    {/if}
</div>

<style lang="scss">
    .browse-container {
        width: 100%;
        height: 93%;

        background-color: #1f2120;
        padding: 0;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-start;

        .grid {
            height: calc(98% - 3rem);
            width: 100%;
            margin-left: 1.75%;

            display: grid;
            grid-template-columns: repeat(5, 18.5%);
            grid-column-gap: 1.5%;
            grid-template-rows: auto auto 1fr 1fr 1fr auto auto;

            overflow-y: scroll;
        }

        @media screen and (min-width: 1100px) {
            .grid {
                grid-template-columns: repeat(5, 18.25%);
                grid-column-gap: 2%;
            }
        }

        @media screen and (max-width: 1099px) {
            .grid {
                grid-template-columns: repeat(4, 23%);
                grid-column-gap: 2%;
            }
        }

        @media screen and (max-width: 900px) {
            .grid {
                grid-template-columns: repeat(3, 31%);
                grid-column-gap: 3%;
            }
        }

        @media screen and (max-width: 600px) {
            .grid {
                grid-template-columns: repeat(2, 48%);
                grid-column-gap: 4%;
            }
        }

        @media screen and (max-width: 400px) {
            .grid {
                grid-template-columns: repeat(1, 96%);
                grid-column-gap: 4%;
            }
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
