<script setup lang="ts">

defineProps<{ relative: Record<string, any> }>()

</script>

<template>
  <div>
    <form @submit.prevent id="createRelativeForm">
      <!-- Required Fields -->
      <label for="firstName">First Name:</label>
      <input type="text" v-model="relative.firstName" id="firstName" name="firstName" required>

      <label for="lastName">Last Name:</label>
      <input type="text" v-model="relative.lastName" id="lastName" name="lastName" required>

      <label for="pinned">Pinned:</label>
      <input type="checkbox" v-model="relative.pinned" id="pinned" name="pinned">

      <!-- Optional Fields -->
      <label for="middleName">Middle Name:</label>
      <input type="text" v-model="relative.middleName" id="middleName" name="middleName">

      <label for="sex">Sex:</label>
      <select name="sex" id="sex" v-model="relative.sex">
        <option value="male">Male</option>
        <option value="female">Female</option>
      </select>

      <label for="birthday">Birthday:</label>

      <input type="date" v-model="relative.birthday" id="birthday" name="birthday">

      <label for="sameness">Sameness:</label>
      <input type="number" id="sameness" v-model="relative.sameness" name="sameness" step="1" min="0" max="10"
        value="0.0">

      <label for="mother">Mother:</label>
      <input type="text" id="mother" v-model="relative.mother" name="mother">

      <label for="father">Father:</label>
      <input type="text" id="father" v-model="relative.father" name="father">

      <label for="phone">Phone:</label>
      <input type="tel" id="phone" v-model="relative.phone" name="phone">

      <label for="email">Email:</label>
      <input type="email" id="email" v-model="relative.email" name="email">

      <label for="lostReason">Lost Reason:</label>
      <textarea id="lostReason" name="lostReason" v-model="relative.lostReason"></textarea>

      <label for="swarthy">Swarthy:</label>
      <input type="number" id="swarthy" name="swarthy" v-model="relative.swarthy" step="1" min="0" max="10" value="0">

      <label for="hotness">Hotness:</label>
      <input type="number" id="hotness" name="hotness" v-model="relative.hotness" step="1" min="0" max="10" value="0">

      <label for="crazy">Crazy:</label>
      <input type="number" id="crazy" name="crazy" v-model="relative.crazy" step="1" min="0" max="10" value="0">

      <label for="employable">Employable:</label>
      <input type="number" id="employable" v-model="relative.employable" name="employable" step="1" min="0" max="10"
        value="0">
      <button type="submit">Submit</button>
    </form>
  </div>
</template>
