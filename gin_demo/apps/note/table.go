package note

import (
    "github.com/google/uuid"
)

type Note struct {
    ID        uuid.UUID `gorm:"type:uuid;default:uuid_generate_v4();primaryKey" json:"id"`
    Title     string    `gorm:"size:255;not null" json:"title"`
    Content   string    `gorm:"type:text;not null" json:"content"`
}
