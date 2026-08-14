import Link from "next/link"

const PostDetail = async ({ params }: { params: Promise<{ id: number }> }) => {
    const { id } = await params
    const postData: Post = await (await fetch("http://localhost:8000/api/post/"+id)).json()
    return (<>
        <h2>{postData.title}</h2>
        <p>用户:<Link href={`/user/${postData.user.id}`}>{postData.user.nickname}</Link> 查看:{postData.n_views} 回复:{postData.n_replies} 评论:{postData.n_comments} 创建时间:{postData.created_at.toString()} </p>
        <hr />
        <div dangerouslySetInnerHTML={{ __html: postData.content }} />

    </>)
}

export default PostDetail