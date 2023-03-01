import "./style.scss";

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
