<script setup lang="ts">
defineProps<{ model_open: boolean }>()
const emits = defineEmits(['close-modal'])

function closeModal() {
  emits("close-modal")
}

</script>

<template>
  <div class="modal" v-if="model_open">
    <div class="modal__container">
      <div>
        <button @click="closeModal">Close</button>
        <slot />
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal {
  position: absolute;
  background: rgba(0, 0, 0, 0.1);
  height: 100vh;
  width: 100vw;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  top: 0;
  left: 0;

}

.modal__container {
  height: 50%;
  width: 500px;
  background: white;
  border: none;
  border-radius: var(--size-sm);
  padding: var(--size-sm);
}
</style>
