import uuid
from sqlmodel import Session, select
from apps.note.table import Note, NoteInsert, NoteUpdate

class NoteService:
    def __init__(self, db: Session):
        self.db = db
    
    def get_all_notes(self):
        statement = select(Note)
        return self.db.exec(statement).all()
    
    def get_note_by_id(self, note_id: uuid.UUID):
        return self.db.get(Note, note_id)
    
    def create_note(self, note_data: NoteInsert):
        db_note = Note(title=note_data.title, content=note_data.content)
        self.db.add(db_note)
        self.db.commit()
        self.db.refresh(db_note)
        return db_note
    
    def update_note(self, note_id: uuid.UUID, note_data: NoteUpdate):
        db_note = self.get_note_by_id(note_id)
        if db_note:
            update_data = note_data.model_dump(exclude_unset=True)
            for field, value in update_data.items():
                setattr(db_note, field, value)
            self.db.add(db_note)
            self.db.commit()
            self.db.refresh(db_note)
        return db_note
    
    def delete_note(self, note_id: uuid.UUID):
        db_note = self.get_note_by_id(note_id)
        if db_note:
            self.db.delete(db_note)
            self.db.commit()
        return db_note
