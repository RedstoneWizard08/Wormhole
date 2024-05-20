<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    import { goto } from "$app/navigation";
    import Delete from "./Delete.svelte";
    import { plugins } from "../api/stores";
    import { commands, type Instance } from "../api/bindings/app";
    import { unwrap } from "../api/util";

    export let data: Instance;
    // This is just the current instance in the parent
    export let current: Instance | null;
    export let deleteing: boolean;

    const clicked = () => {
        goto(`/${data.game_id}/instance/${data.id}`);
    };

    const doLaunch = async (e: MouseEvent) => {
        e.stopPropagation();
        
        unwrap(await commands.launchGame(data.game_id, data, null));
    };

    const doDelete = async (e: MouseEvent) => {
        e.stopPropagation();

        if (deleteing) return;

        current = data;
        deleteing = true;

        unwrap(await commands.deleteInstance(data.id!, null));

        deleteing = false;
    };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="instance-container" on:click={clicked}>
    <img src={$plugins.find((v) => v.game == data.game_id)?.banner_url} class="banner" alt={"background"} />

    <p class="name">{data.name}</p>

    <div class="buttons">
        <button type="button" class="__workaround__action launch" on:click={doLaunch}>
            <i class="icon fa-solid fa-play" />
        </button>

        <Delete action={doDelete} clazz="__workaround__action" />
    </div>
</div>

<style lang="scss">
    .instance-container {
        display: flex;
        align-items: center;
        justify-content: space-between;
        flex-direction: column;

        user-select: none;

        width: 100%;
        height: auto;

        margin: 5% 0;

        border-radius: 8px;
        background-color: #3f4140;

        cursor: pointer;
        transition: background-color 0.5s ease;

        .buttons {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: flex-end;

            width: 100%;
            height: 18%;

            opacity: 0;

            transition: opacity 0.5s ease;

            .launch {
                background-color: #2c8c2c;
                border: none;
                color: white;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #2cac2c;
                }
            }
        }

        &:hover {
            background-color: #4f5150;

            .buttons {
                opacity: 1;
            }
        }

        .banner {
            width: 100%;
            height: 50%;
            margin: 5% 0;
            object-fit: contain;
        }

        .name {
            font-size: 12pt;
            margin: 2% 0;
            text-align: center;
        }
    }
</style>
