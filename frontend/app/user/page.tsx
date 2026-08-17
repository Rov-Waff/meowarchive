import Link from "next/link";
import UserListPageComponent from "./UserListPageComponent";
import { API_BASE } from "@/lib/api";

const UserList = async ({
  searchParams,
}: {
  searchParams: Promise<{ page?: string }>;
}) => {
  const { page = "1" } = await searchParams;
  let userData: PageResult<User> = await (
    await fetch(`${API_BASE}/user/?page_size=30&page_num=${page}`)
  ).json();
  return (
    <>
      <UserListPageComponent
        current={userData.current_page}
        totalPage={userData.total_page}
      />
      {userData.item.map((item) => {
        return (
          <div
            key={item.id}
            className="bg-white border border-gray-200 rounded-[10px] px-4 py-3 my-3 shadow-sm flex items-center gap-4 transition hover:shadow-md hover:-translate-y-px"
          >
            <img
              src={item.avatar}
              alt={item.nickname}
              className="w-14 h-14 rounded-full object-cover bg-gray-100"
            />
            <div className="min-w-0 flex-1">
              <Link
                href={`/user/${item.id}`}
                className="font-semibold text-gray-800 hover:underline"
              >
                {item.nickname}
              </Link>
              <p className="text-sm text-gray-500 truncate">
                {item.description}
              </p>
              <p className="text-xs text-gray-400">
                Lv.{item.level} 获赞:{item.praise_times} 收藏:
                {item.collection_times}
              </p>
            </div>
          </div>
        );
      })}
    </>
  );
};

export default UserList;
