"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default ({
  current,
  totalPage,
}: {
  current: number;
  totalPage: number;
}) => {
  const [targetPage, setTarget] = useState(String(current));
  const router = useRouter();

  const jumpPage = () => {
    const page = Number.parseInt(targetPage, 10);

    if (Number.isNaN(page) || page < 1 || page > totalPage) {
      return;
    }

    router.push(`/user?page=${page}`);
  };

  return (
    <div className="flex items-center flex-wrap gap-2.5 bg-white border border-gray-200 rounded-[10px] px-3.5 py-2 my-3 shadow-sm">
      {current > 1 ? (
        <Link
          className="px-3 py-1 border border-gray-300 rounded-md bg-white text-gray-700 text-sm transition-all hover:border-blue-600 hover:bg-blue-50 hover:text-blue-600"
          href={`/user?page=${current - 1}`}
        >
          上一页
        </Link>
      ) : (
        <span className="px-3 py-1 border border-gray-300 rounded-md bg-gray-50 text-gray-400 text-sm cursor-not-allowed">
          上一页
        </span>
      )}
      <span className="text-sm text-gray-500 whitespace-nowrap">
        第 {current} 页 / 共 {totalPage} 页
      </span>
      {current < totalPage ? (
        <Link
          className="px-3 py-1 border border-gray-300 rounded-md bg-white text-gray-700 text-sm transition-all hover:border-blue-600 hover:bg-blue-50 hover:text-blue-600"
          href={`/user?page=${current + 1}`}
        >
          下一页
        </Link>
      ) : (
        <span className="px-3 py-1 border border-gray-300 rounded-md bg-gray-50 text-gray-400 text-sm cursor-not-allowed">
          下一页
        </span>
      )}
      <span className="inline-flex items-center gap-1.5 ml-auto">
        <input
          value={targetPage}
          onChange={(p) => setTarget(p.target.value)}
          className="w-16 text-center border border-gray-300 rounded-md px-2.5 py-1 text-sm bg-white focus:outline-none focus:border-blue-600 focus:ring-3 focus:ring-blue-600/15"
        />
        <button
          onClick={jumpPage}
          className="bg-blue-600 text-white rounded-md px-3.5 py-1 text-sm cursor-pointer transition hover:bg-blue-700"
        >
          跳转
        </button>
      </span>
    </div>
  );
};
