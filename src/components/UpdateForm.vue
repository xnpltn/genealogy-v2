<script setup lang="ts">
import { type CreateRelativeParams } from '../utils/types';
import { reactive } from 'vue';

defineProps<{ relative: Record<string, any> }>()

const newRelative = reactive({ sameness: 0, hotness: 0, crazy: 0, employable: 0, swarthy: 0, pinned: false, sex: 'male' }) as CreateRelativeParams;

</script>

<template>

  <div class="container">
    <div class="main-container">
      <div class="form-container">
        <form class="form" @submit.prevent>
          <!-- names -->
          <div class="form-group" id="names">
            <div class="input-container">
              <label for="firstName" class="input-label">First Name</label>
              <input type="text" id="firstName" name="firstName" class="form-imput" v-model="newRelative.firstName">
            </div>
            <div class="input-container">
              <label for="middleName" class="input-label">Middle Name</label>
              <input type="text" id="middleName" name="middleName" class="form-imput" v-model="newRelative.middleName">
            </div>
            <div class="input-container">
              <label for="secondName" class="input-label">Last Name</label>
              <input type="text" id="secondName" name="secondName" class="form-imput" v-model="newRelative.lastName">
            </div>
            <div class="input-container">
              <label for="secondName" class="input-label">Sex</label>
              <select name="sex" id="sex" class="form-select" v-model="newRelative.sex">
                <option value="male">Male</option>
                <option value="female">Female</option>
              </select>
            </div>
            <div class="input-container">
              <label for="birthday" class="input-label">Birthday</label>
              <input name="birthday" id="birthday" class="form-input" v-model="newRelative.birthday">
            </div>
            <div class="input-container">
              <label for="birthday" class="input-label">Dies AT</label>
              <input name="diedAt" id="diedAt" class="form-input" v-model="newRelative.diedAt">
            </div>
          </div>

          <hr>
          <!-- contacts-->
          <div class="form-group" id="contacts">
            <div class="input-container">
              <label for="email" class="input-label">Email</label>
              <input type="text" id="email" name="email" class="form-imput" v-model="newRelative.email">
            </div>
            <div class="input-container">
              <label for="phone" class="input-label">Phone</label>
              <input type="text" id="phone" name="phone" class="form-imput" v-model="newRelative.phone">
            </div>
            <div class="input-container">
              <label for="state" class="input-label">State</label>
              <input type="text" id="state" name="state" class="form-imput" v-model="newRelative.state">
            </div>
            <div class="input-container">
              <label for="address" class="input-label">Address</label>
              <input type="text" id="address" name="address" class="form-imput" v-model="newRelative.address">
            </div>
          </div>
          <hr>

          <!-- parents -->
          <div class="form-group" id="parents">
            <div class="input-container">
              <label for="father" class="input-label">Father</label>
              <select name="father" id="father" class="form-select" v-model="newRelative.fatherId">
                <option value="1">Father 1</option>
                <option value="2">Father 2</option>
              </select>
            </div>
            <div class="input-container">
              <label for="father" class="input-label">Mother</label>
              <select name="mother" id="mother" class="form-select" v-model="newRelative.motherId">
                <option value="1">Mother 1</option>
                <option value="2">Mother 2</option>
              </select>
            </div>
          </div>

          <!-- additional fields -->
          <div class="form-group" id="addition">
            <div class="input-container" v-if="newRelative.sex == 'male'">
              <label for="employable" class="input-label">Employable</label>
              <input type="text" id="employable" name="employable" class="form-imput" v-model="newRelative.employable">
            </div>
            <div class="input-container" v-if="newRelative.sex == 'female'">
              <label for="employable" class="input-label">Swarthy</label>
              <input type="text" id="swarthy" name="swarthy" class="form-imput" v-model="newRelative.swarthy">
            </div>
            <div class="input-container" v-if="newRelative.sex == 'female'">
              <label for="hotness" class="input-label">Hotness</label>
              <input type="text" id="hotness" name="hotness" class="form-imput" v-model="newRelative.hotness">
            </div>
            <div class="input-container" v-if="newRelative.sex == 'female'">
              <label for="crazy" class="input-label">Crazy</label>
              <input type="text" id="crazy" name="creazy" class="form-imput" v-model="newRelative.crazy">
            </div>
            <div class="input-container">
              <label for="lostReason" class="input-label">Lost Reason</label>
              <select name="lostReason" id="lostReason" class="form-select" v-model="newRelative.lostReason">
                <option value="1">Reason 1</option>
                <option value="2">Reason 2</option>
              </select>
            </div>
          </div>
          <div class="form-group" id="pinned">
            <div class="input-container">
              <label for="pinned" class="input-label">Sameness</label>
              <input type="text" id="sameness" name="sameness" class="form-imput" v-model="newRelative.sameness">
            </div>
            <div class="input-container">
              <label for="pinned" class="input-label">Pinned</label>
              <input type="checkbox" id="pinned" name="pinned" class="form-imput" v-model="newRelative.pinned">
            </div>
          </div>
          <div class="form-group" id="button">
            <button type="submit">Submit</button>
          </div>
        </form>
      </div>
      <div class="meta-container">
        <div class="meta-card">
          <h1>hello wolrd</h1>
          <h1>hi world</h1>
        </div>
      </div>
    </div>
    <div class="image-container">
      <div class="image-section">
        image

      </div>
      <div class="table-section">
        table

      </div>
    </div>
  </div>

</template>



<style scoped>
.container {
  padding: var(--size-sm);
  display: grid;
  grid-template-columns: 2fr 1fr;
}

.main-container {
  border: 1px solid red;
}


.form-container {
  border: apx solid orange;
  padding: var(--size-sm);

}

.form {
  display: grid;
  gap: var(--size-sm);
}

.form-group {
  border: 1px solid red;
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: var(--size-sm);
}

.input-container {
  display: flex;
  flex-direction: column;
  gap: 0.075rem;
}

.meta-container {
  display: grid;
  grid-template-rows: 2fr 1fr;
  border: 1px solid green;
  padding: var(--size-sm);
}

.image-container {
  border: 1px solid blue;
  display: grid;
}

.image-section {
  border: 1px solid magenta;
}
</style>
