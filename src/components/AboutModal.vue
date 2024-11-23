<script setup lang="ts">
import Modal from './Modal.vue';
import { openAboutModal } from '../composables/about';
import { useStateStore } from '../store/state';
const stateStore = useStateStore()

</script>


<template>
  <div>
    <Modal :model_open="openAboutModal" @close-modal="openAboutModal = false">
      <div class="version-card" :class="{ 'dark-mode': stateStore.darkTheme }">
        <div class="logo">ACME Inc.</div>
        <div class="content">
          <div class="version">Version 3.0.0</div>
          <div class="details">
            <p>Build: 2024.11.23</p>
            <p>Environment: Production</p>
            <p>Branch: main</p>
          </div>
        </div>
        <div class="footer">
          <p>© 2024 ACME Corporation</p>
          <p>All rights reserved</p>
        </div>
      </div>
    </Modal>
  </div>
</template>


<style>
.version-card {
  width: 300px;
  height: 300px;
  background-color: var(--clr-light);
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 20px;
  font-family: Arial, sans-serif;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  transition: all 0.3s ease;
}

.version-card.dark-mode {
  background-color: var(--clr-dark);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.logo {
  font-size: 24px;
  font-weight: bold;
  color: var(--clr-dark);
  text-align: center;
  padding: 10px 0;
  border-bottom: 1px solid var(--clr-light2);
}

.dark-mode .logo {
  color: var(--clr-light);
  border-bottom: 1px solid var(--clr-dark2);
}

.content {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 15px;
}

.version {
  font-size: 20px;
  color: var(--clr-frost);
  text-align: center;
  font-weight: bold;
}

.dark-mode .version {
  color: var(--clr-frost3);
}

.details {
  color: var(--clr-dark1);
  font-size: 14px;
}

.dark-mode .details {
  color: var(--clr-light2);
}

.details p {
  margin: 5px 0;
  text-align: center;
}

.footer {
  font-size: 12px;
  color: var(--clr-dark2);
  text-align: center;
  border-top: 1px solid var(--clr-light2);
  padding-top: 10px;
}

.dark-mode .footer {
  color: var(--clr-light1);
  border-top: 1px solid var(--clr-dark2);
}

.footer p {
  margin: 2px 0;
}
</style>
