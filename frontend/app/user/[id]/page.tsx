import Link from "next/link";
import UserContentComponent from "../UserContentComponent";
import { API_BASE } from "@/lib/api";

const UserDetail = async ({
  params,
  searchParams,
}: {
  params: Promise<{ id: number }>;
  searchParams: Promise<{ tab?: string; page?: string }>;
}) => {
  const { id } = await params;
  const { tab = "reply", page = "1" } = await searchParams;
  const activeTab = tab === "posts" || tab === "comments" ? tab : "reply";

  const userData: User = await (await fetch(`${API_BASE}/user/` + id)).json();

  let replyData: PageResult<UserReplyDTO> | null = null;
  let postData: PageResult<Post> | null = null;
  let commentData: PageResult<UserCommentDTO> | null = null;

  if (activeTab === "posts") {
    postData = await (
      await fetch(`${API_BASE}/user/${id}/posts?page_size=30&page_num=${page}`)
    ).json();
  } else if (activeTab === "comments") {
    commentData = await (
      await fetch(
        `${API_BASE}/user/${id}/comments?page_size=30&page_num=${page}`,
      )
    ).json();
  } else {
    replyData = await (
      await fetch(`${API_BASE}/user/${id}/reply?page_size=30&page_num=${page}`)
    ).json();
  }

  const activeData =
    activeTab === "posts"
      ? postData
      : activeTab === "comments"
        ? commentData
        : replyData;

  const tabItems = [
    { key: "reply", label: "回复" },
    { key: "posts", label: "帖子" },
    { key: "comments", label: "评论" },
  ];

  return (
    <>
      <div className="bg-white border border-gray-200 rounded-[10px] px-5 py-4 my-3 shadow-sm">
        <div className="flex items-center gap-4">
          <img
            src={userData.avatar}
            alt={userData.nickname}
            className="w-20 h-20 rounded-full object-cover bg-gray-100"
          />
          <div className="min-w-0">
            <h2 className="text-[1.4rem] font-semibold">{userData.nickname}</h2>
            <p className="text-sm text-gray-500">Lv.{userData.level}</p>
            <p className="text-sm text-gray-600 mt-1">{userData.description}</p>
          </div>
        </div>
        <div className="flex flex-wrap gap-x-6 gap-y-1 mt-4 text-sm text-gray-500">
          <span>获赞:{userData.praise_times}</span>
          <span>收藏:{userData.collection_times}</span>
          <span>浏览:{userData.view_times}</span>
          <span>作品收藏:{userData.forked_times}</span>
        </div>
        {userData.doing ? (
          <p className="text-xs text-gray-400 mt-2">签名:{userData.doing}</p>
        ) : (
          <></>
        )}
      </div>

      <div className="flex gap-2 my-3">
        {tabItems.map((item) => {
          const active = activeTab === item.key;
          return (
            <Link
              key={item.key}
              href={`/user/${id}?tab=${item.key}`}
              className={
                active
                  ? "px-4 py-1.5 rounded-md bg-blue-600 text-white text-sm font-medium"
                  : "px-4 py-1.5 rounded-md bg-white border border-gray-300 text-gray-600 text-sm transition hover:border-blue-600 hover:text-blue-600"
              }
            >
              {item.label}
            </Link>
          );
        })}
      </div>

      {activeData ? (
        <>
          <UserContentComponent
            current={activeData.current_page}
            totalPage={activeData.total_page}
            base={`/user/${id}?tab=${activeTab}`}
          />
          {activeData.item.length === 0 ? (
            <p className="text-sm text-gray-500">暂无内容</p>
          ) : (
            <></>
          )}
        </>
      ) : (
        <></>
      )}

      {activeTab === "posts" && postData ? (
        postData.item.map((item) => {
          return (
            <div
              key={item.id}
              className="bg-white border border-gray-200 rounded-[10px] px-4 py-3 my-3 shadow-sm transition hover:shadow-md hover:-translate-y-px"
            >
              <Link
                href={`/post/${item.id}`}
                className="font-semibold text-gray-800 hover:underline"
              >
                {item.title}
              </Link>
              <p className="mt-1 text-sm text-gray-500">
                ID:{item.id} 阅读:{item.n_views} 评论:{item.n_comments} 回复:
                {item.n_replies}
              </p>
              <p className="mt-1 text-sm text-gray-500">
                {item.is_authorized ? <>官方</> : <></>}{" "}
                {item.is_pinned ? <>置顶</> : <></>}
                {item.ask_help_flag ? <>求助</> : <></>}{" "}
                {item.tutorial_flag ? <>教程</> : <></>}
              </p>
            </div>
          );
        })
      ) : (
        <></>
      )}

      {activeTab === "comments" && commentData ? (
        commentData.item.map((item) => {
          return (
            <div
              key={item.comment.id}
              className="bg-white border border-gray-200 rounded-[10px] px-4 py-3 my-3 shadow-sm"
            >
              <p className="text-sm text-gray-400">
                在{" "}
                <Link
                  href={`/post/${item.post.id}`}
                  className="text-blue-600 hover:underline"
                >
                  {item.post.title}
                </Link>{" "}
                下评论
              </p>
              <p
                className="mt-1 break-words"
                dangerouslySetInnerHTML={{ __html: item.comment.content }}
              />
              <p className="text-xs text-gray-400 mt-1">
                {item.comment.created_at.toString()}
              </p>
            </div>
          );
        })
      ) : (
        <></>
      )}

      {activeTab === "reply" && replyData ? (
        replyData.item.map((item) => {
          return (
            <div
              key={item.reply.id}
              className="bg-white border border-gray-200 rounded-[10px] px-4 py-3 my-3 shadow-sm"
            >
              <p className="text-sm text-gray-400">
                在{" "}
                <Link
                  href={`/post/${item.post.id}`}
                  className="text-blue-600 hover:underline"
                >
                  {item.post.title}
                </Link>{" "}
                中回复
              </p>
              <p
                className="mt-1 break-words"
                dangerouslySetInnerHTML={{ __html: item.reply.content }}
              />
              <p className="text-xs text-gray-400 mt-1">
                {item.reply.created_at.toString()} 点赞:{item.reply.n_likes}
              </p>
            </div>
          );
        })
      ) : (
        <></>
      )}
    </>
  );
};

export default UserDetail;
