

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
  fileName: string
  filePath: string
  size: number
  type: string
  pinned: boolean
  createdAt: string
}

export type CreateFileParams = {
  relativeId: number
  filePath: string
}
