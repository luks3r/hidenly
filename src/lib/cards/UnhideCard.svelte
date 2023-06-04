<script lang="ts">
	import { clipboard } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import init, { decode } from 'wasm';

	onMount(async () => {
		await init();
	});

	export let shown = '';
	export let hiddenType: string = 'text';

	let decodedText: Promise<string>;
	let decodedImage: Promise<string>;

	async function decodeImage(msg: string): Promise<string> {
		return decodeMessage(msg);
	}

	async function decodeMessage(msg: string): Promise<string> {
		await init();
		return decode(msg);
	}

	function handleInput(e: Event): void {
		decodedText = decodeMessage(shown);
	}

	function handleInputImage(e: Event): void {
		decodedImage = decodeImage(shown);
	}
</script>

{#if hiddenType == 'image'}
	<h3>Encoded Message</h3>
	<textarea
		class="textarea"
		bind:value={shown}
		on:input={handleInputImage}
		placeholder="Enter the encoded message."
	/>

	<span>Result</span>
	{#await decodedImage}
		<h3>Generating...</h3>
	{:then image}
		<img src={image} alt="" />
	{:catch}
		<h3>Error occurred</h3>
	{/await}
{:else}
	<h3>Encoded Message</h3>
	<textarea
		class="textarea"
		bind:value={shown}
		on:input={handleInput}
		placeholder="Enter the encoded message."
	/>
	<br /><br />
	<span>Result</span>
	{#await decodedText}
		<textarea class="textarea" data-clipboard="decodedText" value="Generating..." />
		<button class="btn btn-sm mr-4 variant-filled" disabled={true}>Copy</button>
	{:then text}
		<textarea class="textarea" data-clipboard="decodedText" value={text ?? ""} />
		<button class="btn btn-sm mr-4 variant-filled" use:clipboard={{ input: 'decodedText' }}
			>Copy</button
		>
	{:catch}
		<textarea class="textarea" data-clipboard="decodedText" value="Error occurred" />
		<button class="btn btn-sm mr-4 variant-filled-secondary" disabled={true}>Copy</button>
	{/await}
{/if}
