<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    import { marked } from "marked";
    import { page } from "$app/stores";
    import { demoMods, type InstalledMod } from "../../../../api/models/mod";
    import { formatBytes, unwrap } from "../../../../api/util";
    import Back from "../../../../components/Back.svelte";
    import Delete from "../../../../components/Delete.svelte";
    import { goto } from "$app/navigation";
    import { plugins } from "../../../../api/stores";
    import { commands, type Instance } from "../../../../api/bindings/app";

    let instance: Instance | undefined = undefined;
    let background: string | undefined = undefined;
    let executable: string | undefined = undefined;
    let editing = false;
    let mods: InstalledMod[] = demoMods;
    let editor: HTMLTextAreaElement | undefined;

    $: description = instance?.description;

    const id = $page.params.instance;

    $: (async () => {
        const info = unwrap(await commands.getInstance(parseInt(id || "-1", 10), null));

        instance = info;

        background = $plugins.find((v) => v.game == info.game_id)?.banner_url;
        executable = info.data_dir;
    })();

    const save = async () => {
        if (instance) {
            instance = {
                ...instance,

                description: editor?.value || instance.description,
            };

            // TODO: updateInstance function
        }

        editing = false;
    };

    const edit = () => {
        editing = true;

        setTimeout(() => {
            editor?.focus();
        });
    };

    const launch = async () => {
        if (instance == null) return;

        unwrap(await commands.launchGame(instance.game_id, instance, null));
    };

    const updateDescription = (ev: Event) => {
        const textarea = ev.target as HTMLTextAreaElement;

        description = textarea.value;
    };

    const gotoMods = () => {
        // TODO

        goto(`/${instance?.game_id}/mods`);
    };
</script>

<div class="full-instance-container">
    <Back to="/{instance?.game_id}/instances" />

    <div class="instance">
        <img src={background} class="background" alt="background" />

        <div class="infos">
            <div class="left">
                <p class="name">{instance?.name}</p>
            </div>

            <div class="right">
                {#if editing}
                    <button type="button" class="edit" on:click={save}>
                        <i class="fa-solid fa-save" />
                        &nbsp; Save
                    </button>
                {:else}
                    <button type="button" class="edit" on:click={edit}>
                        <i class="fa-solid fa-pencil" />
                        &nbsp; Edit
                    </button>
                {/if}

                <button type="button" class="add" on:click={gotoMods}>
                    <i class="fa-solid fa-plus" />
                    &nbsp; Add Mods
                </button>
            </div>
        </div>

        {#if editing}
            <textarea
                class="editor"
                value={description}
                on:input={updateDescription}
                on:keydown={updateDescription}
                on:change={updateDescription}
                bind:this={editor} />
        {:else}
            <p class="description">
                {@html marked(instance?.description || "")}
            </p>
        {/if}

        <div class="mods">
            <div class="head">
                <p class="title">{mods.length} mods installed</p>
            </div>

            <table class="items">
                <tr class="item head-item">
                    <td class="name">Mod Name</td>
                    <td class="file">File Name</td>
                    <td class="size">File Size</td>
                    <td class="actions" />
                </tr>

                {#each mods as mod}
                    <tr class="item">
                        <td class="name">{mod.name}</td>
                        <td class="file">{mod.file}</td>
                        <td class="size">{formatBytes(mod.size)}</td>

                        <td class="actions">
                            <Delete action={() => {}} clazz="__workaround__action" />
                        </td>
                    </tr>
                {/each}
            </table>
        </div>
    </div>

    <div class="actions">
        <button type="button" class="action" on:click={launch}>
            <i class="icon fa-solid fa-rocket" />
            &nbsp; Launch
        </button>

        <p class="executable">{executable}</p>
    </div>
</div>

<style lang="scss">
    .full-instance-container {
        width: 100%;
        height: 100%;

        background-color: #1f2120;

        margin: 0;
        padding: 0;

        .instance {
            width: 100%;
            height: 90%;

            overflow-x: hidden;
            overflow-y: scroll;

            .background {
                width: 90%;
                height: 56%;
                object-fit: contain;
                object-position: center;

                padding-left: 3.5%;
                padding-right: 3.5%;

                margin: 0 1.5%;
            }

            .infos {
                width: 97%;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: space-between;

                padding: 0.25% 1.5%;
                border-bottom: 1px solid white;
                margin-bottom: 2%;

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
                }

                .right {
                    width: 50%;
                    justify-content: flex-end;

                    .edit {
                        color: lightskyblue;
                        background-color: transparent;

                        border: 1px solid lightskyblue;
                        padding: 1% 1.5%;
                        font-size: 12pt;
                        font-family: "manteka", serif;

                        border-radius: 8px;

                        display: flex;
                        flex-direction: row;
                        align-items: center;
                        justify-content: center;

                        cursor: pointer;
                        outline: none;
                        transition:
                            color 0.5s ease,
                            background-color 0.5s ease;

                        &:hover {
                            color: black;
                            background-color: lightskyblue;
                        }
                    }

                    .add {
                        color: lightsalmon;
                        background-color: transparent;

                        border: 1px solid lightsalmon;
                        padding: 1% 1.5%;
                        font-size: 12pt;
                        font-family: "manteka", serif;
                        margin-left: 3%;

                        border-radius: 8px;

                        display: flex;
                        flex-direction: row;
                        align-items: center;
                        justify-content: center;

                        cursor: pointer;
                        outline: none;
                        transition:
                            color 0.5s ease,
                            background-color 0.5s ease;

                        &:hover {
                            color: black;
                            background-color: lightsalmon;
                        }
                    }
                }
            }

            .description {
                width: 97%;
                padding: 0 1.5%;
                margin: 0;

                font-family: "Ubuntu Mono", monospace;
            }

            .editor {
                width: 97%;
                height: 20rem;

                margin: 0 1.5%;

                font-family: "Ubuntu Mono", monospace;

                background-color: transparent;
                border: 1px solid white;
                border-radius: 8px;

                color: white;
                font-size: 14pt;
            }

            .mods {
                display: flex;
                flex-direction: column;
                align-items: flex-start;
                justify-content: center;
                width: calc(100% - 5.5rem);
                margin: 0.25rem 2rem;
                padding: 0.25rem 0.75rem;
                border: 1px solid white;

                .items {
                    display: flex;
                    flex-direction: column;
                    align-items: flex-start;
                    justify-content: center;
                    width: 100%;

                    .item {
                        display: flex;
                        flex-direction: row;
                        align-items: center;
                        justify-content: flex-start;

                        width: 100%;
                        margin: 0.5rem 0;
                        font-size: 12pt;

                        &.head-item {
                            padding-bottom: 0.75rem;
                            border-bottom: 1px solid white;
                        }

                        td {
                            margin: 0;
                            padding: 0;
                            text-align: left;

                            display: flex;
                            flex-direction: row;
                            align-items: center;
                            justify-content: flex-start;
                        }

                        .name {
                            width: 35%;
                        }

                        .file {
                            width: 50%;
                        }

                        .size {
                            width: calc(15% - 2rem);
                        }

                        .actions {
                            width: 2rem;
                        }
                    }
                }
            }
        }

        .actions {
            width: 98%;
            height: 8%;

            padding: 1%;

            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: flex-start;

            .action {
                height: 60%;

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

            .executable {
                margin-left: 1.5%;
                font-size: 12pt;

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;

                height: 100%;
            }
        }
    }
</style>
