import * as Comlink from 'comlink';

(async () => {
  const thread = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), { type: 'module' })
  ).thread;

  const numbers = Array.from({ length: 100 }, (_, i) => i + 1);
  const sum = await thread.sum_of_squares(new Int32Array(numbers));
  
  document.body.textContent = `Sum of squares: ${sum}`;
})();
