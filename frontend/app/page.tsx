import Link from "next/link";
import { API_BASE } from "@/lib/api";

export default async function Home() {
  const count: Count = await (await fetch(`${API_BASE}/count`)).json();
  return (
    <>
      <div className="border rounded-lg p-2 border-gray-300 shadow-md">
        <p>本站备份了毛毡2016-2024的所有可以被找到的的帖子，您可以进行检索</p>
        <p>站长:xiaole6324 xiaole602010@qq.com</p>
        <p>感谢编程追梦者为我提供了这个站的favicon</p>
        <p>
          v1.1.0:基于PostgreSQL，做了一个简单的搜索功能，不一定好用
          <br />
          目前接口速度慢到极致，我实在没什么好办法（（
        </p>
        <p>v1.1.1:更换favicon</p>
        <p>
          v2.0.0使用Rust重构整个后端，理论上API速度更快了 REWRITE IT IN
          RrrrrrrrUST
        </p>
        <p>
          v2.1.0:增加了用户搜索功能，解决了倒排索引失效问题，还增加了统计数据
        </p>
      </div>
      <div className="border rounded-lg p-3 border-gray-300 shadow-md mt-6">
        <p className="font-semibold text-gray-800">数据统计</p>
        <div className="mt-2 grid grid-cols-2 sm:grid-cols-4 gap-3">
          <div className="border border-gray-200 rounded-lg px-4 py-3 text-center shadow-sm">
            <p className="text-2xl font-semibold text-blue-600">
              {count.posts}
            </p>
            <p className="mt-1 text-sm text-gray-500">帖子</p>
          </div>
          <div className="border border-gray-200 rounded-lg px-4 py-3 text-center shadow-sm">
            <p className="text-2xl font-semibold text-blue-600">
              {count.replies}
            </p>
            <p className="mt-1 text-sm text-gray-500">回复</p>
          </div>
          <div className="border border-gray-200 rounded-lg px-4 py-3 text-center shadow-sm">
            <p className="text-2xl font-semibold text-blue-600">
              {count.comments}
            </p>
            <p className="mt-1 text-sm text-gray-500">评论</p>
          </div>
          <div className="border border-gray-200 rounded-lg px-4 py-3 text-center shadow-sm">
            <p className="text-2xl font-semibold text-blue-600">{count.user}</p>
            <p className="mt-1 text-sm text-gray-500">用户</p>
          </div>
        </div>
      </div>
      <form
        action="/search"
        method="get"
        className="border rounded-lg p-3 border-gray-300 shadow-md mt-6 flex items-center gap-2 flex-wrap"
      >
        <input
          type="text"
          name="keyword"
          required
          placeholder="输入关键词…"
          className="flex-1 min-w-40 border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-blue-600 focus:ring-3 focus:ring-blue-600/15"
        />
        <select
          name="scope"
          defaultValue="post_title"
          className="border border-gray-300 rounded-lg px-3 py-2 text-sm bg-white focus:outline-none focus:border-blue-600"
        >
          <option value="post_title">帖子标题</option>
          <option value="post_content">帖子正文</option>
          <option value="comment">评论</option>
          <option value="reply">回复</option>
          <option value="user">用户</option>
        </select>
        <button
          type="submit"
          className="bg-blue-600 text-white px-5 py-2 rounded-lg font-semibold shadow-md shadow-blue-600/30 transition hover:bg-blue-700 hover:-translate-y-px"
        >
          搜索
        </button>
      </form>
      <Link
        href={"/board"}
        className="border inline-block mt-6 px-5 py-2 bg-blue-600 text-white font-semibold rounded-lg shadow-md shadow-blue-600/30 transition hover:bg-blue-700 hover:-translate-y-px"
      >
        板块
      </Link>
      <Link
        href={"/post"}
        className="border inline-block mt-6 px-5 py-2 border-gray-300   font-semibold rounded-lg shadow-md shadow-blue-600/30 transition hover:bg-gray-300 hover:-translate-y-px"
      >
        帖子
      </Link>
      <Link
        href={"/user"}
        className="border inline-block mt-6 px-5 py-2  border-gray-300 font-semibold rounded-lg shadow-md shadow-blue-600/30 transition hover:bg-gray-300 hover:-translate-y-px"
      >
        用户
      </Link>
    </>
  );
}
