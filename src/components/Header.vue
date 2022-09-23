<template>
    <div class="header-container">
        <Clock />
        <ConnectionStatus />
    </div>
</template>

<script>
    import Clock from "./Clock.vue";
    import ConnectionStatus from "./ConnectionStatus.vue";
    export default {
        name: "Header",
        data(){
            return {}
        },
        components: {
            Clock,
            ConnectionStatus
        },
        methods: {

        },
        mounted(){

        }
    }
</script>


<style scoped>
    .header-container{
        display: flex;
        flex-direction: row;
        justify-content: flex-end;
        align-items: center;
        padding: 1vw;
        gap: 1vw;
        box-sizing: border-box;
    }
</style>