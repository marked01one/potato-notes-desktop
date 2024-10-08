<template>
    <div class="task-card">
       <p>Hello!, card number {{ cardNo }}</p> 
    </div>
</template>

<script setup lang="ts">
defineProps(['cardNo'])
</script>


<style scoped>
.task-card {
    margin: 0 auto;
    border: 4px solid black;
    width: 50%;
}
</style>