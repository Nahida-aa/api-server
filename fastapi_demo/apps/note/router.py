import uuid
from typing import List
from fastapi import APIRouter, Depends, HTTPException
from sqlmodel import Session
from apps.database import get_db
from apps.note.table import Note, NoteInsert, NoteUpdate
from apps.note.service import NoteService

router = APIRouter(prefix="/note", tags=["Note"])

@router.get("/", response_model=List[Note])
async def get_all_notes(db: Session = Depends(get_db)):
    service = NoteService(db)
    return service.get_all_notes()

@router.put("/", response_model=Note)
async def create_note(note_data: NoteInsert, db: Session = Depends(get_db)):
    service = NoteService(db)
    return service.create_note(note_data)

@router.get("/{note_id}", response_model=Note)
async def get_note(note_id: uuid.UUID, db: Session = Depends(get_db)):
    service = NoteService(db)
    note_obj = service.get_note_by_id(note_id)
    if not note_obj:
        raise HTTPException(status_code=404, detail="Note not found")
    return note_obj

@router.patch("/{note_id}", response_model=Note)
async def update_note(note_id: uuid.UUID, note_data: NoteUpdate, db: Session = Depends(get_db)):
    service = NoteService(db)
    note_obj = service.update_note(note_id, note_data)
    if not note_obj:
        raise HTTPException(status_code=404, detail="Note not found")
    return note_obj

@router.delete("/{note_id}", response_model=Note)
async def delete_note(note_id: uuid.UUID, db: Session = Depends(get_db)):
    service = NoteService(db)
    note_obj = service.delete_note(note_id)
    if not note_obj:
        raise HTTPException(status_code=404, detail="Note not found")
    return note_obj
