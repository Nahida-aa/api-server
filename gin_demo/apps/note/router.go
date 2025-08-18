package note

import (
    "net/http"

    "github.com/gin-gonic/gin"
    "github.com/google/uuid"
)

// RegisterNoteRoutes 注册笔记 API
func RegisterNoteRoutes(r *gin.Engine, service *NoteService) {
    noteGroup := r.Group("/notes")
    {
        // 创建
        // @Summary 创建笔记
        // @Tags Notes
        // @Accept json
        // @Produce json
        // @Param note body CreateNoteRequest true "笔记信息"
        // @Success 200 {object} Note
        // @Router /notes [post]
        noteGroup.POST("", func(c *gin.Context) {
            var req CreateNoteRequest
            if err := c.ShouldBindJSON(&req); err != nil {
                c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
                return
            }
            note := Note{ID: uuid.New(), Title: req.Title, Content: req.Content}
            if err := service.CreateNote(&note); err != nil {
                c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
                return
            }
            c.JSON(http.StatusOK, note)
        })

        // 查询所有
        // @Summary 获取笔记列表
        // @Tags Notes
        // @Produce json
        // @Success 200 {array} Note
        // @Router /notes [get]
        noteGroup.GET("", func(c *gin.Context) {
            notes, err := service.ListNotes()
            if err != nil {
                c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
                return
            }
            c.JSON(http.StatusOK, notes)
        })

        // 查询单个
        // @Summary 获取单个笔记
        // @Tags Notes
        // @Produce json
        // @Param id path string true "笔记ID"
        // @Success 200 {object} Note
        // @Router /notes/{id} [get]
        noteGroup.GET("/:id", func(c *gin.Context) {
            id := c.Param("id")
            note, err := service.GetNoteByID(id)
            if err != nil {
                c.JSON(http.StatusNotFound, gin.H{"error": "Note not found"})
                return
            }
            c.JSON(http.StatusOK, note)
        })

        // 更新
        // @Summary 更新笔记
        // @Tags Notes
        // @Accept json
        // @Produce json
        // @Param id path string true "笔记ID"
        // @Param note body UpdateNoteRequest true "更新数据"
        // @Success 200 {string} string "ok"
        // @Router /notes/{id} [put]
        noteGroup.PUT("/:id", func(c *gin.Context) {
            id := c.Param("id")
            var req UpdateNoteRequest
            if err := c.ShouldBindJSON(&req); err != nil {
                c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
                return
            }
            if err := service.UpdateNote(id, req); err != nil {
                c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
                return
            }
            c.JSON(http.StatusOK, gin.H{"status": "ok"})
        })

        // 删除
        // @Summary 删除笔记
        // @Tags Notes
        // @Param id path string true "笔记ID"
        // @Success 200 {string} string "deleted"
        // @Router /notes/{id} [delete]
        noteGroup.DELETE("/:id", func(c *gin.Context) {
            id := c.Param("id")
            if err := service.DeleteNote(id); err != nil {
                c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
                return
            }
            c.JSON(http.StatusOK, gin.H{"status": "deleted"})
        })
    }
}
