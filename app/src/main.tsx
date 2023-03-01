import "./style.scss";

import { render } from "preact";
import { App } from "./app";

const root = document.getElementById("root")!;

render(<App />, root);
