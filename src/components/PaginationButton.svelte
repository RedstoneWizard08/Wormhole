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
