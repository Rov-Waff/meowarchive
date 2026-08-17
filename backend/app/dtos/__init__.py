from pydantic import BaseModel


class PageResult[T](BaseModel):
    total_page: int
    current_page: int
    has_prev: bool
    has_next: bool
    item: list[T]
