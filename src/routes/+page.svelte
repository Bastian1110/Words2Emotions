<script>
     // @ts-nocheck
    let recognition;
    let text = "Sup?";
    try {
        const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
        recognition = new SpeechRecognition();
    } catch (e) {
        console.error(e);
    }

    recognition.interimResults = true;
    recognition.lang = 'es';

    const activateRecognition = () => {
        recognition.start();
        console.log("Recognition started")
        return;
    }

    const deactivateRecognition = () => {
        recognition.stop();
        console.log("Recognition stoped")
        return;
    }
    recognition.onresult = (event) => {
        text = event.results[0][0].transcript;
    }
</script>

<h1 class="font-bold text-6xl m-10 text-[#affaff]">Words to Emotions</h1>
<div class="text-4xl m-10 text-[#d4f9fc]">
    <p>{text}</p>
</div>

<div class="grid place-items-center">
    <button on:mousedown={activateRecognition} on:mouseup={deactivateRecognition} class="trasnition duration-300 bg-[#6653bef7] p-6 rounded-full hover:scale-110 hover:bg-[#5d48bdf7] active:bg-[#7a43b4f7]"> 
        <svg class="h-16 w-16 text-[#affaff]"  fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"/>
        </svg>
    </button>
</div>

<style lang="postcss">
    :global(html){
        background: #8171c7f7;
    }
</style>