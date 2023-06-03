<script lang="ts">
	import '@skeletonlabs/skeleton/themes/theme-hamlindigo.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '../app.postcss';
	import './styles.css';

	import GithubIcon from '$lib/images/github.svg?component';

	import { AppBar, AppShell } from '@skeletonlabs/skeleton';
	import { Drawer, drawerStore } from '@skeletonlabs/skeleton';

	import Navigation from '$lib/navigation/Navigation.svelte';

	function drawerOpen(): void {
		drawerStore.open({});
	}
</script>

<Drawer>
	<h2 class="p-4">Navigation</h2>
	<hr />
	<Navigation />
</Drawer>

<AppShell slotSidebarLeft="bg-surface-500/5 w-0">
	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">
				<div class="flex items-center">
					<button class="lg:hidden btn btn-sm mr-4" on:click={drawerOpen}>
						<span>
							<svg viewBox="0 0 100 80" class="fill-token w-4 h-4">
								<rect width="100" height="20" />
								<rect y="30" width="100" height="20" />
								<rect y="60" width="100" height="20" />
							</svg>
						</span>
					</button>
					<a href="/" class="no-underline hover:no-underline text-inherit">
						<strong class="text-xl uppercase"> Hidenly </strong>
					</a>
				</div>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<div class="flex items-center">
                    <a href="/" class="no-underline hover:no-underline text-inherit">
						<GithubIcon width="24" />
                    </a>
					<a href="/" class="no-undeline hover:nounderline text-inherit">
						<GithubIcon width="24" />
					</a>
                </div>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>

	<svelte:fragment slot="sidebarLeft">
		<Navigation />
	</svelte:fragment>
	<main>
		<slot />
	</main>
</AppShell>

<style>
</style>
