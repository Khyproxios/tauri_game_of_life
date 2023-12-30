<script>
    import {spring} from 'svelte/motion';
    import {invoke} from "@tauri-apps/api/tauri";

    let runGame = false;
    let width = 10;
    let height = 10;
    let speed = 1;

    /*
    TODO: Find out how to use the animations with the inputs

    const displayed_count = spring();
    $: displayed_count.set(value);
    $: offset = modulo($displayed_count, 1);

     */

    const toggleRunGame = async () => runGame = !runGame;

    const resetGame = async () => {
        runGame = false;
        width = 10;
        height = 10;
        speed = 1;
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
        const value = e.target.value;

        if (value && 0.1 < value && value <= 10) {
            speed = value;
        }
    };

    /*
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

    const increase = async () => {
        await invoke("increase").then(() => getValue());
    };

    const decrease = async () => {
        await invoke("decrease").then(() => getValue());
    };

    const getValue = async () => {
        value = await invoke("get_value")
    };
     */
</script>

<form class="mt-2 ml-2 col-3 h-100">
  <div class="form-group mb-1">
    <!--  Start, Stop, Reset  -->
    {#if runGame}
      <button on:click={toggleRunGame}
              class="btn btn-outline-primary btn-sm btn-dark w-25 text-sm-start">
        Pause
      </button>
    {:else}
      <button on:click={toggleRunGame}
              class="btn btn-outline-primary btn-sm btn-dark w-25 text-sm-start">
        Play
      </button>
    {/if}

    <button on:click={resetGame}
            class="btn btn-outline-primary btn-sm btn-dark w-25 text-sm-start">
      Reset
    </button>
  </div>

  <!--  Horizontal & Vertical Counts  -->
  <div class="form-group mb-1">
    <label class="text-light w-25 mb-2 text-sm-start"
           for="numWidth">
      Width
    </label>
    <input id="numWidth"
           type="number"
           class="form-control-sm text-bg-dark w-50 mb-2"
           on:change={changeWidth}
           min="8"
           max="1024"
           value={width}
    />
    <label for="numHeight"
           class="text-light w-25 text-sm-start">
      Height
    </label>
    <input id="numHeight"
           type="number"
           class="form-control-sm text-bg-dark w-50"
           on:change={changeHeight}
           min="8"
           max="1024"
           value={height}
    />
  </div>

  <div class="form-group mb-1">
    <!--  Speed Slider  -->
    <input type="number"
           class="form-control-sm text-bg-dark text-light w-50"
           on:change={changeSpeed}
           min="0.1"
           max="10"
           value={speed}
    />
    <input type="range"
           class="form-control-sm text-bg-dark text-light w-50"
           on:change={changeSpeed}
           min="0.1"
           max="10"
           value={speed}
    />
  </div>
</form>

<style>
</style>