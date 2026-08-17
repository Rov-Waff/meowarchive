import Link from "next/link";

export default async function Home() {
  return (
    <>
      <div className="border rounded-lg p-2 border-gray-300 shadow-md">
        <p>本站备份了毛毡2016-2024的所有可以被找到的的帖子，您可以进行检索</p>
        <p>站长:xiaole6324 xiaole602010@qq.com</p>
        <p>感谢编程追梦者为我提供了一点精神支持（雾）</p>
      </div>
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
