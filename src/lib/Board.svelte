<script>
    import { invoke } from "@tauri-apps/api/tauri";

    let board = {};

    const getBoard = async () => {
        board = await invoke("get_board");
    };
</script>

<div class="col-9 h-100">

</div>