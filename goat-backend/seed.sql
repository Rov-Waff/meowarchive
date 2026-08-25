-- 测试数据 seed.sql
-- 用法: psql -U <user> -d codemao_forum -f seed.sql
-- 适用于 goat-backend(axum + sea-orm) 与前端联调。

-- 清空旧数据（注意外键顺序）
truncate table comments cascade;
truncate table replies cascade;
truncate table posts cascade;
truncate table board cascade;
truncate table "user" cascade;

-- ============ 用户 ============
insert into "user" (id, nickname, sex, description, doing, level, avatar, collection_times, forked_times, praise_times, view_times)
values
    (1, '喵小美', 0, '一只爱编程的猫', '努力学 Rust 中', 3,
     'https://cdn-community.codemao.cn/47/community/d2ViXzEwMDFfMTk0MTQ4XzE5NDE0OF8xNjEzMzExNDQ1ODY5XzZlYmRkN2E0.jpeg',
     12, 3, 45, 1200),
    (2, '代码猫王', 1, '社区老人', '写个会说话的猫', 5,
     'https://cdn-community.codemao.cn/47/community/d2ViXzEwMDFfMTkyMjIyXzE5MjIyMl8xNjEzMzExNDQ1ODY5XzZhYmMxZWEy.jpeg',
     88, 20, 320, 56000),
    (3, '阿伟', 1, '萌新求带', '请大家多多指教', 1,
     'https://cdn-community.codemao.cn/47/community/d2ViXzEwMDFfMTkxMTExXzE5MTExMV8xNjEzMzExNDQ1ODY5XzRkY2NmMjBl.jpeg',
     2, 0, 8, 300),
    (4, '小白猫', 0, '画画猫', '喜欢做小游戏', 2,
     'https://cdn-community.codemao.cn/47/community/d2ViXzEwMDFfMTkwMDAwXzE5MDAwMF8xNjEzMzExNDQ1ODY5XzFhMmIzYzRk.jpeg',
     30, 15, 150, 8900);

-- ============ 板块 ============
insert into board (id, name, description, is_hot, n_posts, n_discussions)
values
    (1, '你问我答', '大家互帮互助的地方', true, 2, 1),
    (2, '作品展示', '晒出你的作品', true, 1, 0),
    (3, '闲聊灌水', '随便聊聊', false, 0, 0);

-- ============ 帖子 ============
insert into posts (id, ask_help_flag, board_id, board_name, content, created_at, is_authorized, is_featured, is_pinned, n_comments, n_replies, n_views, title, tutorial_flag, updated_at, user_id)
values
    (101, 1, 1, '你问我答', '<p>我写了一个 Rust 程序总是报 borrow checker 的错误，请问大家怎么处理生命周期？</p>',
     now() - interval '3 day', false, false, false, 2, 2, 130, '【求助】Rust 生命周期问题', 0, now() - interval '3 day', 1),
    (102, 0, 1, '你问我答', '<p>分享一个用 Python 做爬虫的小技巧，抓取静态网页用 requests 就够啦。</p>',
     now() - interval '2 day', true, true, true, 1, 1, 250, 'Python 爬虫入门小技巧', 1, now() - interval '2 day', 2),
    (103, 0, 2, '作品展示', '<p>这是我做的贪吃蛇小游戏，大家看看怎么样？</p>',
     now() - interval '1 day', false, false, false, 0, 0, 60, '自制贪吃蛇小游戏', 0, now() - interval '1 day', 4);

-- ============ 回复 ============
insert into replies (id, content, created_at, is_liked, is_top, n_comments, n_likes, post_id, update_at, user_id)
values
    (201, '<p>生命周期问题一般是作用域没搞清楚，建议先把借用规则记牢。</p>',
     now() - interval '2 day', false, true, 1, 3, 101, now() - interval '2 day', 2),
    (202, '<p>可以试试把引用换成所有权，或者用智能指针 Rc。</p>',
     now() - interval '1 day', false, false, 0, 1, 101, now() - interval '1 day', 3),
    (203, '<p>感谢分享，收藏了！</p>',
     now() - interval '1 day', false, false, 0, 2, 102, now() - interval '1 day', 1);

-- ============ 评论 ============
insert into comments (id, content, created_at, is_liked, n_likes, reply_id, reply_user_id, user_id)
values
    (301, '讲得太清楚了，谢谢大佬！', now() - interval '1 day', false, 1, 201, 2, 1),
    (302, '学习了，下次也试试。', now() - interval '12 hour', false, 0, 201, 2, 4),
    (303, '好耶，这就去试。', now() - interval '6 hour', false, 0, 203, 1, 3);

-- 重置自增序列，保证后续插入 id 不冲突
select setval(pg_get_serial_sequence('"user"', 'id'), (select coalesce(max(id), 1) from "user"));
select setval(pg_get_serial_sequence('board', 'id'), (select coalesce(max(id), 1) from board));
select setval(pg_get_serial_sequence('posts', 'id'), (select coalesce(max(id), 1) from posts));
select setval(pg_get_serial_sequence('replies', 'id'), (select coalesce(max(id), 1) from replies));
select setval(pg_get_serial_sequence('comments', 'id'), (select coalesce(max(id), 1) from comments));
