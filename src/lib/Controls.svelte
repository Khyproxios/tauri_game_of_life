<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {aliveCount, board, speed} from "../store.js";

    let runGame = false;
    let width = 10;
    let height = 10;

    let clearInterval;

    const runUpdate = async () => {
        if (!runGame) {
            return;
        }

        await board.update();

        let interval = 1000 / $speed;
        console.log('interval:', interval);
        clearInterval = setTimeout(runUpdate, interval);
    };

    const toggleRunGame = async () => {
        runGame = !runGame;

        if (runGame) {
            await runUpdate();
        } else if (clearInterval) {
            clearInterval();
        }
    }

    const nextStep = async () => {
        await invoke('update')
            .then(async () => board.set(await invoke('get_board')))
            .then(async () => aliveCount.set(await invoke('get_alive_count')));
    };

    const resetGame = async () => {
        runGame = false;
        width = 10;
        height = 10;
        speed.set(1);

        await board.reset();

        if (clearInterval) {
            console.log('clearing the interval');
            clearInterval();
        }
    };

    const changeWidth = async (e) => {
        const value = e.target.value;

        if (value && 8 < value && value <= 1024) {
            width = value;
        }
    };

    const changeHeight = async (e) => {
        const value = e.target.value;

        if (value && 8 < value && value <= 1024) {
            height = value;
        }
    };

    const changeSpeed = async (e) => {
        console.log('changeSpeed:', e)
        const value = parseInt(e.target.value);

        if (value && 0.1 < value && value <= 10) {
            speed.set(value);
            await runUpdate();
        }
    };
</script>

<div class="grid-span-2 h-full w-full text-peach">
  <div class="block rounded bg-mantle m-2 p-2">
    <!--  Start, Stop, Reset  -->
    {#if runGame}
      <button on:click={toggleRunGame} class="rounded w-16 h-8"> Pause</button>
    {:else}
      <button on:click={toggleRunGame} class="rounded w-16 h-8"> Play</button>
    {/if}

    <button on:click={nextStep} class="rounded w-16 h-8">󰙢 Step</button>

    <button on:click={resetGame} class="rounded w-16 h-8"> Reset</button>
  </div>

  <!--  Horizontal & Vertical Counts  -->
  <div class="block rounded bg-mantle m-2 p-2 flex flex-row">
    <label class="rounded w-16 h-8" for="numWidth">Width</label>
    <input id="numWidth" type="number" class="rounded w-16 h-8 text-center" on:change={changeWidth} min="8" max="1024"
           value={width}/>
    <label for="numHeight" class="rounded w-16 h-8">Height</label>
    <input id="numHeight" type="number" class="rounded w-16 h-8 text-center" on:change={changeHeight} min="8"
           max="1024" value={height}/>
  </div>

  <div class="block rounded bg-mantle m-2 p-2">
    <!--  Speed Slider  -->
    <input type="number" class="rounded w-16 h-8" on:change={changeSpeed} min="0.1" max="2" step="0.1" value={$speed}/>
    <input type="range" class="rounded w-32 h-8" on:change={changeSpeed} min="0.1" max="2" step="0.1" value={$speed}/>
  </div>

  <label class="rounded w-50 h-8">Cells alive: {$aliveCount}</label>
</div>

<style>
</style>