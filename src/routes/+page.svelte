<script lang="ts">
	import { hideMessage, unhideMessage } from '$lib/hidenly';
	import { TabGroup, Tab, FileButton } from '@skeletonlabs/skeleton';

	let shownMessage: string = '';
	let hiddenMessage: string = '';
	let files: FileList;

	let encodedMessage: string = '';

	let tabSet: number = 0;
	let hiddenType: string = '1';

	function hide(shown: string, hidden: string): string {
		if (!hidden) return '';
		return hideMessage(shown, hidden);
	}

	function unhide(encoded: string): string {
		if (!encoded) return '';
		return unhideMessage(encoded);
	}
</script>

<div class="container p-10 space-y-4">
	<TabGroup>
		<Tab bind:group={tabSet} name="hideTab" value={0}>Hide</Tab>
		<Tab bind:group={tabSet} name="unhideTab" value={1}>Unhide</Tab>

		<svelte:fragment slot="panel">
			<span>Hidden Type</span>
			<select class="select" bind:value={hiddenType}>
				<option value="1">Text</option>
				<option value="2">Image</option>
			</select>
			<br /><br />
			{#if tabSet === 0}
				<span>Shown text</span>
				<textarea
					class="textarea"
					bind:value={shownMessage}
					placeholder="Enter a message you want to be shown."
				/>
				<br />
				<br />
				{#if hiddenType === '1'}
					<h3>Hidden Message</h3>
					<textarea
						class="textarea"
						bind:value={hiddenMessage}
						placeholder="Enter a message you want to hide."
					/>
				{:else if hiddenType === '2'}
					<h3>Image</h3>
					<FileButton name="imageToHide" bind:files={files} />

				{/if}
				<br /><br />
				<span>Result</span>
				<textarea class="textarea" value={hideMessage(shownMessage, hiddenMessage)} />
			{:else if tabSet === 1}
				<span>Message</span>
				<textarea
					class="textarea"
					bind:value={encodedMessage}
					placeholder="Enter an encrypted message."
				/>

				<br /><br /><br />
				<span>Unhidden</span>
				<textarea class="textarea" value={unhideMessage(encodedMessage)} />
			{/if}
		</svelte:fragment>
	</TabGroup>
</div>
