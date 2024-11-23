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
  city: string | null;
  zipcode: string | null;
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
  city: string | null;
  zipcode: string | null;
}


export type UpdateRelativeParams = {
  id: number;
  firstName: string;
  middleName: string;
  lastName: string;
  sex: string;
  birthday: string | null;
  sameness: number | null;
  mother: string | null;
  father: string | null;
  phone: string | null;
  email: string | null;
  pinned: boolean;
  lostReason: string | null;
  swarthy: number | null;
  hotness: number | null;
  crazy: number | null;
  employable: number | null;
  city: string | null;
  zipcode: string | null;
}
