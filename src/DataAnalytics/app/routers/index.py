from fastapi import APIRouter

from app.processing import collect_usage

# from app import ModelCRUD, Schema

router = APIRouter()


@router.get("/")
async def get_by_id() -> None:
    print("invoke")
    await collect_usage()
