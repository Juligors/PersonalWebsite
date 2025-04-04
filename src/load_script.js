// import init from './bella.js'

// document.getElementById('loading-message').style.display = 'block';

// try{
//     init()
// }
// catch(error){}
// finally{
//     document.getElementById('loading-message').style.display = 'none';
// }

// Function to initialize the Bevy simulation
async function initializesimulation() {
    // Show the loading message
    document.getElementById('loading-message').style.display = 'block';
  
    try {
      const wasmModule = await import('./bella.js');
      await wasmModule.default(); // Initialize the simulation
    } catch (error) {
      console.error('Error loading the simulation:', error);
      document.getElementById('loading-message').innerText = 'Failed to load the simulation.';
    } finally {
      document.getElementById('loading-message').style.display = 'none';
    }
  }
  
  // Call the initializesimulation function when the window loads
  window.onload = initializesimulation;
