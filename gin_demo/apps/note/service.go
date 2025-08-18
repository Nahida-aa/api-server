package note

import (
    "gorm.io/gorm"
)

type NoteService struct {
    DB *gorm.DB
}

func (s *NoteService) CreateNote(note *Note) error {
    return s.DB.Create(note).Error
}

func (s *NoteService) GetNoteByID(id string) (*Note, error) {
    var note Note
    err := s.DB.First(&note, "id = ?", id).Error
    return &note, err
}

func (s *NoteService) ListNotes() ([]Note, error) {
    var notes []Note
    err := s.DB.Find(&notes).Error
    return notes, err
}

func (s *NoteService) UpdateNote(id string, data UpdateNoteRequest) error {
    return s.DB.Model(&Note{}).Where("id = ?", id).Updates(data).Error
}

func (s *NoteService) DeleteNote(id string) error {
    return s.DB.Delete(&Note{}, "id = ?", id).Error
}
