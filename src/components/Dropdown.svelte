<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    import type { DropdownItem } from "../api/dropdown";

    export let val: string | number;
    export let valText: string;
    export let left = false;
    export let right = false;
    export let items: DropdownItem[];

    let shown = false;

    const makeOnSelected = (value: string | number, txt: string) => {
        return () => {
            val = value;
            valText = txt;

            shown = false;
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
