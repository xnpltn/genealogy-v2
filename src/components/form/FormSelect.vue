<script setup lang="ts">
import { useStateStore } from '../../store/state';

const stateStore = useStateStore()
const model = defineModel()

type option = {
  value: any,
  name: string
}
defineProps<{ options: option[], name: string, label: string }>()

</script>

<template>
  <div class="input-container">
    <label :for="name" class="input-label" :class="{ 'input-label_black': stateStore.darkTheme }">{{ label }}</label>
    <select :name="name" :id="name" class="form-select" v-model="model"
      :class="{ 'form-select_black': stateStore.darkTheme }">
      <option v-for="option in options" :value="option.value">{{ option.name }}</option>
    </select>

  </div>

</template>
