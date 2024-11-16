<script setup lang="ts">
import { inject, type Ref } from 'vue';
const active_tab = inject<Ref<number>>("active_tab")
const hasActiveRelative = inject<Ref<boolean>>("hasActiveRelative")
const showNotes = inject("showNotes") as Ref<boolean, boolean>
</script>

<template>
  <div class="aside">
    <div class="aside__tabs">
      <button :class="{ 'active': active_tab == 0 }" class="btn"
        @click="active_tab = 0; hasActiveRelative = false">Relatives</button>
      <button :class="{ 'active': active_tab == 1 }" class="btn"
        @click="active_tab = 1; hasActiveRelative = false">Females</button>
      <button :class="{ 'active': active_tab == 2 }" class="btn"
        @click="active_tab = 2; hasActiveRelative = false">Employees</button>
    </div>
    <div>
      <button class="editButton" @click="active_tab = 4; showNotes = false" v-if="hasActiveRelative">Edit</button>
      <button class="editButton" @click="active_tab = 3">new relative</button>
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
