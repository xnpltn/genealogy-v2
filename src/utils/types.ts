export type RelativeIndividual = {
  sex: string,
  birthday: string | null,
  diedAt: string | null,
  firstName: string;
  middleName: string | null;
  lastName: string;
  id: number;
  name: string;
  age: number | null;
  sameness: number | null;
  swarthy: number | null;
  hotness: number | null;
  crazy: number | null;
  employable: number | null;
  mother: string | null;
  father: string | null;
  phone: string | null;
  email: string | null;
  pinned: boolean;
  lostReason: string | null;
  address: string | null;
  state: string | null
  createdAt: string | null;
  updatedAt: string | null;
}

export type FemaleIndividual = {
  id: number;
  name: string;
  age?: number | null;
  sameness?: number | null;
  swarthy?: number | null;
  hotness?: number | null;
  crazy?: number | null;
  mother?: string | null;
  father?: string | null;
  phone?: string | null;
  email?: string | null;
  pinned: boolean;
  createdAt?: string | null;
  updatedAt?: string | null;
}

export type EmployeeIndividual = {
  id: number;
  name: string;
  age?: number | null;
  sameness?: number | null;
  employable?: number | null;
  mother?: string | null;
  father?: string | null;
  phone?: string | null;
  email?: string | null;
  pinned: boolean;
  createdAt?: string | null;
  updatedAt?: string | null;
}


/*
    .bind(new_relative.sameness)
    .bind(new_relative.lost_reason)
    .bind(new_relative.sex)
    .bind(new_relative.birthday)
    .bind(new_relative.died_at)
    .bind(new_relative.first_name)
    .bind(new_relative.middle_name)
    .bind(new_relative.last_name)
    .bind(new_relative.phone)
    .bind(new_relative.email)
    .bind(new_relative.pinned)
    .bind(new_relative.hotness)
    .bind(new_relative.crazy)
    .bind(new_relative.swarthy)
    .bind(new_relative.employable)
    .bind(new_relative.mother_id)
    .bind(new_relative.father_id)
    .bind(new_relative.state)
    .bind(new_relative.address)
 */
export type CreateRelativeParams = {
  firstName: string;
  middleName: string;
  lastName: string;
  sex: string;
  birthday: string | null;
  diedAt: string | null
  sameness: number | null;
  motherId: number | null;
  fatherId: number | null;
  phone: string | null;
  email: string | null;
  pinned: boolean;
  lostReason: string | null;
  swarthy: number | null;
  hotness: number | null;
  crazy: number | null;
  employable: number | null;
  state: string | null
  address: string | null
}


export type UpdateRelativeParams = {
  id: number;
  firstName: string;
  middleName?: string;
  lastName: string;
  sex: string;
  birthday?: string | null;
  sameness?: number | null;
  mother?: string | null;
  father?: string | null;
  phone?: string | null;
  email?: string | null;
  pinned: boolean;
  lostReason?: string | null;
  swarthy?: number | null;
  hotness?: number | null;
  crazy?: number | null;
  employable?: number | null;
}
