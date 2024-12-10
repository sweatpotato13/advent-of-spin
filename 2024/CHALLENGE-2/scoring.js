export function scoring(name) {
    const randomFactor = Math.floor(Math.random() * 100);
    return Math.max(1, Math.min(randomFactor, 99));
}
