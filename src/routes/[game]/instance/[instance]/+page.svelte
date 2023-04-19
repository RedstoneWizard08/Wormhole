<script lang="ts">
    import { type InstanceInfo, KSPGame } from "../../../../api/instance";
    import { invoke_proxy } from "../../../../api/invoke";
    import ksp1logo from "../../../../assets/ksp.png";
    import ksp2logo from "../../../../assets/ksp2.png";
    import { marked } from "marked";
    import { page } from "$app/stores";

    let instances = false;
    let instanceInfo: InstanceInfo | null = null as InstanceInfo | null;
    let background: string | null = null;
    let executable: string | null = null;
    let editing = false;

    let editor: HTMLTextAreaElement | undefined;

    $: description = instanceInfo?.description;
    $: instances = /\/instances?(\/\d+)?/i.test($page.url.pathname);

    const id = $page.params.instance;

    $: (async () => {
        const info = await invoke_proxy("get_instance_info", {
            instanceId: parseInt(id || "-1", 10),
        });

        instanceInfo = info;

        background =
            info.game == KSPGame.KSP2 || info.game.toString() == "KSP2" ? ksp2logo : ksp1logo;

        executable = info.install_path;
    })();

    const save = async () => {
        if (instanceInfo) {
            instanceInfo = {
                ...instanceInfo,

                description: editor?.value || instanceInfo.description,
            };

            await invoke_proxy("update_description", {
                instanceId: instanceInfo.id,
                description: (editor?.value || instanceInfo.description)!,
            });
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
        if (instanceInfo)
            await invoke_proxy("launch", {
                instanceId: instanceInfo.id,
            });
    };

    const updateDescription = (ev: Event) => {
        const textarea = ev.target as HTMLTextAreaElement;

        description = textarea.value;
    };
</script>

<div class="full-instance-container">
    <a class={`link ${instances ? "active" : ""}`} href="/{instanceInfo?.game}/instances">
        <div class="return-container">
            <div class="return-arrow">
                <i class="fa-solid fa-long-arrow-left" />
            </div>
            <div class="return-circle" />
        </div>
    </a>

    <div class="instance">
        <img src={background} class="background" alt="background" />

        <div class="infos">
            <div class="left">
                <p class="name">{instanceInfo?.name}</p>
            </div>

            <div class="right">
                <p class="time">
                    <i class="fa-solid fa-clock" />
                    &nbsp;&nbsp;
                    {instanceInfo?.time_played || "0 minutes"}
                </p>

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
                {@html marked(instanceInfo?.description || "", {
                    sanitize: true,
                })}
            </p>
        {/if}
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

        .return-container {
            position: relative;

            .return-button {
                position: absolute;
                top: 10px;
                left: 10px;
                user-select: none;
                cursor: pointer;
            }

            .return-arrow {
                position: absolute;
                user-select: none;
                top: 10px;
                left: 10px;
                width: 40px;
                height: 40px;
                display: flex;
                justify-content: center;
                align-items: center;
                border-radius: 50%;
                background-color: #008000;
                z-index: 2;
                cursor: pointer;

                transition: background-color 0.5s ease;

                &:hover {
                    background-color: #00a000;
                }

                i {
                    font-size: 24px;
                    color: white;
                }
            }

            .return-circle {
                position: absolute;
                user-select: none;
                top: 10px;
                left: 10px;
                width: 40px;
                height: 40px;
                border-radius: 50%;
                background-color: green;
                z-index: 1;
            }
        }

        .instance {
            width: 100%;
            height: 90%;

            overflow-y: scroll;

            .background {
                width: 90%;
                height: 56%;
                object-fit: contain;
                object-position: center;

                border-bottom: 1px solid white;

                padding-bottom: 4%;
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

                    .time {
                        color: lightgreen;
                        margin-right: 4%;
                    }

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

                        transition: color 0.5s ease, background-color 0.5s ease;

                        &:hover {
                            color: black;
                            background-color: lightskyblue;
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

                transition: color 0.5s ease, background-color 0.5s ease;

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
