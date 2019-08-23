import { App } from '../wasm_build/index';
// import {memory} from "../wasm_build/index_bg";

const app = App.new();

window.addEventListener('beforeunload', () => {
    app.free();
});

export { app };
