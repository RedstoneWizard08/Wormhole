import "./style.scss";

import "@fortawesome/fontawesome-free/scss/fontawesome.scss";
import "@fortawesome/fontawesome-free/scss/regular.scss";
import "@fortawesome/fontawesome-free/scss/solid.scss";
import "@fortawesome/fontawesome-free/scss/brands.scss";

import { render } from "preact";
import { App } from "./app";
import { createMockAPI } from "./dev";
import eruda from "eruda";

const root = document.getElementById("root")!;

if (import.meta.env.TAURI_WEB_DEV) {
    eruda.init();
    createMockAPI();
}

render(<App />, root);
