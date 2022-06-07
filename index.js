const rust = import('./pkg');

rust
  .then(m => {
    console.log(m.start_game(10))
  })
  .catch(console.error);