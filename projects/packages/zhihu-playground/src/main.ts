import "./style.css";
import { renderPmFrame } from "./lib/packing-frame";

const app = document.querySelector("#app");
if (!(app instanceof HTMLElement)) {
    throw new Error("missing #app");
}

app.innerHTML = `
<main class="layout">
    <header>
        <h1>SXO Packing</h1>
        <p class="subtitle">史瓦西度规 · CPU 测地线（赤道薄盘） · UCF 运行时 · L2/L0 呈现</p>
    </header>
    <section class="stage">
        <canvas id="pm-view" width="640" height="360"></canvas>
    </section>
    <section class="controls">
        <label>
            质量 M
            <input id="mass-range" type="range" min="0.5" max="3" step="0.1" value="1" />
            <span id="mass-label">1.0</span>
        </label>
        <label>
            宽度
            <input id="width-input" type="number" min="320" max="960" step="80" value="640" />
        </label>
        <label>
            高度
            <input id="height-input" type="number" min="180" max="540" step="45" value="360" />
        </label>
        <button id="render-btn" type="button">渲染一帧</button>
    </section>
    <footer class="status" id="status">点击「渲染一帧」开始</footer>
</main>`;

const canvas = document.querySelector("#pm-view");
const massRange = document.querySelector("#mass-range");
const massLabel = document.querySelector("#mass-label");
const widthInput = document.querySelector("#width-input");
const heightInput = document.querySelector("#height-input");
const renderBtn = document.querySelector("#render-btn");
const statusEl = document.querySelector("#status");

if (
    !(canvas instanceof HTMLCanvasElement) ||
    !(massRange instanceof HTMLInputElement) ||
    !(massLabel instanceof HTMLSpanElement) ||
    !(widthInput instanceof HTMLInputElement) ||
    !(heightInput instanceof HTMLInputElement) ||
    !(renderBtn instanceof HTMLButtonElement) ||
    !(statusEl instanceof HTMLElement)
) {
    throw new Error("demo markup is incomplete");
}

let rendering = false;

massRange.addEventListener("input", () => {
    massLabel.textContent = Number(massRange.value).toFixed(1);
});

renderBtn.addEventListener("click", async () => {
    if (rendering) {
        return;
    }
    const mass = Number(massRange.value);
    const width = Number(widthInput.value);
    const height = Number(heightInput.value);
    canvas.width = width;
    canvas.height = height;
    rendering = true;
    renderBtn.disabled = true;
    statusEl.textContent = "渲染中…（CPU 测地线，可能需数秒）";
    const t0 = performance.now();
    try {
        const { statusLine } = await renderPmFrame(canvas, mass);
        const dt = ((performance.now() - t0) / 1000).toFixed(2);
        statusEl.textContent = `${statusLine} · ${dt}s`;
    } catch (err) {
        statusEl.textContent = `失败: ${err instanceof Error ? err.message : String(err)}`;
    } finally {
        rendering = false;
        renderBtn.disabled = false;
    }
});
