<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import PaginationButton from "./PaginationButton.svelte";

    export let pages: number;
    export let page: number;
    
    const dispatch = createEventDispatcher();

    let offset = 0;
    const offsetted = 8;

    $: _array = [...Array(offsetted + 1)];

    const setCurrent = (n: number) => {
        page = Math.max(0, n);
        offset = Math.max(0, n - 3) + offsetted > pages ? pages - offsetted : Math.max(0, n - 3);
        dispatch("change");
    };
</script>

<div class="pagination">
    <PaginationButton
        page={Math.max(0, page - 1)}
        {setCurrent}
        active={Math.max(0, page - 1) == page}
        prev />

    {#each _array as _, i}
        {#if i + offset > 0 && i + offset <= pages}
            <PaginationButton page={i + offset} {setCurrent} active={i + offset == page} />
        {/if}
    {/each}

    {#if offset + offsetted < pages}
        <PaginationButton page={-1} setCurrent={(v) => void v} active={false} ellipsis />

        <PaginationButton page={pages} {setCurrent} active={pages == page} />
    {/if}

    <PaginationButton
        page={Math.min(pages, page + 1)}
        {setCurrent}
        active={Math.min(pages, page + 1) == page}
        next />
</div>

<style lang="scss">
    .pagination {
        width: calc(100% - 1rem);
        height: 2rem;
        padding: 0.5rem;
        margin: 0.25rem 0;

        user-select: none;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;

        overflow: hidden;
    }
</style>
