from typing import Annotated
from fastapi import Depends, FastAPI
from sqlmodel import SQLModel
from apps.note import router as note_router
from apps.database import engine

# 创建数据库表
SQLModel.metadata.create_all(engine)

app = FastAPI(
    title="Note API",
    description="Note management API with FastAPI",
    version="1.0.0",
    openapi_tags=[
        {
            "name": "Note",
            "description": "Note management operations"
        }
    ]
)

@app.get("/")
async def root():
    return {"message": "Hello World"}

app.include_router(note_router.router)

