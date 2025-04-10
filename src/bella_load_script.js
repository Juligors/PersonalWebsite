async function loadBella() {
  try {
    const wasmModule = await import('./bella.js');
    await wasmModule.default(); // Initialize the simulation
  } catch (error) {
    console.error('Error loading the simulation:', error);
  }
}


async function initializeSimulation() {
  // NOTE: Bevy scrolls to the canvas when it loads, we need to prevent it
  const old_scroll_into_view = Element.prototype.scrollIntoView
  Element.prototype.scrollIntoView = function () {
    console.warn("Blocked scrollIntoView on", this);
  };
  const old_focus = HTMLCanvasElement.prototype.focus
  HTMLCanvasElement.prototype.focus = function () {
    console.warn("Blocked canvas.focus()");
  };

  await loadBella();

  Element.prototype.scrollIntoView = old_scroll_into_view
  HTMLCanvasElement.prototype.focus = old_focus

  document.getElementById('loading-message').style.display = 'none';
}

await initializeSimulation();
