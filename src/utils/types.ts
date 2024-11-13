export type Relative = {
  id: number
  name: string;
  age: number | null;
  sameness: number | null;
  mother: string | null;
  father: string | null;
  phone: string | null;
  email: string | null;
  pinned: boolean;
  lostReason: string | null;
  created: Date | null;
  updated: Date | null;
}

export type Female = {
  id: number
  name: string;
  age: number | null;
  sameness: number | null;
  swarthy: number | null
  hotness: number | null
  crazy: number | null
  mother: string | null;
  father: string | null;
  phone: string | null;
  email: string | null;
  pinned: boolean;
  lostReason: string | null;
  created: Date | null;
  updated: Date | null;
}

export type Employee = {
  id: number
  name: string;
  age: number | null;
  sameness: number | null;
  employable: number | null
  mother: string | null;
  father: string | null;
  phone: string | null;
  email: string | null;
  pinned: boolean;
  lostReason: string | null;
  created: Date | null;
  updated: Date | null;
}
