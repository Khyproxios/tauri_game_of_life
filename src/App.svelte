<script>
  import Counter from './lib/Counter.svelte'
</script>

<main class="container">
  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Counter />
  </div>

</main>

<style>
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
