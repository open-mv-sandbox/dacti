async function init() {
  // Load the viewer WebAssembly module
  console.log('initializing module...');
  const {Viewer} = await import('../pkg');

  // Create the viewer
  const canvas = document.getElementById("viewer");
  const viewer = await Viewer.from_canvas(canvas);

  // Add an object
  viewer.add_object(42);
}

init();
