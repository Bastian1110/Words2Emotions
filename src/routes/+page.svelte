<script>
// @ts-nocheck

    let recognition;
    let paragraph;
    try {
        const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
        recognition = new SpeechRecognition();
        console.log(recognition)
        recognition.interimResults = true;
        recognition.lang = 'es';
        recognition.addEventListener('result', e => {
			const transcript = Array.from(e.results)
				.map(result => result[0])
				.map(result => result.transcript)
				.join('')
			console.log(transcript);
            paragraph.innerHTML = transcript;

		});
        recognition.start();
		recognition.addEventListener('end', recognition.start);
    } catch (e) {
        console.error(e);
    }
</script>

<h1 class="font-bold text-6xl m-10 text-[#affaff]">Speech Test</h1>
<div class="text-4xl m-10 text-[#d4f9fc]">
    <p bind:this={paragraph}></p>
</div>




<style lang="postcss">
    :global(html){
        background: #8171c7f7;
    }
</style>