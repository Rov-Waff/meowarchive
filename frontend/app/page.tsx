import Link from "next/link";

export default async function Home() {
  let boards: Array<Board> = await (await fetch("http://localhost:8000/api/board/all")).json()
  console.log(boards)
  return (

    <>
      <Link href={"/board"}>板块</Link>
    </>
  );
}
