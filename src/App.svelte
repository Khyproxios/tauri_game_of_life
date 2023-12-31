<script>
    import './styles.css';
    import Controls from "./lib/Controls.svelte";
    import Board from "./lib/Board.svelte";
    import {board} from "./store.js";
</script>

<main class="w-screen h-dvh mocha bg-crust flex flex-row">
  <div class="w-1/5">
    <Controls />
  </div>
  <div class="w-4/5">
    <Board board={$board} />
  </div>
</main>