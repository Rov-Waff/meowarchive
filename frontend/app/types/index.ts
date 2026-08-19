interface Board {
  name: string;
  n_posts: number;
  id: number;
  is_hot: boolean;
  n_discussions: number;
}
/*
 {
      "id": 2116,
      "ask_help_flag": 0,
      "board_id": 5,
      "board_name": "你问我答",
      "created_at": "2017-02-13T17:15:42",
      "is_authorized": false,
      "is_featured": false,
      "is_pinned": false,
      "n_comments": 0,
      "n_replies": 16,
      "n_views": 133,
      "title": "【求助】在线",
      "tutorial_flag": 0,
      "user_id": 194148,
      "user": {
        "nickname": "MAlopos",
        "sex": 0,
        "doing": "https://shequ.codemao.cn/wiki/novel/cover/116848\n小说求收藏",
        "level": 1,
        "collection_times": 1312,
        "praise_times": 2749,
        "id": 194148,
        "description": "我是小天狼星",
        "avatar": "https://cdn-community.codemao.cn/47/community/d2ViXzEwMDFfMTk0MTQ4XzE5NDE0OF8xNjEzMzExNDQ1ODY5XzZlYmRkN2E0.jpeg",
        "forked_times": 2999,
        "view_times": 168512
      }
*/
interface User {
  id: number;
  nickname: string;
  avatar: string;
  sex: number;
  doing: string;
  level: number;
  collection_times: number;
  praise_times: number;
  description: string;
  forked_times: number;
  view_times: number;
}

interface Post {
  id: number;
  ask_help_flag: number;
  board_id: number;
  board_name: string;
  created_at: Date;
  is_authorized: boolean;
  is_pinned: boolean;
  n_comments: number;
  n_replies: number;
  n_views: number;
  tutorial_flag: 0;
  user: User;
  title: string;
  content: string;
}

interface PageResult<T> {
  total_page: number;
  current_page: number;
  has_prev: boolean;
  has_next: boolean;
  item: Array<T>;
}

interface Reply {
  content: string;
  created_at: Date;
  is_top: boolean;
  n_likes: number;
  update_at: Date;
  id: number;
  is_liked: boolean;
  n_comments: number;
  post_id: number;
  user_id: number;
}

interface Comment {
  is_liked: boolean;
  content: string;
  reply_id: number;
  user_id: number;
  id: number;
  created_at: Date;
  n_likes: number;
  reply_user_id: number;
  user: User;
}

interface ReplyDTO {
  reply: Reply;
  comments: Array<Comment>;
  user: User;
}

interface UserReplyDTO {
  reply: Reply;
  post: Post;
  user: User;
}

interface UserCommentDTO {
  comment: Comment;
  reply: Reply;
  post: Post;
  user: User;
}

interface PostSearchItem {
  title: string;
  id: number;
  user: User;
  n_replies: number;
  n_view: number;
  n_comments: number;
}

interface CommentSearchItem {
  id: number;
  content: string;
  created_at: Date;
  n_likes: number;
  user: User;
}

interface ReplySearchItem {
  id: number;
  content: string;
  created_at: Date;
  n_likes: number;
  n_comments: number;
  user: User;
}
