import os
import unittest

os.environ.setdefault(
    "DB_URL", "postgresql+asyncpg://user:pass@localhost:5432/meowarchive"
)

from app.dtos.user import UserCommentDTO, UserReplyDTO
from app.model import Comments, Posts, Replies, User, get_session
from app.router.user import user


class UserReplyDTOJoinTest(unittest.TestCase):
    def test_reply_includes_post(self):
        author = User(
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
        post = Posts(
            id=5,
            ask_help_flag=0,
            board_id=2,
            board_name="测试板块",
            content="post",
            created_at="2020-01-01T00:00:00",
            is_authorized=False,
            is_featured=False,
            is_pinned=False,
            n_comments=1,
            n_replies=1,
            n_views=0,
            title="帖子标题",
            tutorial_flag=0,
            user_id=1,
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
            post=post,
            user=author,
        )

        payload = UserReplyDTO(reply=reply, post=post, user=author).model_dump()

        self.assertEqual(payload["post"]["id"], 5)
        self.assertEqual(payload["post"]["title"], "帖子标题")
        self.assertEqual(payload["user"]["id"], 1)


class UserCommentDTOJoinTest(unittest.TestCase):
    def test_comment_includes_reply_and_post(self):
        author = User(
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
        post = Posts(
            id=5,
            ask_help_flag=0,
            board_id=2,
            board_name="测试板块",
            content="post",
            created_at="2020-01-01T00:00:00",
            is_authorized=False,
            is_featured=False,
            is_pinned=False,
            n_comments=1,
            n_replies=1,
            n_views=0,
            title="帖子标题",
            tutorial_flag=0,
            user_id=1,
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
            post=post,
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
            reply=reply,
            user=author,
        )

        payload = UserCommentDTO(
            comment=comment, reply=reply, post=post, user=author
        ).model_dump()

        self.assertEqual(payload["reply"]["id"], 10)
        self.assertEqual(payload["post"]["id"], 5)
        self.assertEqual(payload["user"]["id"], 1)


class UserRouterRouteTest(unittest.TestCase):
    def test_new_routes_exist_and_use_callable_dependency(self):
        paths = {route.path for route in user.routes if "GET" in route.methods}
        self.assertIn("/user/{id}/reply", paths)
        self.assertIn("/user/{id}/posts", paths)
        self.assertIn("/user/{id}/comments", paths)

        for route in user.routes:
            if route.path in {
                "/user/{id}/reply",
                "/user/{id}/posts",
                "/user/{id}/comments",
            }:
                dependency = route.dependant.dependencies[0]
                self.assertIs(dependency.call, get_session)


if __name__ == "__main__":
    unittest.main()
