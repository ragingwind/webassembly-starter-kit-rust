import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

async function importModule() {
  if (!(await threads())) {
    return;
  }

  const thread = await import('./pkg/with_thread.js');
  await thread.default();
  await thread.initThreadPool(navigator.hardwareConcurrency);

  return Comlink.proxy({
    sum_of_squares: thread.sum_of_squares,
  });
}

Comlink.expose({
  thread: importModule(),
});
