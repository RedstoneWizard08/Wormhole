<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    import InstanceCard from "../../../components/InstanceCard.svelte";
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import { commands, type Dirs, type Instance, type PluginInfo } from "../../../api/bindings/app";
    import { unwrap } from "../../../api/util";

    let adding = false;
    let deleteing = false;

    let instances: Instance[] = [];
    let current: Instance | null = null;
    let dirs: Dirs | null = null;
    let info: PluginInfo | null = null;

    let gameId = parseInt($page.params.game);

    let name = "";
    $: path = dirs?.data + "/" + info?.display_name + "/" + name;

    onMount(async () => {
        instances = unwrap(await commands.getInstances(gameId, null));
        dirs = unwrap(await commands.getDirs(null));
        info = unwrap(await commands.info(gameId, null));
    });

    const addInstance = async () => {
        unwrap(await commands.addInstance({
            id: null,
            name: name,
            cache_dir: dirs?.cache + "/" + info?.display_name,
            game_id: gameId,
            created: new Date().getUTCDate(),
            data_dir: path,
            description: "",
            install_dir: dirs?.data + "/" + info?.display_name,
            updated: new Date().getUTCDate(),
        }, null));

        adding = false;

        instances = unwrap(await commands.getInstances(gameId, null));
    };

    const deleteInstance = async () => {
        commands.deleteInstance(current?.id!, null);

        deleteing = !deleteing;
    };

    const toggleAdding = () => {
        adding = !adding;
    };

    const toggleDeleteing = () => {
        deleteing = !deleteing;
    };
</script>

{#if adding}
    <div class="add-modal-background">
        <div class="add-modal">
            <div class="modal-header">
                <span class="title">Add Instance</span>

                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <i class="fa-solid fa-times close" on:click={toggleAdding} />
            </div>

            <input type="text" placeholder="Instance name" class="name" bind:value={name} />

            <p>Will be created in: {path}</p>

            <button type="button" class="submit-button" on:click={addInstance}> Continue </button>
        </div>
    </div>
{/if}

{#if deleteing}
    <div class="add-modal-background">
        <div class="add-modal">
            <div class="modal-header">
                <span class="title">Delete Instance</span>

                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <i class="fa-solid fa-times close" on:click={toggleDeleteing} />
            </div>

            <p class="delete-text">
                Are you sure you want to delete the instance "{current?.name}"?
            </p>

            <button type="button" class="yes-button" on:click={deleteInstance}> Yes </button>

            <button type="button" class="no-button" on:click={() => (deleteing = !deleteing)}>
                No
            </button>
        </div>
    </div>
{/if}

<div class="instances-wrapper">
    <button class="add-instance-button" on:click={() => (adding = !adding)}>
        <i class="fa-solid fa-plus" />
    </button>

    <div class="instances-container">
        {#if Array.isArray(instances)}
            {#each instances as info}
                <InstanceCard data={info} bind:deleteing bind:current />
            {/each}
        {/if}
    </div>
</div>

<!-- svelte-ignore css-unused-selector -->
<style scoped lang="scss">
    .instances-wrapper {
        width: 100%;
        height: 100%;

        background-color: #1f2120;

        margin: 0;
        padding: 0;

        position: relative;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        .add-instance-button {
            width: 2.5rem;
            height: 2.5rem;

            position: absolute;

            background-color: transparent;
            color: #4edf4e;
            border-radius: 8px;
            padding: 8px;
            font-size: 16px;
            cursor: pointer;

            transition:
                background-color 0.5s ease,
                color 0.5s ease,
                border-color 0.5s ease;

            border: 1px solid #4c4c4c;
            outline: none;

            left: calc(100% - 3rem);
            top: calc(100% - 3rem);

            z-index: 4;

            font-family: "manteka", serif;

            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: center;

            i,
            i::after {
                text-align: center;
            }

            &:hover {
                border-color: #1e7f1e;
                background-color: #1e7f1e;
            }
        }

        .instances-container {
            width: 100%;
            height: 100%;

            margin: 0 1% 0 2.5%;
            padding: 0;

            overflow-y: scroll;

            display: grid;
            grid-template-columns: repeat(5, 18.5%);
            grid-column-gap: 1.5%;
            grid-template-rows: auto auto 1fr 1fr 1fr auto auto;

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
    }

    .add-modal-background {
        width: 100%;
        height: 100%;

        margin: 0;
        padding: 0;

        position: absolute;
        z-index: 300;

        background-color: rgba(0, 0, 0, 0.6);

        display: flex;
        flex-direction: row;
        align-items: flex-start;
        justify-content: center;

        .add-modal {
            width: 50%;
            height: 50%;

            margin: 0;
            padding: 0;

            margin-top: 10%;
            border-radius: 8px;

            background-color: #2f3130;

            .modal-header {
                border-radius: 8px 8px 0 0;

                width: 98.3%;
                height: 10%;

                padding: 0 1.5%;
                padding-right: 0;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: space-between;

                border: 1px solid #3f4140;
                background-color: #3f4140;

                .close {
                    cursor: pointer;

                    height: 100%;
                    width: 6%;

                    display: flex;
                    flex-direction: row;
                    align-items: center;
                    justify-content: center;

                    background-color: transparent;
                    border-radius: 3px;

                    transition: background-color 0.5s ease;

                    &:hover {
                        background-color: #4f5150;
                    }
                }
            }

            .dropdown {
                margin-top: 1.5%;
            }

            .name {
                margin: 1.5%;

                width: 95.5%;
                height: 8%;

                text-indent: 6px;
                font-family: Ubuntu;
                font-size: 11pt;
                background-color: transparent;

                border: 2px solid #3f4140;
                border-radius: 8px;

                color: white;
                outline: none;

                &::placeholder {
                    font-size: 10pt;
                    font-family: "manteka", serif;
                    text-transform: uppercase;
                }
            }

            .select-dir {
                margin: 0 1.5%;
                width: 95.5%;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: flex-start;

                .select-dir-button {
                    margin: 0;
                    padding: 1.25% 1.5%;

                    color: white;
                    font-family: "manteka", serif;
                    text-transform: uppercase;
                    background-color: transparent;

                    border: 2px solid #3f4140;
                    border-radius: 8px;
                    cursor: pointer;
                    outline: none;

                    transition: background-color 0.5s ease;

                    &:hover {
                        background-color: #4f5150;
                    }
                }

                .select-dir-text {
                    margin: 0;
                    padding: 0;

                    margin-left: 2%;
                }
            }

            .submit-button {
                margin: 1.5%;
                padding: 1.25% 1.5%;

                color: white;
                font-family: "manteka", serif;
                text-transform: uppercase;
                background-color: transparent;

                border: 2px solid #3f4140;
                border-radius: 8px;
                cursor: pointer;
                outline: none;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #4f5150;
                }
            }

            .yes-button {
                margin: 0.5% 1.5%;
                padding: 1.25% 1.5%;

                color: white;
                font-family: "manteka", serif;
                text-transform: uppercase;
                background-color: #1c7c1c;

                border: 2px solid #1cac1c;
                border-radius: 8px;
                cursor: pointer;
                outline: none;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #1cac1c;
                }
            }

            .no-button {
                margin: 0.5% 0.5%;
                padding: 1.25% 2%;

                color: white;
                font-family: "manteka", serif;
                text-transform: uppercase;
                background-color: #9c1c1c;

                border: 2px solid #cc1c1c;
                border-radius: 8px;
                cursor: pointer;
                outline: none;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #cc1c1c;
                }
            }

            .delete-text {
                margin: 1.5%;
                font-family: Ubuntu;
            }
        }
    }
</style>
