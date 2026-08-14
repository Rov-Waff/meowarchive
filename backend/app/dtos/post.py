from datetime import datetime

from pydantic import BaseModel

from app.model import Comments, Replies, User


class PostDetailsDTO(BaseModel):
    ask_help_flag:int
    board_id:int
    content:str
    is_authorized:bool
    is_pinned:bool
    n_replies:int
    title:str
    user:User
    id:int
    board_name:str
    created_at:datetime
    is_featured:bool
    n_comments:int
    n_views:int
    tutorial_flag:int

class PostRepliesDTO(BaseModel):
    reply:Replies
    user:User
    comments:list[Comments]