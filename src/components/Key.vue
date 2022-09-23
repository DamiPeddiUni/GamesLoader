<template>
    <div class="key-box">
        <img class="key-image" :src="getImageURL"/>
        <div class="key-hint">{{keyHint}}</div>
    </div>
    
</template>

<script>
export default {
    name: "Key",
    data(){
        return{
            
        }
    },
    props: {
        keyLabel: String,
        keyHint: String
    },
    methods: {

    },
    mounted(){
    
    },
    computed: {
        getImageURL(){
            return new URL('../assets/Buttons/button_xbox_digital_' + this.keyLabel.toLowerCase() + '_5.svg', import.meta.url).href
        }
    }
}
</script>


<style scoped>
    .key-box{
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        gap: 0.6vw;
    }
    .key-image{
        
        width: 2vw;
        height: 2vw;
    }

    .key-hint{
        color: white;
        font-size: 1vw;
    }
</style>