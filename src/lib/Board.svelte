<script>
    import Cell from "./Cell.svelte";

    export let board = {
        width: 10,
        height: 10,
        cells: Array(100).fill(false)
    };

    const createStyle = () => `
      grid-template-rows: repeat(${board.height}, minmax(0, 1fr));
      grid-template-columns: repeat(${board.width}, minmax(0, 1fr));
      aspect-ratio: 1/1;
      width: 100vh;
      margin: auto;`;

    const getAliveAtIndex = (x, y) => {
        return board.cells[y * board.width + x];
    };
</script>

{#key board}
  <div id="board-container" class="grid-col-2 grid-span-5 h-full bg-crust">
    <div class="grid" style={createStyle()}>
      {#each {length: board.height} as _, y}
        {#each {length: board.width} as _, x}
          <Cell x={x} y={y} isAlive={getAliveAtIndex(x, y)} />
        {/each}
      {/each}
    </div>
  </div>
{/key}