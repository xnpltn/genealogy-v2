<script setup lang="ts">
defineProps<{ model_open: boolean, title?: string, cancelTitle?: string }>()
const emits = defineEmits(['close-modal'])

function closeModal() {
  emits("close-modal")
}

</script>

<template>
  <div class="modal" v-if="model_open">
    <div class="modal__container">
      <div class="model__header">
        <div class="modal__title" v-if="title?.length">
          <h1>{{ title }}</h1>
        </div>
        <button class="cancelButton" @click="closeModal">{{ cancelTitle || "Close" }}</button>
      </div>
      <div>
        <slot />
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
  height: auto;
  width: auto;
  background: white;
  border: none;
  border-radius: var(--size-sm);
  padding: var(--size-sm);
}

.model__header {
  display: flex !important;
  justify-content: space-between !important;
  align-items: center !important;
  padding: 15px !important;
  border-bottom: 2px solid var(--clr-light2) !important;
}

.modal__title {
  flex: 1 !important;
  text-align: center !important;
}

.modal__title h1 {
  margin: 0 !important;
  font-size: 1.5rem !important;
  font-weight: bold !important;
  color: var(--clr-dark) !important;
}

cancelButton {
  background-color: transparent !important;
  border: none !important;
  color: var(--clr-dark) !important;
  font-size: 1rem !important;
  cursor: pointer !important;
  font-weight: bold !important;
}

cancelButton:hover {
  color: var(--clr-aurora) !important;
}

cancelButton:focus {
  outline: none !important;
}
</style>
