

/*


pub struct File {
    pub id: u8,
    pub file_name: String,
    pub file_path: String,
    pub size: u32,
    pub r#type: String,
    pub pinned: bool,
    pub created_at: String,
}

#[derive(Debug, FromRow, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreateFileParams {
    pub relative_id: u8,
    pub file_path: String,
}

*/



export type File = {
  id: number
  file_name: string
  file_path: string
  size: number
  type: string
  pinned: boolean
  created_at: string
}

export type CreateFileParams = {
  relativeId: number
  filePath: string
}
