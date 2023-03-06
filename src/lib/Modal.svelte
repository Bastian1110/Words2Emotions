<script>
	import { slide } from 'svelte/transition';

	let isOpen = true;
</script>

<button class="px-4 py-2 bg-blue-400 text-white rounded-xl" on:click={() => (isOpen = !isOpen)}>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		fill="none"
		viewBox="0 0 24 24"
		stroke-width="1.5"
		stroke="currentColor"
		class="w-8 h-8"
	>
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z"
		/>
	</svg>
</button>

<div
	class="transition-all duration-500 fixed inset-0 z-10 overflow-y-auto bg-blue-900 bg-opacity-60"
	style="display: {isOpen ? 'block' : 'none'};"
	transition:slide={{ duration: 1000 }}
>
	<div class="flex items-center justify-center min-h-screen">
		<div class="bg-white rounded-lg shadow-lg px-6 py-4 w-[70%]">
			<section class="w-[100%]">
				<h2 class="font-bold text-4xl inline-block">Welcome!</h2>
				<button
					class="px-4 py-1 bg-blue-500 text-white inline-block float-right"
					on:click={() => (isOpen = false)}>X</button
				>
			</section>
			<section class="w-[45%] inline-block">
				<p class="text-lg">
					This is webpage of the project <a
						class="text-blue-500"
						target="_blank"
						rel="noopener noreferrer"
						href="https://github.com/Bastian1110/Words2Emotions">Words2Emotions</a
					>, which aims to make an AI model run natively on your computer, without the need of downloading anything!<br>
					The model is trained to try to classify what you are feeling based on the things you say to the web page. It's important to say that
					it only has an acurracy of <i>53.2%</i>, so it fails once for every two trys.
				</p>
			</section>
			<section class="w-[45%] inline-block">
				<img src="images/first_cat.png" alt="Thinking cat" class="w-[80%] float-right"/>
			</section>
		</div>
	</div>
</div>
