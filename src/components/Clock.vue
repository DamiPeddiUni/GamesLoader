<template>
    <div class="clock-label">{{hours}}<span>:</span>{{minutes}}<span>{{period}}</span></div>
</template>

<script>
export default {
    name: "Clock",
    data(){
        return{
            hours: "00",
            minutes: "00",
            period: "AM"
        }
    },
    methods: {
        setTime(){
            const date = new Date()
            let hours = date.getHours()
            let minutes = date.getMinutes()
            this.hours = hours
            this.period = hours >= 12 ? "PM" : "AM"
            this.minutes = minutes <= 9 ?  `${minutes}`.padStart(2, 0) : minutes
        }
    },
    mounted(){
        setInterval(() => this.setTime(), 1000)
    }
}
</script>


<style scoped>
    .clock-label{
        color: white;
        font-size: 1vw;
    }
    .clock-label span{
        font-size: 0.75vw
    }
</style>