<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {aliveCount} from "../store.js";

    export let x = 0;
    export let y = 0;
    export let isAlive = false;

    const onCellClick = async () =>
        await invoke('toggle', { position: { x, y } })
            .then(() => isAlive = !isAlive)
            .then(async () => aliveCount.set(await invoke('get_alive_count')));
</script>

<div class="{isAlive ? 'alive' : 'dead'} {isAlive ? 'bg-peach' : 'bg-mantle'} grid-col-{x + 1} grid-row-{y + 1}"
     on:click={onCellClick}>
</div>

<style>
  .alive {
      aspect-ratio: 1;
      border-radius: 15%;
      transform: scale(1);
      transition: all 0.1s;
  }

  .dead {
      aspect-ratio: 1;
      border-radius: 15%;
      transform: scale(0.75);
      transition: all 0.1s;
  }
</style>
