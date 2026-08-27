import Link from "next/link";
import RepliesPostComponent from "../RepliesPostComponent";
import { API_BASE } from "@/lib/api";

const PostDetail = async ({
  params,
  searchParams,
}: {
  params: Promise<{ id: number }>;
  searchParams: Promise<{ page: number }>;
}) => {
  const { id } = await params;
  const { page } = await searchParams;
  const postDetail: PostDetail = await (
    await fetch(`${API_BASE}/post/` + id)
  ).json();
  const replyData: PageResult<ReplyDTO> = await (
    await fetch(`${API_BASE}/post/${id}/replies?size=30&page=1`)
  ).json();
  const { post: postData, user: postAuthor } = postDetail;
  return (
    <>
      <h2 className="text-[1.4rem] my-2">{postData.title ?? ""}</h2>
      <p className="text-sm text-gray-500 flex items-center flex-wrap gap-x-1.5">
        用户:
        <img
          src={postAuthor.avatar ?? ""}
          alt={postAuthor.nickname ?? ""}
          className="w-5 h-5 rounded-full object-cover bg-gray-100"
        />
        <Link
          href={`/user/${postAuthor.id}`}
          className="text-blue-600 hover:underline"
        >
          {postAuthor.nickname ?? ""}
        </Link>
        查看:{postData.n_views ?? 0} 回复:{postData.n_replies ?? 0} 评论:
        {postData.n_comments ?? 0} 创建时间:{postData.created_at?.toString() ?? ""}
      </p>
      <hr className="border-0 border-t border-gray-200 my-3.5" />
      <div
        className="bg-white border border-gray-200 rounded-[10px] px-5 py-4 my-3 leading-7 break-words [&_p]:my-2 [&_img]:rounded-lg"
        dangerouslySetInnerHTML={{ __html: postData.content ?? "" }}
      />
      <hr className="border-0 border-t border-gray-200 my-3.5" />
      <h3 className="text-lg mt-4 mb-2">回复</h3>
      <RepliesPostComponent
        current={replyData.current}
        totalPage={replyData.total}
        id={id}
      />
      {replyData.item.map((item) => {
        return (
          <div
            className="border p-1 m-1 border-gray-300 bg-white rounded-[10px] shadow-sm"
            key={item.reply.id}
          >
            <span className="flex items-center gap-1.5">
              <img
                src={item.user.avatar ?? ""}
                alt={item.user.nickname ?? ""}
                className="w-6 h-6 rounded-full object-cover bg-gray-100"
              />
              <Link
                href={`/user/${item.user.id}`}
                className="text-blue-600 hover:underline"
              >
                {item.user.nickname ?? ""}
              </Link>
            </span>
            <p
              dangerouslySetInnerHTML={{ __html: item.reply.content ?? "" }}
            ></p>
            <p>
              {item.reply.is_liked ? <>置顶</> : <></>} 点赞
              {item.reply.n_likes ?? 0}{" "}
            </p>
            <hr className="border-0 border-t border-gray-200 my-3.5" />
            <div className="bg-gray-50 rounded-lg px-3 py-2 mt-2">
              <p>评论</p>

              {item.comments.map((c) => {
                return (
                  <div
                    key={c.id}
                    className="py-1 border-b border-dashed border-gray-200 last:border-b-0"
                  >
                    <p
                      className="break-words"
                      dangerouslySetInnerHTML={{ __html: c.content ?? "" }}
                    ></p>
                  </div>
                );
              })}
              <hr className="border-0 border-t border-gray-200 my-3.5" />
            </div>
          </div>
        );
      })}
    </>
  );
};

export default PostDetail;
