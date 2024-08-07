<script lang="ts">
import ModEntry from "$components/ModEntry.svelte";
import { marked } from "marked";
import { page } from "$app/stores";
import Back from "$components/Back.svelte";
import { goto } from "$app/navigation";
import { plugins } from "$api/stores";
import { RPC, type Mod, type Instance, type ModLoader, unwrap } from "$bindings";
import { onMount } from "svelte";
import LoaderDropdown from "$components/LoaderDropdown.svelte";

let instance: Instance = null!;
let background: string | undefined;
let executable: string | undefined;
let editing = false;
let mods: Mod[] = [];
let editor: HTMLTextAreaElement | undefined;
let loader: ModLoader | undefined;
let installing = false;

$: description = instance?.description;

const id = $page.params.instance;

const refresh = async () => {
    instance = unwrap(await RPC.instance.read(Number.parseInt(id || "-1", 10)));
    mods = await RPC.mods.read(instance.id);

    background = $plugins.find((v) => v.game === instance.gameId)?.banner_url;
    executable = instance.dataDir;

    loader = instance.loader ? JSON.parse(instance.loader) : { Vanilla: "<unknown>" };
};

onMount(refresh);

const save = async () => {
    if (instance && editor && editor.value) {
        instance = unwrap(
            await RPC.instance.update({
                id: instance.id,
                description: editor.value,
            })
        );
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

    // console.log(unwrap(await commands.launchGame(instance.game_id, instance, null)));
};

const updateDescription = (ev: Event) => {
    const textarea = ev.target as HTMLTextAreaElement;

    description = textarea.value;
};

const gotoMods = () => {
    goto(`/${instance?.gameId}/mods?instance=${instance?.id}`);
};

const reinstall = async () => {
    if (instance == null) return;

    installing = true;
    // instance = unwrap(await commands.installLoader(loader!, instance, null));
    installing = false;
};
</script>

<div class="full-instance-container">
    <Back to="/{instance?.gameId}/instances" />

    <div class="instance">
        <img src={background} class="background" alt="background" />

        <div class="infos">
            <div class="left">
                <p class="name">{instance?.name}</p>
            </div>

            <div class="right">
                {#if loader && instance.gameId == 432}
                    <LoaderDropdown bind:loader on:change={reinstall} bind:loading={installing} />
                {/if}

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
                bind:this={editor}
            />
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
                <ModEntry {instance} head />

                {#each mods as mod}
                    <ModEntry {mod} {instance} on:uninstall={refresh} />
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

        background-color: var(--base-color);
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
                        font-family: Ubuntu;
                        font-size: 12pt;

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
                        font-family: Ubuntu;
                        font-size: 12pt;
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

                font-family: Ubuntu;
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
