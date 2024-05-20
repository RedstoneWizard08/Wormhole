<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    export let onSearch: (query: string) => void;

    const handleInput = (event: Event) => {
        const target = event.target as HTMLInputElement;

        onSearch(target.value);
    };
</script>

<div class="searchbar">
    <input type="text" placeholder="Search" on:input={handleInput} />
</div>

<style lang="scss">
    .searchbar {
        width: 100%;
        height: 100%;

        user-select: none;

        input {
            width: 100%;
            height: 100%;

            border: none;
            font-family: "manteka", serif;
            text-align: center;
            background-color: transparent;
            color: #ffffff;
            outline: none;
        }
    }
</style>
