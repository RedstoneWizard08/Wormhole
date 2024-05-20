<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    export let page: number;
    export let setCurrent: (n: number) => void;
    export let active: boolean;
    export let next = false;
    export let prev = false;
    export let ellipsis = false;

    const clickHandler = () => {
        if (active) return;
        setCurrent(page);
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
    class="pagination-button"
    class:active={active && !next && !prev}
    class:utility={next || prev}
    class:ellipsis
    on:click={clickHandler}>
    {#if next}
        <i class="fa-solid fa-caret-right" />
    {:else if prev}
        <i class="fa-solid fa-caret-left" />
    {:else if ellipsis}
        <i class="fa-solid fa-ellipsis" />
    {:else}
        {page}
    {/if}
</div>

<style lang="scss">
    .pagination-button {
        height: 2.25rem;
        width: 2.25rem;

        margin: -0.5rem 0.3rem 0;

        text-align: center;
        border-radius: 50%;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;

        cursor: pointer;

        background-color: #5c5f5c;

        transition: background-color 0.5s ease;

        &:hover {
            background-color: #7c7f7c;
        }

        &.active {
            background-color: #9c9f9c;
        }

        &.utility {
            background-color: transparent;

            &:hover {
                background-color: #3c3f3c;
            }
        }

        &.ellipsis {
            background-color: transparent;
            cursor: default;
        }
    }
</style>
