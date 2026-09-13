interface Board {
  id: number;
  name: string | null;
  description: string | null;
  is_hot: boolean | null;
  n_posts: number | null;
  n_discussions: number | null;
}

interface User {
  id: number;
  nickname: string | null;
  avatar: string | null;
  sex: number | null;
  doing: string | null;
  level: number | null;
  collection_times: number | null;
  praise_times: number | null;
  description: string | null;
  forked_times: number | null;
  view_times: number | null;
}

interface Post {
  id: number;
  ask_help_flag: number | null;
  board_id: number | null;
  board_name: string | null;
  content: string | null;
  created_at: string | null;
  is_authorized: boolean | null;
  is_featured: boolean | null;
  is_pinned: boolean | null;
  n_comments: number | null;
  n_replies: number | null;
  n_views: number | null;
  title: string | null;
  tutorial_flag: number | null;
  updated_at: string | null;
  user_id: number | null;
}

interface PostDetail {
  post: Post;
  user: User;
}

interface PageResult<T> {
  current: number;
  total: number;
  has_prev: boolean;
  has_next: boolean;
  item: Array<T>;
}

interface Reply {
  id: number;
  content: string | null;
  created_at: string | null;
  is_liked: boolean | null;
  is_top: boolean | null;
  n_comments: number | null;
  n_likes: number | null;
  post_id: number | null;
  update_at: string | null;
  user_id: number | null;
}

interface Comment {
  id: number;
  content: string | null;
  created_at: string | null;
  is_liked: boolean | null;
  n_likes: number | null;
  reply_id: number | null;
  reply_user_id: number | null;
  user_id: number | null;
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

interface Count {
  posts: number;
  replies: number;
  comments: number;
  user: number;
}

interface PostSearchItem {
  id: number;
  title: string | null;
  user: User;
  n_replies: number | null;
  n_view: number | null;
  n_comments: number | null;
}

interface CommentSearchItem {
  id: number;
  content: string | null;
  created_at: string | null;
  n_likes: number | null;
  user: User;
}

interface ReplySearchItem {
  id: number;
  content: string | null;
  created_at: string | null;
  n_likes: number | null;
  n_comments: number | null;
  user: User;
}
