export type RelativeIndividual = {
  id: number;
  name: string;
  age?: number | null;
  sameness?: number | null;
  mother?: string | null;
  father?: string | null;
  phone?: string | null;
  email?: string | null;
  pinned: boolean;
  lostReason?: string | null;
  createdAt?: string | null;
  updatedAt?: string | null;
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
