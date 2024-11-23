<script setup lang="ts">
import Modal from './Modal.vue';
import { showError, errorTitle, errorValue } from '../composables/error';
function close() {
  errorValue.value = ''
  showError.value = false
  errorTitle.value = ''
}

</script>

<template>
  <div>
    <Modal :title="errorTitle" :model_open="showError" @close-modal="close">
      <div class="error-modal">
        {{ errorValue }}
      </div>
    </Modal>
  </div>
</template>

<style scoped>
.error-modal {
  min-height: 70px;
  min-width: 70px;
}
</style>
