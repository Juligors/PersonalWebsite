const pressedButtonSelector = '[data-theme][aria-pressed="true"]';
const defaultTheme = 'dark';

const applyTheme = (theme) => {
  const target = document.querySelector(`[data-theme="${theme}"]`);
  document.documentElement.setAttribute("data-selected-theme", theme);
  document.querySelector(pressedButtonSelector).setAttribute('aria-pressed', 'false');
  target.setAttribute('aria-pressed', 'true');
};

const handleThemeSelection = (event) => {
  const target = event.target;
  const isPressed = target.getAttribute('aria-pressed');
  const theme = target.getAttribute('data-theme');

  if (isPressed !== "true") {
    applyTheme(theme);
    localStorage.setItem('selected-theme', theme);
  }
}

const setInitialTheme = () => {
  const savedTheme = localStorage.getItem('selected-theme');
  if (savedTheme && savedTheme !== defaultTheme) {
    applyTheme(savedTheme);
  }
};

setInitialTheme();

const themeSwitcher = document.querySelector('#theme-switcher');
const buttons = themeSwitcher.querySelectorAll('button');

buttons.forEach((button) => {
  button.addEventListener('click', handleThemeSelection);
});

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
