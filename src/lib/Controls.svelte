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
        if (runGame) {
            runGame = false;
            clearInterval();
        }

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

    const resizeBoard = async () => {
        await invoke('resize_board', {width: parseInt(width), height: parseInt(height)})
            .then(async () => board.set(await invoke('get_board')))
            .then(async () => aliveCount.set(await invoke('get_alive_count')));
    };

    const changeSpeed = async (e) => {
        const value = Math.max(0.1, Math.min(e.target.value, 2));

        if (value && 0.1 < value && value <= 2) {
            speed.set(value);
            await runUpdate();
        }
    };
</script>

<div class="grid-span-2 h-full w-full text-peach">
  <div class="block rounded bg-mantle m-3 p-2">
    <!--  Start, Stop, Reset  -->
    {#if runGame}
      <button on:click={toggleRunGame} class="rounded-lg w-16 h-8"> Pause</button>
    {:else}
      <button on:click={toggleRunGame} class="rounded-gl w-16 h-8"> Play</button>
    {/if}

    <button on:click={nextStep} class="rounded-lg w-16 h-8">󰙢 Step</button>

    <button on:click={resetGame} class="rounded-lg w-16 h-8"> Reset</button>
  </div>

  <!--inline-flex items-center justify-center text-sm font-medium text-gray-900 rounded-lg group bg-gradient-to-br from-purple-600 to-blue-500-->
  <!--  Horizontal & Vertical Counts  -->
  <div class="block rounded bg-mantle m-3 p-2 pb-4">
    <div class="w-full grid grid-cols-2 gap-2 p-2">
      <label class="rounded w-16 h-8 grid-cols-1" for="numWidth">Width</label>
      <input id="numWidth" type="number" class="rounded-lg w-16 h-8 text-center grid-cols-2 mr-0 bg-crust"
             on:change={changeWidth} min="8" max="1024"
             value={width}/>
      <label for="numHeight" class="rounded w-16 h-8 grid-cols-1">Height</label>
      <input id="numHeight" type="number" class="rounded-lg w-16 h-8 text-center grid-cols-2 mr-0 bg-crust"
             on:change={changeHeight} min="8"
             max="1024" value={height}/>
    </div>

    <div class="flex">
      <button on:click={resizeBoard} class="mx-auto mt-3">󰩨 Resize</button>
    </div>
  </div>

  <div class="block rounded bg-mantle m-3 p-2">
    <!--  Speed Slider  -->
    <input type="number" class="rounded-lg w-16 h-8 text-center grid-cols-2 mr-0 bg-crust" on:change={changeSpeed}
           min="0.1" max="2" step="0.1" value={$speed}/>
    <input type="range" class="rounded w-32 h-8" on:change={changeSpeed} min="0.1" max="2" step="0.1" value={$speed}/>
  </div>

  <div class="m-3 flex">
    <label class="rounded h-8 justify-center m-auto">Cells alive: {$aliveCount}</label>
  </div>
</div>

<style>
</style>