import uuid
from sqlmodel import SQLModel, Field
from typing import Optional

class Note(SQLModel, table=True):
    id: Optional[uuid.UUID] = Field(default_factory=uuid.uuid4, primary_key=True)
    title: str = Field(max_length=255)
    content: str

# 输入输出模型
class NoteInsert(SQLModel):
    title: str
    content: str

class NoteUpdate(SQLModel):
    title: Optional[str] = None
    content: Optional[str] = None

# Note 类既是数据表模型也是响请求响应模型
