// Automatically built for us by webpack
const module = import('../pkg');

console.log('initializing module...');
module.then(m => {
  const canvas = document.getElementById("viewer");
  m.create_viewer(canvas);
});
