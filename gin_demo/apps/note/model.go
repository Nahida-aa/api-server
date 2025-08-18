package note

// 创建笔记请求
type CreateNoteRequest struct {
    Title   string `json:"title" binding:"required"`
    Content string `json:"content" binding:"required"`
}

// 更新笔记请求
type UpdateNoteRequest struct {
    Title   string `json:"title"`
    Content string `json:"content"`
}
