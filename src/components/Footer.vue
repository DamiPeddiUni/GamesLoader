<template>
    <div class="footer-container" v-if="showingMenu == false">
        <Key keyLabel="A" keyHint="Avvia"/>
        <Key keyLabel="X" keyHint="Menu"/>
    </div>
    <div class="footer-container" v-if="showingMenu">
        <Key keyLabel="A" keyHint="Seleziona"/>
        <Key keyLabel="B" keyHint="Indietro"/>
    </div>
</template>

<script>
    import Key from './Key.vue';
    export default {
        name: "Footer",
        data(){
            return {}
        },
        components: {
            Key
        },
        props: {
            showingMenu: Boolean
        },
        methods: {

        },
        mounted(){

        }
    }
</script>


<style scoped>
    .footer-container{
        display: flex;
        flex-direction: row;
        justify-content: flex-end;
        align-items: center;
        padding: 1vw;
        gap: 1vw;
        box-sizing: border-box;
    }
</style>