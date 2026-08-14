import Link from "next/link"

const Board = async () => {
    let boards: Array<Board> = await (await fetch("http://localhost:8000/api/board/all")).json()
    return (<>
        <p>板块列表... !?板块块版?!</p>
        <hr />
        {
            boards.map((item) => {
                return <div key={item.id}>
                    <Link href={`/board/${item.id}`}>{item.name}</Link>
                    <br />
                    <p>
                        {item.is_hot ? <>热门</> : <></>}
                        讨论:{item.n_discussions}
                        帖子:{item.n_posts}
                    </p>
                </div>
            })
        }
    </>)
}

export default Board