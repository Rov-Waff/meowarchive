import Link from "next/link";
import { API_BASE } from "@/lib/api";

export const dynamic = "force-dynamic";

const SCOPE_ENDPOINTS: Record<string, { path: string; label: string }> = {
  post_title: { path: "/post/search/title", label: "帖子标题" },
  post_content: { path: "/post/search/content", label: "帖子正文" },
  comment: { path: "/comment/search/content", label: "评论" },
  reply: { path: "/reply/search/content", label: "回复" },
};

const PAGE_SIZE = 30;

type SearchResult =
  | { scope: "post"; data: PageResult<PostSearchItem> }
  | { scope: "comment"; data: PageResult<CommentSearchItem> }
  | { scope: "reply"; data: PageResult<ReplySearchItem> };

function SearchPagination({
  keyword,
  scope,
  current,
  totalPage,
  hasPrev,
  hasNext,
}: {
  keyword: string;
  scope: string;
  current: number;
  totalPage: number;
  hasPrev: boolean;
  hasNext: boolean;
}) {
  const href = (p: number) =>
    `/search?keyword=${encodeURIComponent(keyword)}&scope=${scope}&page=${p}`;
  const buttonClass =
    "px-3 py-1 border border-gray-300 rounded-md bg-white text-gray-700 text-sm transition-all hover:border-blue-600 hover:bg-blue-50 hover:text-blue-600";
  const disabledClass =
    "px-3 py-1 border border-gray-300 rounded-md bg-gray-50 text-gray-400 text-sm cursor-not-allowed";

  return (
    <div className="flex items-center flex-wrap gap-2.5 bg-white border border-gray-200 rounded-[10px] px-3.5 py-2 my-3 shadow-sm">
      {hasPrev ? (
        <Link className={buttonClass} href={href(current - 1)}>
          上一页
        </Link>
      ) : (
        <span className={disabledClass}>上一页</span>
      )}
      <span className="text-sm text-gray-500 whitespace-nowrap">
        第 {current} 页 / 共 {totalPage} 页
      </span>
      {hasNext ? (
        <Link className={buttonClass} href={href(current + 1)}>
          下一页
        </Link>
      ) : (
        <span className={disabledClass}>下一页</span>
      )}
    </div>
  );
}

export default async function SearchPage({
  searchParams,
}: {
  searchParams: Promise<{ keyword?: string; scope?: string; page?: string }>;
}) {
  const { keyword = "", scope = "post_title", page = "1" } =
    await searchParams;
  const pageNum = Number.parseInt(page, 10) || 1;
  const activeScope = SCOPE_ENDPOINTS[scope] ? scope : "post_title";
  const endpoint = SCOPE_ENDPOINTS[activeScope];

  if (!keyword.trim()) {
    return (
      <div className="border rounded-lg p-4 border-gray-300 shadow-md">
        <p>请输入关键词后再搜索。</p>
        <Link href="/" className="text-blue-600 hover:underline">
          返回首页
        </Link>
      </div>
    );
  }

  const url = `${API_BASE}${endpoint.path}?keyword=${encodeURIComponent(
    keyword,
  )}&page_num=${pageNum}&page_size=${PAGE_SIZE}`;

  let result: SearchResult | null = null;
  try {
    const res = await fetch(url, { cache: "no-store" });
    if (!res.ok) {
      throw new Error(`HTTP ${res.status}`);
    }
    if (activeScope === "comment") {
      result = { scope: "comment", data: await res.json() };
    } else if (activeScope === "reply") {
      result = { scope: "reply", data: await res.json() };
    } else {
      result = { scope: "post", data: await res.json() };
    }
  } catch {
    result = null;
  }

  if (!result) {
    return (
      <div className="border rounded-lg p-4 border-gray-300 shadow-md mt-6">
        <p>搜索失败，请确认后端服务可用后重试。</p>
        <Link href="/" className="text-blue-600 hover:underline">
          返回首页
        </Link>
      </div>
    );
  }

  if (result.scope === "comment") {
    return (
      <>
        <h2 className="text-[1.4rem] my-2">
          搜索范围:{endpoint.label} 关键词:{keyword}
        </h2>
        <SearchPagination
          keyword={keyword}
          scope={activeScope}
          current={result.data.current_page}
          totalPage={result.data.total_page}
          hasPrev={result.data.has_prev}
          hasNext={result.data.has_next}
        />
        {result.data.item.map((i) => (
          <div
            key={i.id}
            className="bg-white border border-gray-200 rounded-[10px] px-4 py-3 my-3 shadow-sm"
          >
            <span className="flex items-center gap-1.5">
              <img
                src={i.user.avatar}
                alt={i.user.nickname}
                className="w-6 h-6 rounded-full object-cover bg-gray-100"
              />
              <Link
                href={`/user/${i.user.id}`}
                className="text-blue-600 hover:underline"
              >
                {i.user.nickname}
              </Link>
            </span>
            <p
              className="break-words"
              dangerouslySetInnerHTML={{ __html: i.content }}
            />
            <p className="text-sm text-gray-500">
              点赞:{i.n_likes} 时间:{i.created_at.toString()}
            </p>
          </div>
        ))}
      </>
    );
  }

  if (result.scope === "reply") {
    return (
      <>
        <h2 className="text-[1.4rem] my-2">
          搜索范围:{endpoint.label} 关键词:{keyword}
        </h2>
        <SearchPagination
          keyword={keyword}
          scope={activeScope}
          current={result.data.current_page}
          totalPage={result.data.total_page}
          hasPrev={result.data.has_prev}
          hasNext={result.data.has_next}
        />
        {result.data.item.map((i) => (
          <div
            key={i.id}
            className="bg-white border border-gray-200 rounded-[10px] px-4 py-3 my-3 shadow-sm"
          >
            <span className="flex items-center gap-1.5">
              <img
                src={i.user.avatar}
                alt={i.user.nickname}
                className="w-6 h-6 rounded-full object-cover bg-gray-100"
              />
              <Link
                href={`/user/${i.user.id}`}
                className="text-blue-600 hover:underline"
              >
                {i.user.nickname}
              </Link>
            </span>
            <p
              className="break-words"
              dangerouslySetInnerHTML={{ __html: i.content }}
            />
            <p className="text-sm text-gray-500">
              点赞:{i.n_likes} 评论:{i.n_comments} 时间:
              {i.created_at.toString()}
            </p>
          </div>
        ))}
      </>
    );
  }

  return (
    <>
      <h2 className="text-[1.4rem] my-2">
        搜索范围:{endpoint.label} 关键词:{keyword}
      </h2>
      <SearchPagination
        keyword={keyword}
        scope={activeScope}
        current={result.data.current_page}
        totalPage={result.data.total_page}
        hasPrev={result.data.has_prev}
        hasNext={result.data.has_next}
      />
      {result.data.item.map((i) => (
        <div
          key={i.id}
          className="bg-white border border-gray-200 rounded-[10px] px-4 py-3 my-3 shadow-sm transition hover:shadow-md hover:-translate-y-px"
        >
          <Link
            href={`/post/${i.id}`}
            className="font-semibold text-gray-800 hover:underline"
          >
            {i.title}
          </Link>
          <p className="mt-1 text-sm text-gray-500">
            阅读:{i.n_view} 回复:{i.n_replies} 评论:{i.n_comments}
          </p>
          <p className="mt-1 text-sm text-gray-500 flex items-center gap-1.5">
            作者:
            <img
              src={i.user.avatar}
              alt={i.user.nickname}
              className="w-5 h-5 rounded-full object-cover bg-gray-100"
            />
            <Link
              href={`/user/${i.user.id}`}
              className="text-blue-600 hover:underline"
            >
              {i.user.nickname}
            </Link>
          </p>
        </div>
      ))}
    </>
  );
}
