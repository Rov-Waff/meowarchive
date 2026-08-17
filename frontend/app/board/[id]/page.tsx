import Link from "next/link";
import JumpComponent from "../components/JumpComponent";
import { API_BASE } from "@/lib/api";

interface Props {
  searchParams: Promise<{ page?: string }>;
  params: Promise<{ id: number }>;
}

const BoardPosts = async ({ searchParams, params }: Props) => {
  const { page = "1" } = await searchParams;
  const { id } = await params;
  let pageData: PageResult<Post> = await (
    await fetch(`${API_BASE}/board/${id}/page?page_size=30&page_num=${page}`)
  ).json();
  return (
    <>
      <JumpComponent
        boardId={id}
        current={pageData.current_page}
        totalPage={pageData.total_page}
      />
      <hr className="border-0 border-t border-gray-200 my-3.5" />
      {pageData.item.map((item) => {
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
            <hr className="border-0 border-t border-gray-200 my-3.5" />
          </div>
        );
      })}
    </>
  );
};

export default BoardPosts;
