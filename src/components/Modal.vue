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
        <slot />
        <button @click="closeModal">Close</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal {
  z-index: 100;
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
  height: 250px;
  width: 700px;
  background: white;
  border: none;
  border-radius: var(--size-sm);
  padding: var(--size-sm);
}

.modal__tabbar {
  background-color: red;
  display: flex;
  justify-content: end;
}

.modal__form-container {
  width: 100%;
}
</style>
