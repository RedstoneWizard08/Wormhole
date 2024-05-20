<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { DropdownItem } from "../api/dropdown";

    export let val: string | number;
    export let valText: string;
    export let left = false;
    export let right = false;
    export let items: DropdownItem[];

    let shown = false;
    const dispatch = createEventDispatcher();

    const makeOnSelected = (value: string | number, txt: string) => {
        return () => {
            val = value;
            valText = txt;

            shown = false;
            dispatch("change");
        };
    };

    const onClick = () => {
        shown = !shown;
    };
</script>

<div class="dropdown" class:active={shown} class:left class:right>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div class="selected" class:active={shown} on:click={onClick}>
        {valText}
    </div>

    <div class="items" class:hide={!shown}>
        {#each items as item}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div class:same={val == item.id} on:click={makeOnSelected(item.id, item.text)}>
                {item.text}
            </div>
        {/each}
    </div>
</div>

<!-- svelte-ignore css-unused-selector -->
<style lang="scss">
    .dropdown {
        width: 40%;
        margin-left: 1.5%;

        user-select: none;

        position: relative;

        border: 1px solid #4c4c4a;
        border-radius: 8px;

        &.left {
            justify-self: start;
        }

        &.right {
            justify-self: end;
        }

        &.active {
            border-bottom-left-radius: 0;
            border-bottom-right-radius: 0;
        }

        .selected {
            border-radius: 8px;
            background-color: transparent;

            &::after {
                position: absolute;
                content: "";
                top: 14px;
                right: 10px;
                width: 0;
                height: 0;
                border: 6px solid transparent;
            }

            &.active {
                border-bottom-left-radius: 0;
                border-bottom-right-radius: 0;
            }

            .arrow-active::after {
                border-color: transparent transparent #fff transparent;
                top: 7px;
            }
        }

        .items div,
        .selected {
            color: #fff;
            padding: 8px 16px;
            border-color: transparent transparent rgba(0, 0, 0, 0.1) transparent;
            cursor: pointer;
        }

        .items {
            position: absolute;
            background-color: #1f2120;
            top: 100%;
            left: 0;
            right: 0;
            z-index: 99;

            border: 1px solid #4c4c4a;

            border-bottom-left-radius: 8px;
            border-bottom-right-radius: 8px;
        }

        .hide {
            display: none;
        }

        .items div:hover,
        .same {
            background-color: rgba(0, 0, 0, 0.2);
        }
    }
</style>
