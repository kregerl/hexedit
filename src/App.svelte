<script lang="ts">
	import VirtualList from "@sveltejs/svelte-virtual-list";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { onDestroy, onMount } from "svelte";
	import HexLine from "./HexLine/HexLine.svelte";

	let bytes: Uint8Array = new Uint8Array();
	let fileOpenedUnlistener: UnlistenFn;
	onMount(async () => {
		fileOpenedUnlistener = await listen("file-opened", (event) => {
			let payload = event.payload as Uint8Array;
			bytes = payload;
			// let path = event.payload as string;
			// bytes = await readBinaryFile(path);
			console.log("Read", bytes);
		});
	});

	onDestroy(() => {
		fileOpenedUnlistener();
	});

	function formatBytes(bytes: Uint8Array): number[][] {
		let lines: number[][] = [];
		for (let i = 0; i < bytes.length; i += 16) {
			let lineBytes: number[] = [];
			for (let j = i; j < Math.min(i + 16, bytes.length); j++) {
				// let hexByte = bytes[j].toString(16).padStart(2, "0").toUpperCase();
				// lineBytes.push(`${hexByte}`);
				lineBytes.push(bytes[j]);
			}
			lines.push(lineBytes);
		}
		return lines;
	}

	function scrollTest(event) {
		console.log("Scrolled: ", event);
	}

</script>

<div on:mousewheel={scrollTest}>
	{#each formatBytes(bytes) as row}
		<HexLine bytes={row} />
	{/each}
</div>

<style>
	div {
		line-height: 20px;
		/* overflow-y: scroll; */
		min-height: 200px;
		height: 100vh;
		color: white;
		border: 2px solid red;
	}
</style>
