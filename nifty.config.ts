// packing-man workspace Nifty configuration (format + hygiene checks).
import { defineConfig } from '@doki-land/nifty';

export default defineConfig({
    format: {
        preset: 'npm-tools',
        includes: ['scripts/**', 'projects/packages/**', 'projects/examples/**', 'package.json', 'nifty.config.ts'],
        excludes: ['**/dist/**', '**/fixtures/**'],
        rust: true,
        javascript: true,
    },
    changelog: {
        repo: 'oovm/packing-man',
    },
});
