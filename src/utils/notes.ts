/*


pub struct CreateNoteParams {
    pub relative_id: u8,
    pub text: String,
    pub pinned: bool,
}


pub struct Note {
    pub id: u8,
    pub text: String,
    pub pinned: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub struct UpdateNoteParams {
    pub id: u8,
    pub text: String,
    pub pinned: bool,
}
*/


export type CreateNoteParams = {
  relative_id: number
  text: string
  pinned: boolean
}


export type Note = {
  id: number
  text: string
  pinned: boolean
  created_at: string
  updated_at: string
}

export type UpdateNoteParams = {
  id: number
  text: string
  pinned: boolean
}
