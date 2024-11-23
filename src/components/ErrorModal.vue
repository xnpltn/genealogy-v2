<script setup lang="ts">
import Modal from './Modal.vue';
import { showError, errorTitle, errorValue } from '../composables/error';
function close() {
  errorValue.value = ''
  showError.value = false
  errorTitle.value = ''
}
import { useStateStore } from '../store/state';
const stateStore = useStateStore()

</script>

<template>
  <div>
    <Modal :title="errorTitle" :model_open="showError" @close-modal="close">
      <div class="error-modal" :class="{ 'dark-mode': stateStore.darkTheme }">
        <div class="error-icon">
          <svg viewBox="0 0 24 24" fill="none" width="24" height="24">
            <path d="M12 9v5M12 17.01l.01-.011M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"
              stroke="var(--clr-aurora)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
        <div class="error-message">
          {{ errorValue }}
        </div>
      </div>
    </Modal>
  </div>
</template>

<style scoped>
.error-modal {
  width: 400px;
  padding: 16px;
  border-radius: 8px;
  background-color: var(--clr-light);
  box-shadow: 0 2px 8px rgba(191, 97, 106, 0.1);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  border: 1px solid var(--clr-aurora);
  transition: all 0.3s ease;
}

.error-modal.dark-mode {
  background-color: var(--clr-dark1);
  box-shadow: 0 2px 8px rgba(191, 97, 106, 0.2);
}

.error-icon {
  display: flex;
  justify-content: center;
  align-items: center;
  animation: pulse 2s infinite;
}

.error-icon svg {
  width: 32px;
  height: 32px;
}

.error-message {
  color: var(--clr-aurora);
  font-size: 14px;
  text-align: center;
  font-weight: 500;
  word-break: break-word;
  max-width: 100%;
}

@keyframes pulse {
  0% {
    transform: scale(1);
    opacity: 1;
  }

  50% {
    transform: scale(1.1);
    opacity: 0.8;
  }

  100% {
    transform: scale(1);
    opacity: 1;
  }
}

/* Optional: Add a subtle shake animation when the error first appears */
@keyframes shake {

  0%,
  100% {
    transform: translateX(0);
  }

  10%,
  30%,
  50%,
  70%,
  90% {
    transform: translateX(-2px);
  }

  20%,
  40%,
  60%,
  80% {
    transform: translateX(2px);
  }
}

.error-modal {
  animation: shake 0.5s ease-in-out;
}
</style>
