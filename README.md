rust-blog/
├── src/
│   ├── main.rs
│   ├── generator.rs  # 静态页面生成器
│   ├── markdown.rs   # markdown 转 HTML
│   └── template.rs   # 模板渲染
├── posts/            # 博文 markdown 文件
│   ├── hello.md
│   └── ...
├── static/           # 静态资源 (CSS、JS、图片等)
├── templates/        # HTML 模板
│   ├── base.html
│   └── post.html
├── public/           # 生成后的静态页面
│   ├── index.html
│   └── posts/...
├── Cargo.toml

