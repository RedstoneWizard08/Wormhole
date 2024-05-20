// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

import { blue, cyan, green, magenta, red, yellow } from "colorette";
import puppeteer from "puppeteer";
import which from "which";

const logAllRequests = true;

const main = async () => {
    console.log(`[${cyan("INFO")}] Starting browser...`);

    const browser = await puppeteer.launch({
        product: "firefox",
        executablePath: which.sync("firefox"),
    });

    const page = await browser.newPage();

    page.on("console", (message) => {
        const type = message.type().substr(0, 3).toUpperCase();

        // eslint-disable-next-line no-unused-vars
        const colors: { [key: string]: (text: string) => string } = {
            LOG: (text) => text,
            ERR: red,
            WAR: yellow,
            INF: cyan,
        };

        const color = colors[type] || blue;

        console.log(color(`${type} ${message.text()}`));
    });

    page.on("pageerror", ({ message }) => console.log(red(message)));

    page.on(
        "response",
        (response) =>
            logAllRequests ||
            (!response.status().toString().startsWith("2") &&
                !response.status().toString().startsWith("3") &&
                console.log(green(`${response.status()} ${response.url()}`)))
    );

    page.on("requestfailed", (request) =>
        console.log(magenta(`${request.failure()?.errorText} ${request.url()}`))
    );

    await page.goto(process.argv[2]);

    await new Promise((r) => setTimeout(r, 999999999));

    await browser.close();
};

main();
