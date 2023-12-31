<script>
    import Cell from "./Cell.svelte";

    export let board;

    let style = `
      grid-template-rows: repeat(${board.width}, minmax(0, 1fr));
      grid-template-columns: repeat(${board.height}, minmax(0, 1fr));
      aspect-ratio: 1/1;
      width: 100vh;
      margin:auto;`;

    const getAliveAtIndex = (x, y) => {
        return board.cells[y * board.width + x];
    }
</script>

{#key board}
  <div id="board-container" class="grid-col-2 grid-span-5 h-full bg-crust">
    <div class="grid" {style}>
      {#each {length: board.height} as _, y}
        {#each {length: board.height} as _, x}
          <Cell x={x} y={y} isAlive={getAliveAtIndex(x, y)} />
        {/each}
      {/each}
    </div>
  </div>
{/key}