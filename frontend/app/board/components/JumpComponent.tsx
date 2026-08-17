"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default ({
  boardId,
  current,
  totalPage,
}: {
  boardId: number;
  current: number;
  totalPage: number;
}) => {
  const [target, setTarget] = useState(String(current));
  const router = useRouter();

  const handleJump = () => {
    const page = Number.parseInt(target, 10);

    if (Number.isNaN(page) || page < 1 || page > totalPage) {
      return;
    }

    router.push(`/board/${boardId}?page=${page}`);
  };

  return (
    <div className="pagination">
      {current > 1 ? (
        <Link className="pagination-btn" href={`?page=${current - 1}`}>
          上一页
        </Link>
      ) : (
        <span className="pagination-btn disabled">上一页</span>
      )}
      <span className="pagination-info">
        第 {current} 页 / 共 {totalPage} 页
      </span>
      {current < totalPage ? (
        <Link className="pagination-btn" href={`?page=${current + 1}`}>
          下一页
        </Link>
      ) : (
        <span className="pagination-btn disabled">下一页</span>
      )}
      <span className="pagination-jump">
        <input
          value={target}
          onChange={(e) => setTarget(e.target.value)}
          type="text"
        />
        <button onClick={handleJump}>跳转</button>
      </span>
    </div>
  );
};
