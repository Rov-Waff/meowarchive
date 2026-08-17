import Link from "next/link";
import { API_BASE } from "@/lib/api";

const Board = async () => {
  let boards: Array<Board> = await (
    await fetch(`${API_BASE}/board/all`)
  ).json();
  return (
    <>
      <p>板块列表... !?板块块版?!</p>
      <hr className="border-0 border-t border-gray-200 my-3.5" />
      {boards.map((item) => {
        return (
          <div
            key={item.id}
            className="bg-white border border-gray-200 rounded-[10px] px-4 py-3 my-3 shadow-sm transition hover:shadow-md hover:-translate-y-px"
          >
            <Link
              href={`/board/${item.id}`}
              className="text-lg font-semibold text-gray-800 hover:underline"
            >
              {item.name}
            </Link>
            <br />
            <p className="mt-1 text-sm text-gray-500">
              {item.is_hot ? <>热门</> : <></>}
              讨论:{item.n_discussions}
              帖子:{item.n_posts}
            </p>
          </div>
        );
      })}
    </>
  );
};

export default Board;
