import { renderPackingFrame } from "@sxo/packing-unknown-wasm32";

const WASM_URL = "/pm_wasm_bg.wasm";

export async function renderPmFrame(
    canvas: HTMLCanvasElement,
    mass: number,
): Promise<{ tier: string; statusLine: string }> {
    const { ton, tier } = await renderPackingFrame(canvas, { mass, wasmUrl: WASM_URL });
    const info = ton.info();
    return {
        tier,
        statusLine:
            `${info.name} v${info.version} · UCF diag=0x${info.ucfDiagMask.toString(16)} · ` +
            `present ${tier} · ${canvas.width}×${canvas.height}`,
    };
}
