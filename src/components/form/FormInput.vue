<script setup lang="ts">
import { useStateStore } from '../../store/state';
const stateStore = useStateStore()
const model = defineModel()
defineProps<{ typ?: string, name: string, title: string, max?: number, min?: number, step?: number }>()
</script>

<template>
  <div class="input-container">
    <label for="email" class="input-label" :class="{ 'input-label_black': stateStore.darkTheme }">{{ title }}</label>
    <input v-bind="{ id: name, name: name, type: typ || 'text', max: max, min: min, step: step }" class="form-input"
      v-model="model" :class="{ 'form-input_black': stateStore.darkTheme }">
  </div>
</template>
