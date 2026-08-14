
import Link from "next/link";
import JumpComponent from "../components/JumpComponent";

interface Props {
    searchParams: Promise<{ page?: string }>
    params: Promise<{ id: number }>
}

const BoardPosts = async ({ searchParams, params }: Props) => {
    const { page = '1' } = await searchParams;
    const { id } = await params;
    let pageData: PageResult<Post> = await (await fetch(`http://localhost:8000/api/board/${id}/page?page_size=30&page_num=${page}`)).json()
    return (
        <>
            第 {pageData.current_page} 页，共 {pageData.total_page} 页
            <br />
            {pageData.current_page > 1 ? <Link href={`?page=${pageData.current_page - 1}`}>上一页</Link> : <></>}&nbsp;
            {pageData.current_page < pageData.total_page ? <Link href={`?page=${pageData.current_page + 1}`}>下一页</Link> : <></>}
            <JumpComponent boardId={id} />
            <hr />
            {pageData.item.map((item) => {
                return (
                    <div key={item.id}>
                        <Link href={`/post/${item.id}`}>{item.title}</Link>
                        <p>
                            ID:{item.id} 阅读:{item.n_views} 评论:{item.n_comments} 回复:{item.n_replies}
                        </p>
                        <p>
                            {item.is_authorized ? <>官方</> : <></>}  {item.is_pinned ? <>置顶</> : <></>}
                            {item.ask_help_flag ? <>求助</> : <></>} {item.tutorial_flag ? <>教程</> : <></>}
                        </p>
                        <hr />
                    </div>
                );
            })}

        </>
    );
};

export default BoardPosts;
