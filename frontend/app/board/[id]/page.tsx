"use client";

import Link from "next/link";
import { useEffect, useState } from "react";

const BoardPosts = async () => {
  let [currentPage, setPage] = useState(1);
  let pageData: PageResult<Post> = {
    total_page: 0,
    current_page: 0,
    has_next: false,
    has_prev: false,
    item: [],
  };
  useEffect(() => {
    fetch(
      `http://localhost:8000/api/board/5/page?page_size=30&page_num=${currentPage}`,
    ).then((r) => {
       = r.json();
    });
  }, [currentPage]);
  return (
    <>
      {pageData.item.map((item) => {
        return (
          <div key={item.id}>
            <Link href={`/post/${item.id}`}>{item.title}</Link>
          </div>
        );
      })}
    </>
  );
};

export default BoardPosts;
