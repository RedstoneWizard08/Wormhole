<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    import { goto } from "$app/navigation";
    import type { Mod } from "../api/wrap";

    export let mod: Mod;
    export let game: number;

    let installed = false;
    let installing = false;

    const capText = (text: string, size: number) => {
        if (text.length > size) return `${text.substring(0, size - 3)}...`;

        return text;
    };

    const onDownload = async (ev: MouseEvent) => {
        ev.preventDefault();
        ev.stopPropagation();

        installing = true;

        // TODO: Install mod

        installing = false;
    };

    const isInstalled = () => {
        return false;
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="mod-tile" on:click={() => goto(`/${game}/mod/${mod.id}`)}>
    <!-- svelte-ignore a11y-img-redundant-alt -->
    <!-- <img src={mod.background} class="image" alt="mod-background image" /> -->

    <div class="info">
        <p class="title">{capText(mod.name, 22)}</p>

        <button type="button" class="action" on:click={onDownload}>
            {#if installing}
                <i class="icon fa-solid fa-spinner fa-spin" />
            {:else if isInstalled()}
                <i class="icon fa-solid fa-trash-can" />
            {:else}
                <i class="icon fa-solid fa-circle-down" />
            {/if}
        </button>
    </div>
</div>

<style lang="scss">
    .mod-tile {
        width: 96%;
        height: 90%;

        user-select: none;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        padding: 2%;
        border-radius: 4px;

        transition: background-color 0.5s ease;
        cursor: pointer;

        &:hover {
            background-color: #3f4140;
        }

        .image {
            margin-top: 2%;
            width: 96%;
            height: 100%;
            object-fit: cover;
            border-radius: 4px;
        }

        .info {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: space-between;

            width: 96%;
            padding: 0 2%;

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
                }
            }
        }
    }
</style>
