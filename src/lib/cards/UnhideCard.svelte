<script lang="ts">
	import { unhideMessage } from '$lib/hidenly';
	import { clipboard } from '@skeletonlabs/skeleton';

	export let shown = '';
	export let hiddenType: string = 'text';

	let decodedImage = '';
</script>

<h3>Encoded Message</h3>
<textarea class="textarea" bind:value={shown} placeholder="Enter the encoded message." />

{#if hiddenType == 'image'}
	<span>Result</span>
	<img src={decodedImage} alt="" />
{:else}
	<br /><br />
	<span>Result</span>
	<textarea class="textarea" data-clipboard="decodedText" value={unhideMessage(shown)} />
	<button class="btn btn-sm mr-4 variant-filled" use:clipboard={{ input: 'decodedText' }}
		>Copy</button
	>
{/if}
