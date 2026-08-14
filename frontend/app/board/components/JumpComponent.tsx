'use client'

import { useRouter } from "next/navigation"
import { useState } from "react"

export default ({ boardId }: { boardId: number }) => {
    const [target, setTarget] = useState('')
    const router = useRouter()

    const handleJump = () => {
        router.push(`/board/${boardId}?page=${target}`)
    }

    return (<>
        <input value={target} onChange={(e) => setTarget(e.target.value)} type="text" />
        <button onClick={handleJump}>Jump!</button>
    </>
    )
}