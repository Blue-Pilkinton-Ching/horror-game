<script>
	import initGameWasm from '$lib/assets/wasm/game';

	initGameWasm().then(() => {
		loading = false;
	});

	let loading = $state(true);
</script>

{#if loading}
	<p>Loading...</p>
{/if}
<canvas id="game_canvas"></canvas>
