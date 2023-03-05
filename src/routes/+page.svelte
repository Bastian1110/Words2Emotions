<script>
	// @ts-nocheck
	//Importing the WASM generated in the rust code
	import { onMount } from 'svelte';
	import init, { EmotionRecognition } from 'text_emotions_rust';
	//Creating the emotion detection model
	let model;
	onMount(async () => {
		let wasm = await init();
		model = EmotionRecognition.new();
	});
	let emotion;
	//Helper for passing the input to the model
	const predictEmotion = (input) => {
		let result = model.predict(input);
		emotion = result;
	};
	import { Modal } from '$lib';

	let recognition;
	let text = 'Say something ...';
	try {
		const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
		recognition = new SpeechRecognition();
	} catch (e) {
		console.error(e);
	}

	recognition.interimResults = true;
	recognition.lang = 'en';

	const activateRecognition = () => {
		recognition.start();
		console.log('Recognition started');
		return;
	};

	const deactivateRecognition = () => {
		recognition.stop();
		console.log('Recognition stoped');
		predictEmotion(text);
		myEffect(emotions);
		return;
	};

	recognition.onresult = (event) => {
		text = event.results[0][0].transcript;
	};

	const letters = 'qwertyuioplkjhgfdsazxcvbnm';

	let binding;
	let val = null;
	const myEffect = (newText) => {
		let iteration = 0;
		clearInterval(val);
		val = setInterval(() => {
			binding.innerText = newText
				.split('')
				.map((letter, index) => {
					if (index < iteration) {
						return newText[index];
					}
					return letters[Math.floor(Math.random() * 26)];
				})
				.join('');
			if (iteration >= newText.length) {
				clearInterval(val);
			}
			iteration += 1 / 9;
		}, 30);
	};
</script>

<h1 class="font-bold text-4xl md:text-6xl mx-10 mt-12 mb-10 text-[rgb(27,18,57)]">
	Words to Emotions
</h1>
<div
	class="bg-[#f7f7f7] p-4 border-2 border-[#00000066] rounded-lg max-h-[10rem] h-[10rem] overflow-auto text-2xl mx-10 text-[rgba(79,68,112,0.83)] "
>
	<p>{text}</p>
</div>

<div class="grid place-items-center mt-14">
	<button
		on:mousedown={activateRecognition}
		on:mouseup={deactivateRecognition}
		class="trasnition duration-300 bg-[#c6d2fa] p-6 rounded-full group active:scale-110 hover:bg-[#8e8eecf7] active:bg-[#c1a9ebf7]"
	>
		<svg
			class="transition duration-300 h-16 w-16 group-hover:text-[#d1e5ef] text-[#2e4360]"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
			/>
		</svg>
	</button>
</div>

<div class="grid place-items-center mt-[8%]">
	<h1 bind:this={binding} class="text-[rgb(27,18,57)] text-6xl md:text-6xl font-bold">neutral</h1>
</div>

<div class="text-[rgba(74,61,115,0.83)] float-left text-2xl absolute bottom-0 left-0 ml-5 mb-5">
	<Modal />
</div>

<div class="text-[rgba(74,61,115,0.83)] float-right text-2xl absolute bottom-4 right-6 ml-6">
	By <a
		class="underline"
		target="_blank"
		rel="noopener noreferrer"
		href="https://github.com/Bastian1110">Sebastian Mora</a
	>
</div>

<style lang="postcss">
	:global(html) {
		background: #faf8f6;
		font-family: 'Roboto-Mono', monospace;
	}
</style>
