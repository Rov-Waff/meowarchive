import Link from "next/link";
import RepliesPostComponent from "../RepliesPostComponent";

const PostDetail = async ({
  params,
  searchParams,
}: {
  params: Promise<{ id: number }>;
  searchParams: Promise<{ page: number }>;
}) => {
  const { id } = await params;
  const { page } = await searchParams;
  const postData: Post = await (
    await fetch("http://localhost:8000/api/post/" + id)
  ).json();
  const replyData: PageResult<ReplyDTO> = await (
    await fetch(
      `http://localhost:8000/api/post/${id}/replies?page_size=30&page_num=1`,
    )
  ).json();
  return (
    <>
      <h2>{postData.title}</h2>
      <p>
        用户:
        <Link href={`/user/${postData.user.id}`}>
          {postData.user.nickname}
        </Link>{" "}
        查看:{postData.n_views} 回复:{postData.n_replies} 评论:
        {postData.n_comments} 创建时间:{postData.created_at.toString()}{" "}
      </p>
      <hr />
      <div dangerouslySetInnerHTML={{ __html: postData.content }} />
      <hr />
      <h3>评论</h3>
      <RepliesPostComponent
        current={replyData.current_page}
        totalPage={replyData.total_page}
        id={id}
      />
      {replyData.item.map((item) => {
        return (
          <div className="border p-1 m-1" key={item.reply.id}>
            <Link href={`/user/${item.user.id}`}>{item.user.nickname}</Link>
            <p dangerouslySetInnerHTML={{ __html: item.reply.content }}></p>
            <p>
              {item.reply.is_liked ? <>置顶</> : <></>} 点赞
              {item.reply.n_likes}{" "}
            </p>
            <hr />
            <div>
              <p>评论</p>

              {item.comments.map((c) => {
                return (
                  <div key={c.id}>
                    <Link href={`/user/${c.user_id}`}>{c.user.nickname}</Link>
                    <p dangerouslySetInnerHTML={{ __html: c.content }}></p>
                  </div>
                );
              })}
              <hr />
            </div>
          </div>
        );
      })}
    </>
  );
};

export default PostDetail;
