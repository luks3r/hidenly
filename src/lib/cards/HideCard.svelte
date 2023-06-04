<script lang="ts">
	import { FileButton, clipboard } from '@skeletonlabs/skeleton';
	import { hideMessage } from '$lib/hidenly';
	import { Gzip, Gunzip } from 'zlibt2';

	export let shown = '';
	export let hidden = '';

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

	function unicodeStringToNumberArray(str: string): number[] {
		const encoder = new TextEncoder();
		const encodedBytes = encoder.encode(str);
		const numberArray = Array.from(encodedBytes);
		return numberArray;
	}

	function numberArrayToUnicodeString(numberArray: number[]): string {
		const uint8Array = new Uint8Array(numberArray);
		const decoder = new TextDecoder();
		const unicodeString = decoder.decode(uint8Array);
		return unicodeString;
	}

	async function hideImage(shown: string, image: File): Promise<string> {
		const convertedImage: string = await imageToBase64(image);
		const imageWithMeta = `data:${image.type};base64,${convertedImage}`;
		console.log(imageWithMeta);
		const gzip = new Gzip(unicodeStringToNumberArray(imageWithMeta));
		const compressedImageArray = gzip.compress() as number[];
		const compressedImageString = numberArrayToUnicodeString(compressedImageArray);
		console.log(compressedImageString);
		return hideMessage(shown, imageWithMeta);
	}

	async function onFileChange(e: Event) {
		const firstFile: File = (e.target as HTMLInputElement)?.files?.item(0)!;
		const baseEncoded = await imageToBase64(firstFile);
		image = `data:${firstFile.type};base64,${baseEncoded}`;
		await hideImage(shown, firstFile);
	}
</script>

<span>Shown text</span>
<textarea class="textarea" bind:value={shown} placeholder="Enter a message you want to be shown." />

{#if hiddenType == 'image'}
	<h3>Image</h3>
	<FileButton class="btn variant-filled" name="imageToHide" on:change={onFileChange} />
	<br />
	<img src={image} width="300" alt="" />
	<br /><br />
	<span>Result</span>
	<textarea class="textarea" data-clipboard="encodedText" value={encodedImage} />
	<button class="btn btn-sm mr-4 variant-filled" use:clipboard={{ input: 'encodedText' }}
		>Copy</button
	>
{:else}
	<h3>Hidden Message</h3>
	<textarea
		class="textarea"
		bind:value={hidden}
		placeholder="Enter a message you want to hide."
		disabled={!shown}
	/>
	<br /><br />
	<span>Result</span>
	<textarea class="textarea" data-clipboard="encodedImage" value={hideMessage(shown, hidden)} />
	<button class="btn btn-sm mr-4 variant-filled" use:clipboard={{ input: 'encodedImage' }}
		>Copy</button
	>
{/if}
