<template>
    <div class="status">
        <img src="../assets/ConnectionStatus_Connected.svg" v-if="connected"/>
        <img src="../assets/ConnectionStatus_Disconnected.svg" v-else/>
    </div>
</template>

<script>
export default {
    name: "ConnectionStatus",
    data(){
        return{
            connected: false
        }
    },
    methods: {
        checkConnection(){
            this.connected = window.navigator.onLine
        }
    },
    mounted(){
        setInterval(() => this.checkConnection(), 5000)
    }
}
</script>


<style scoped>
    .status {
        width: 1.5vw;
        height: 1.5vw;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .status img{
        width: 100%;
        height: 100%;
        object-fit: contain;
    }
</style>