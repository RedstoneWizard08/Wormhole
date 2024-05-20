<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    import PaginationButton from "./PaginationButton.svelte";

    export let pages: number;
    export let page: number;

    let offset = 0;
    const offsetted = 8;

    $: _array = [...Array(offsetted + 1)];

    const setCurrent = (n: number) => {
        page = Math.max(0, n);
        offset = Math.max(0, n - 3) + offsetted > pages ? pages - offsetted : Math.max(0, n - 3);
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
