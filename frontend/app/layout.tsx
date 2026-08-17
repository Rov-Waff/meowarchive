import type { Metadata } from "next";
import "./globals.css";
export const metadata: Metadata = {
  title: "猫史档案馆",
  description: "编程猫社区论坛归档站，收录板块、帖子、回复与用户数据。",
};

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html lang="zh-CN">
      <body className="min-h-full flex flex-col max-w-[860px] mx-auto px-4 pt-5 pb-12 bg-[#f6f7f9] text-[#24292f] leading-relaxed font-sans">
        <h1 className="text-3xl font-extrabold tracking-wide mb-1 bg-linear-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
          猫史档案馆
        </h1>
        <hr className="border-0 border-t border-gray-200 my-3.5" />
        {children}
      </body>
    </html>
  );
}
