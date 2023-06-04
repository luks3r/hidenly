<script lang="ts">
	import { FileButton, clipboard } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import init, { encode } from 'wasm';

	onMount(async () => await init());

	export let message = '';
	export let secret = '';
	let encodedText = encodeMessage(message, secret);

	export let hiddenType: string = 'text';

	let image = '';
	let encodedImage = '';

	async function imageToBase64(image: File): Promise<string> {
		const arrayBuffer: ArrayBuffer = await image.arrayBuffer();
		const uint8Array = new Uint8Array(arrayBuffer);

		let binaryString: string = '';
		uint8Array.forEach((byte) => {
			binaryString += String.fromCharCode(byte);
		});

		return btoa(binaryString);
	}

	async function encodeImage(msg: string, image: File): Promise<string> {
		const convertedImage: string = await imageToBase64(image);
		const imageWithMeta = `data:${image.type};base64,${convertedImage}`;
		return encodeMessage(msg, imageWithMeta);
	}

	async function onFileChange(e: Event) {
		const firstFile: File = (e.target as HTMLInputElement)?.files?.item(0)!;
		const baseEncoded = await imageToBase64(firstFile);
		image = `data:${firstFile.type};base64,${baseEncoded}`;
		encodedImage = await encodeImage(message, firstFile);
	}

	async function encodeMessage(msg: string, secret: string): Promise<string> {
		await init();
		return encode(msg, secret);
	}

	function handleInput(e: Event) {
		encodedText = encodeMessage(message, secret);
	}
</script>

<span>Shown text</span>
<textarea
	class="textarea"
	bind:value={message}
	placeholder="Enter a message you want to be shown."
/>

{#if hiddenType == 'image'}
	<h3>Image</h3>
	<FileButton
		class="btn variant-filled"
		name="imageToHide"
		disabled={message.length === 0}
		on:change={onFileChange}
	/>
	<br />
	<img src={image} width="300" alt="" />
	<br /><br />
	<span>Result</span>
	{#await encodedImage}
		<textarea class="textarea" value="Generating..." />
	{:then text}
		<textarea class="textarea" disabled={true} data-clipboard="encodedImage" value={text} />
		<button class="btn btn-sm mr-4 variant-filled" use:clipboard={{ input: 'encodedImage' }}
			>Copy</button
		>
	{:catch error}
		<textarea class="textarea variant-filled-secondary" value="Error occurred" />
	{/await}
{:else}
	<h3>Hidden Message</h3>
	<textarea
		class="textarea"
		bind:value={secret}
		on:input={handleInput}
		placeholder="Enter a message you want to hide."
		disabled={message.length === 0}
	/>
	<br /><br />
	<span>Result</span>
	{#await encodedText}
		<textarea class="textarea" value="Generating..." />
		<button class="btn btn-sm mr-4 variant-filled" disabled={true}>Copy</button>
	{:then text}
		<textarea class="textarea" data-clipboard="encodedText" value={text} />
		<button class="btn btn-sm mr-4 variant-filled" use:clipboard={{ input: 'encodedText' }}
			>Copy</button
		>
	{:catch error}
		<textarea class="textarea variant-filled-secondary" value="Error occurred" />
		<button class="btn btn-sm mr-4 variant-filled-secondary" disabled={true}>Copy</button>
	{/await}
{/if}
