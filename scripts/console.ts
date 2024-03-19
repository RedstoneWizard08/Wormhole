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
