<template>
    <div class="game-card" :style="{ 'background-image': 'url(' + game.cover + ')'}">
        <div class="game-info-container">
            <div class="info-label">{{game.name}}</div>
            <!--div class="info-container">
                <div class="info-label">0 ore</div>
                <div class="info-label">·</div>
                <div class="info-label">22/09/22</div>
            </div-->
            <Key keyLabel="A" keyHint="Avvia"/>
        </div>
    </div>
</template>

<script>
import Key from './Key.vue';
export default {
    name: "Game",
    data() {
        return {};
    },
    props: {
        game: Object
    },
    methods: {},
    mounted() {
    },
    components: { Key }
}
</script>


<style scoped>
    .game-card{
        width: auto;
        aspect-ratio: 2/3;
        height: 100%;
        display: inline-block;
        margin: 0 10px;
        background-position: center;
        background-repeat: no-repeat;
        background-size: cover;
        border-radius: 0.3vw;
        transition: scale 0.15s ease-in-out;
        opacity: 0.75;
        position: relative;
        overflow: hidden;
    }

    .game-card::before, .game-card::after {
        width: 100%;
        height:100%;
        z-index: 3;
        content:'';
        position: absolute;
        top:0;
        left:0;
        box-sizing: border-box;
        -webkit-transform: scale(0);
        transition: 0.5s;
    }

    .game-card::before {
        border-bottom: 0.2vw solid #FFF;
        border-left: 0.2vw solid #FFF;
        -webkit-transform-origin: 0 100%;
    }

    .game-card::after {
        border-top: 0.2vw solid #FFF;
        border-right: 0.2vw solid #FFF;
        -webkit-transform-origin: 100% 0%;
    }

    .game-card.selected{
        scale: 1.05;
        /*outline: 0.2vw solid white;*/
        opacity: 1;
    }

    .game-card.selected::after, .game-card.selected::before {
        -webkit-transform: scale(1);
    }

    .game-info-container{
        position: absolute;
        bottom: 0;
        left: 50%;
        transform: translateX(-50%);
        border-radius: 0;
        width: 100%;
        height: 0;
        background: black;
        opacity: 1;
        color: white;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        transition: height 0.5s ease-in-out, padding 0.15s ease-in-out;
        overflow: hidden;
        font-size: 1.5vw;
        white-space: pre-wrap;
        text-align: center;
        display: flex;
        flex-direction: column;
        gap: 1vh;
        justify-content: center;
        align-items: center;
    }

    .game-card.selected .game-info-container{
        height: auto;
        padding: 2vh 0;
    }

    .info-container{
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		gap: 1vh;
	}

	.info-label{
		color: white;
		font-size: 1.2vw;
	}

</style>