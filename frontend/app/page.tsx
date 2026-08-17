import Link from "next/link";

export default async function Home() {
  return (
    <>
      <Link
        href={"/board"}
        className="inline-block mt-6 px-5 py-2 bg-blue-600 text-white font-semibold rounded-lg shadow-md shadow-blue-600/30 transition hover:bg-blue-700 hover:-translate-y-px"
      >
        板块
      </Link>
      <Link
        href={"/post"}
        className="inline-block mt-6 ml-4 text-blue-600 hover:underline"
      >
        帖子
      </Link>
    </>
  );
}
