import os
from sqlmodel import create_engine, Session

DATABASE_URL = os.getenv("DATABASE_URL", "postgresql://aa@localhost:5432/web_demo")

engine = create_engine(DATABASE_URL)

def get_db():
    with Session(engine) as session:
        yield session
