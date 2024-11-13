<script setup lang="ts">
import Settings from '../icons/Settings.vue';
//import Info from '../icons/Info.vue';
import { inject, type Ref } from 'vue';
const active_tab = inject<Ref<number>>("active_tab")
</script>

<template>
  <div class="aside">
    <div>
      <div class="aside__tabs">
        <button :class="{ 'active': active_tab == 0 }" class="btn" @click="active_tab = 0">Relatives</button>
        <button :class="{ 'active': active_tab == 1 }" class="btn" @click="active_tab = 1">Females</button>
        <button :class="{ 'active': active_tab == 2 }" class="btn" @click="active_tab = 2">Employees</button>
      </div>
      <hr>
      <div class="aside__tabs">
        <button :class="{ 'active': active_tab == 3 }" class="btn" @click="active_tab = 3">Add New</button>
      </div>
    </div>

    <div class="aside__info">
      <button class="icon-btn " :class="{'setting-btn': active_tab == 4}" @click="active_tab = 4">
        <Settings :height="32" :width="32" />
      </button>
    </div>
  </div>
</template>


<style scoped>
.aside {
  top: 0;
  left: 0;
  bottom: 0;
  width: 300px;
  position: fixed;
  height: 100vh;
  background-color: var(--clr-light2);
  display: flex;
  justify-content: space-between;
  flex-direction: column;
  padding-top: var(--size-sm);
}


.aside__tabs {
  padding: var(--size-sm);
  gap: var(--size-sm);
  display: flex;
  flex-direction: column;
}

.aside__info {
  display: flex;
  align-items: center;
  justify-content: end;
  margin-bottom: var(--size-3xl);
  padding-right: var(--size-sm);
  gap: var(--size-sm);
}

.btn {
  width: 100%;
  height: var(--size-5xl);
  border: none;
  background-color: var(--clr-dark);
  border-radius: var(--size-xxs);
  color: var(--clr-light);
  font-size: var(--size-xl);
  cursor: pointer;
}

.icon-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: background-color 0.2s ease;
  background-color: rgba(0, 0, 0, 0.2);
}

.setting-btn{
 background-color: white !important; 
}

.icon-btn:hover {
  background-color: rgba(0, 0, 0, 0.1);
}

.active {
  background-color: var(--clr-light);
  color: black;
}
</style>
