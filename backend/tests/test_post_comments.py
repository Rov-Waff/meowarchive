import os
import unittest

os.environ.setdefault("DB_URL", "postgresql+asyncpg://user:pass@localhost:5432/meowarchive")

from app.model import Comments, Replies, User
from app.dtos.post import CommentDTO, PostRepliesDTO


class PostCommentsUserFieldTest(unittest.TestCase):
    def test_comments_include_user(self):
        user = User(
            id=1,
            nickname="alice",
            sex=0,
            description="",
            doing="",
            level=1,
            avatar="",
            collection_times=0,
            forked_times=0,
            praise_times=0,
            view_times=0,
        )
        reply = Replies(
            id=10,
            content="reply",
            created_at="2020-01-01T00:00:00",
            is_liked=False,
            is_top=False,
            n_comments=1,
            n_likes=0,
            post_id=5,
            update_at="2020-01-01T00:00:00",
            user_id=1,
            user=user,
        )
        comment = Comments(
            id=20,
            content="comment",
            created_at="2020-01-01T00:00:00",
            is_liked=False,
            n_likes=0,
            reply_id=10,
            reply_user_id=1,
            user_id=1,
            user=user,
        )

        payload = PostRepliesDTO(
            reply=reply,
            user=user,
            comments=[CommentDTO(**comment.model_dump(), user=comment.user)],
        )

        self.assertIn("user", payload.model_dump()["comments"][0])
        self.assertEqual(payload.model_dump()["comments"][0]["user"]["id"], 1)


if __name__ == "__main__":
    unittest.main()
