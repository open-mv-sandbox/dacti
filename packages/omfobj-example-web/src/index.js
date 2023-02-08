// Automatically built for us by webpack
const module = import('../pkg');

console.log('initializing module...');
module.then(m => m.greet());
