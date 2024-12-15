<script setup lang="ts">
import Modal from './Modal.vue';
import { showSuccess, successTitle, successValue } from '../composables/success';
function close() {
  successTitle.value = ''
  showSuccess.value = false
  successValue.value = ''
}
import { useStateStore } from '../store/state';
const stateStore = useStateStore()

</script>

<template>
  <div>
    <Modal :title="successTitle" :model_open="showSuccess" @close-modal="close">
      <div class="error-modal" :class="{ 'dark-mode': stateStore.darkTheme }">
        <div class="error-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none">
            <rect x="2" y="2" width="20" height="20" rx="4" fill="#4CAF50" />
            <path d="M8 12l3 3 5-5" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
        <div class="error-message">
          {{ successValue }}
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
  border: 1px solid green;
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
  color: green;
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
