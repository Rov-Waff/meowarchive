# Meowarchive猫史档案馆

爬取了编程猫论坛2016-2024的全部帖子，这里开源了前后端的代码，但不提供数据库记录

# 快速开始

后端使用uv+fastapi，可以直接使用`main.py`快速启动开发环境，前端使用nextjs
数据库使用了PostgreSQL，启动前请配置好.env的DB_URL环境变量

```bash
# 后端，请使用init.sql初始化数据库
$ uv sync
$ uv run main.py

#前端
$ npm i
$ npm run dev
```
## 协议

基于Apache 2.0协议开源