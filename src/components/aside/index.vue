<script setup lang="ts">
import { useStateStore } from '../../store/state';

const stateStore = useStateStore()
</script>

<template>
  <div class="aside">
    <div class="aside__tabs">
      <button :class="{ 'active': stateStore.activeTab == 0 }" class="btn"
        @click="stateStore.changeActiveTab(0); stateStore.setShowNotesToFalse()">Relatives</button>
      <button :class="{ 'active': stateStore.activeTab == 1 }" class="btn"
        @click="stateStore.changeActiveTab(1); stateStore.setShowNotesToFalse()">Females</button>
      <button :class="{ 'active': stateStore.activeTab == 2 }" class="btn"
        @click="stateStore.changeActiveTab(2); stateStore.setShowNotesToFalse()">Employees</button>
    </div>
    <div>
      <button class="editButton" @click="stateStore.changeActiveTab(4); stateStore.setShowNotesToFalse()"
        v-if="stateStore.hasActiveRelative">Edit</button>
      <button class="editButton" @click="stateStore.changeActiveTab(3); stateStore.setShowNotesToFalse()">new
        relative</button>
    </div>
  </div>
</template>


<style scoped>
.aside {
  background-color: var(--clr-light2);
  display: flex;
  justify-content: space-between;
  padding: var(--size-sm);
}

.aside__tabs {
  display: flex;
  gap: var(--size-sm);
}

.btn {
  height: var(--size-5xl);
  border: none;
  background-color: var(--clr-dark);
  border-radius: var(--size-xxs);
  color: var(--clr-light);
  font-size: var(--size-xl);
  cursor: pointer;
}


.active {
  background-color: var(--clr-light);
  color: black;
}
</style>
