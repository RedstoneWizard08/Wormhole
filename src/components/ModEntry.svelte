<script lang="ts">
import { formatBytes, unwrap } from "$api/util";
import { createEventDispatcher, onMount } from "svelte";
import { commands, type DbMod, type Instance } from "$api/bindings/app";
import Delete from "./Delete.svelte";

export let mod: DbMod | null = null;
export let instance: Instance;
export let head = false;

let source: string | null = null;

const disp = createEventDispatcher();

onMount(async () => {
    source = unwrap(await commands.getSourceId(mod?.source_id!, null));
});

const uninstall = async () => {
    unwrap(await commands.uninstallMod(instance.game_id, mod!, instance, null));

    disp("uninstall");
};
</script>

<tr class="mod" class:head-item={head}>
    {#if head}
        <td class="name">Mod Name</td>
        <td class="file">File Name</td>
        <td class="size">File Size</td>
        <td class="source">Source</td>
        <td class="actions" />
    {:else}<td class="name">{mod?.name}</td>
        <td class="file">{mod?.file_name}</td>
        <td class="size">{formatBytes(mod?.size || 0)}</td>
        <td class="source">{source}</td>

        <td class="actions">
            <Delete action={uninstall} clazz="__workaround__action" />
        </td>
    {/if}
</tr>

<style lang="scss">
    .mod {
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
            width: 40%;
        }

        .size {
            width: calc(15% - 2rem);
        }

        .source {
            width: 10%;
        }

        .actions {
            width: 2rem;
        }
    }
</style>
