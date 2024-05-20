<!--
MIT License

Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. -->

<script lang="ts">
    export let to: string;
</script>

<a href={to}>
    <div class="return-container">
        <div class="return-arrow">
            <i class="fa-solid fa-long-arrow-left" />
        </div>
        <div class="return-circle" />
    </div>
</a>

<style scoped lang="scss">
    .return-container {
        position: relative;

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
</style>
