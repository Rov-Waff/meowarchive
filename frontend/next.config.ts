import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // standalone 只用于自托管（Docker），Vercel 云构建需要关闭
  // 构建时设置 NEXT_OUTPUT_STANDALONE=true 即可启用（Dockerfile 已配）
  output:
    process.env.NEXT_OUTPUT_STANDALONE === "true" ? "standalone" : undefined,
};

export default nextConfig;
