from pydantic import BaseModel

from app.model import Comments, Posts, Replies, User


class UserReplyDTO(BaseModel):
    reply: Replies
    post: Posts
    user: User


class UserCommentDTO(BaseModel):
    comment: Comments
    reply: Replies
    post: Posts
    user: User
