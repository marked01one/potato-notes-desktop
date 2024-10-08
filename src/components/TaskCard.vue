<script setup lang="ts">
defineProps(['cardNo'])
</script>

<template>
    <div class="task-card">
       <p>Hello!, card number {{ cardNo }}</p> 
    </div>
</template>

<style scoped>
.task-card {
    margin: 0 auto;
    border: 4px solid black;
    width: 50%;
}
</style>