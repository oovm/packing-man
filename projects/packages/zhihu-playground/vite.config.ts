import { defineConfig } from "vite";

export default defineConfig({
    publicDir: "public",
    build: {
        outDir: "dist",
        emptyOutDir: true,
    },
});
