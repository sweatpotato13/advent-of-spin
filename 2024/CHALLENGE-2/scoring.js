/**
 * This module is the JS implementation of the `reverser` WIT world
 */

/**
 * The Javascript export below represents the export of the `reverse` interface,
 * which which contains `reverse-string` as it's primary exported function.
 */
export const scoring = {
    /**
     * This Javascript will be interpreted by `jco` and turned into a
     * WebAssembly binary with a single export (this `reverse` function).
     */
    scoring(name) {
        const randomFactor = Math.floor(Math.random() * 100);
        return Math.max(1, Math.min(randomFactor, 99));
    },
};
