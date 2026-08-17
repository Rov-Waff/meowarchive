"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default ({
  current,
  id,
  totalPage,
}: {
  current: number;
  id: number;
  totalPage: number;
}) => {
  const [targetPage, setTarget] = useState(String(current));
  const router = useRouter();

  const jumpPage = () => {
    const page = Number.parseInt(targetPage, 10);

    if (Number.isNaN(page) || page < 1 || page > totalPage) {
      return;
    }

    router.push(`/post/${id}?page=${page}`);
  };

  return (
    <div className="pagination">
      {current > 1 ? (
        <Link
          className="pagination-btn"
          href={`/post/${id}?page=${current - 1}`}
        >
          上一页
        </Link>
      ) : (
        <span className="pagination-btn disabled">上一页</span>
      )}
      <span className="pagination-info">
        第 {current} 页 / 共 {totalPage} 页
      </span>
      {current < totalPage ? (
        <Link
          className="pagination-btn"
          href={`/post/${id}?page=${current + 1}`}
        >
          下一页
        </Link>
      ) : (
        <span className="pagination-btn disabled">下一页</span>
      )}
      <span className="pagination-jump">
        <input value={targetPage} onChange={(p) => setTarget(p.target.value)} />
        <button onClick={jumpPage}>跳转</button>
      </span>
    </div>
  );
};
