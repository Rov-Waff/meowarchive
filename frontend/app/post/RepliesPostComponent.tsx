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
    <>
      {current > 1 ? (
        <Link href={`/post/${id}?page=${current - 1}`}>上一页</Link>
      ) : (
        <></>
      )}
      {current < totalPage ? (
        <Link href={`/post/${id}?page=${current + 1}`}>下一页</Link>
      ) : (
        <></>
      )}
      <input
        value={targetPage}
        onChange={(p) => setTarget(p.target.value)}
      />
      <button onClick={jumpPage}>跳转页面</button>
    </>
  );
};
