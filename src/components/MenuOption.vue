<template>
    <div class="menu-option">
        <div class="menu-label">{{optionLabel}}</div>
    </div>
</template>

<script>
export default {
    name: "MenuOption",
    data(){
        return{
            
        }
    },
    props: {
        optionLabel: String
    },
    methods: {

    },
    mounted(){
    
    },
    computed: {
    }
}
</script>

<style scoped>
    
    .menu-option{
        width: 100%;
        height: auto;
        box-sizing: border-box;
        padding: 0.75vw;
        background-color: transparent;
        border-radius: 0.5vw;
    }

    .menu-option.selected{
        outline: 0.2vw solid white;
    }
    .menu-label{
        width: 100%;
        text-align: center;
        font-size: 1vw;
        color: white;
    }
</style>