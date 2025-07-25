use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use axum_server::tls_rustls::RustlsConfig;
use tera::{Tera, Context};
use lazy_static::lazy_static;
use serde_json::json;
use pulldown_cmark::{Parser, Options, html};
use std::fs;

lazy_static! {
    static ref TERA: Tera = {
        let mut tera = Tera::new("templates/**/*.html").expect("Parsing error(s)");
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

fn base_context() -> Context {
    let mut context = Context::new();

    // 语言相关
    context.insert("lang", "zh-CN");
    context.insert("language_title", "语言");
    context.insert("language_options", &vec![
        json!({"url": "/zh/", "name": "中文", "selected": true}),
        json!({"url": "/en/", "name": "English", "selected": false}),
    ]);

    // 页面标题和描述
    context.insert("title", "我的博客");
    context.insert("description", "欢迎访问我的博客");
    context.insert("canonical_url", "https://lee.haoren.info/");
    context.insert("rss_url", "https://lee.haoren.info/rss.xml");
    context.insert("rss_title", "我的博客 RSS 订阅");

    // Open Graph 和 Twitter 卡片信息
    context.insert("og_title", "我的博客");
    context.insert("og_description", "欢迎访问我的博客");
    context.insert("og_url", "https://lee.haoren.info/");
    context.insert("og_image", "https://lee.haoren.info/images/og-image.png");
    context.insert("og_site_name", "Lee的博客");

    context.insert("twitter_title", "我的博客");
    context.insert("twitter_description", "欢迎访问我的博客");
    context.insert("twitter_image", "https://lee.haoren.info/images/twitter-image.png");

    // 应用相关信息
    context.insert("application_name", "Lee博客");
    context.insert("apple_mobile_web_app_title", "Lee博客");
    context.insert("theme_color", "#ffffff");
    context.insert("msapplication_TileColor", "#ffffff");

    // Header 相关
    context.insert("home_url", "/");
    context.insert("title", "Lee博客");
    context.insert("header_title", "浩·仁");
    context.insert("logo_url", "/static/images/logo.png");

    context.insert("posts_url", "/posts");
    context.insert("posts_text", "文章");

    context.insert("tags_url", "/tags");
    context.insert("tags_text", "标签");

    context.insert("categories_url", "/categories");
    context.insert("categories_text", "分类");

    context.insert("docs_url", "/docs");
    context.insert("docs_text", "文档");

    context.insert("about_url", "/about");
    context.insert("about_text", "关于");

    context.insert("github_url", "https://github.com/Travis-Lee");

    context.insert("search_placeholder", "搜索文章...");
    context.insert("search_title", "搜索");
    context.insert("clear_title", "清除搜索");
    context.insert("theme_switch_title", "切换主题");

    // Footer 相关
    context.insert("back_to_top_title", "返回顶部");
    context.insert("view_comments_title", "查看评论");
    context.insert("copy_title", "复制代码");
    context.insert("max_shown_lines", &10);

    context.insert("algolia_app_id", "你的AlgoliaAppID");
    context.insert("algolia_index", "你的索引名称");
    context.insert("algolia_search_key", "你的搜索Key");
    context.insert("highlight_tag", "em");
    context.insert("max_result_length", &100);
    context.insert("no_results_found", "未找到结果");
    context.insert("snippet_length", &50);
    context.insert("search_type", "algolia");

    context.insert("cancel_text", "取消");

    // 文章列表示例
    context.insert("posts", &vec![
        json!({"url": "/posts/hello-world", "title": "你好，世界"}),
        json!({"url": "/posts/rust-tutorial", "title": "Rust 教程"}),
    ]);

    // 分类列表示例
    context.insert("categories", &vec![
        json!({"url": "/categories/rust", "name": "Rust"}),
        json!({"url": "/categories/web", "name": "Web 开发"}),
    ]);

    // 标签列表示例
    context.insert("tags", &vec![
        json!({"url": "/tags/rust", "name": "Rust"}),
        json!({"url": "/tags/tutorial", "name": "教程"}),
    ]);

    // 关于页面内容示例
    //context.insert("about_content", "这是Lee的个人博客，分享技术文章和编程心得。");

    context.insert("static_url", "/static");
    context
}

async fn index_handler() -> impl IntoResponse {
    let context = base_context();
    match TERA.render("index.html", &context) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("渲染失败详细信息:\n{:#?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "模板渲染错误").into_response()
        }
    }
}



async fn posts_handler() -> Html<String> {
    let context = base_context();
    let rendered = TERA.render("posts.html", &context).unwrap();
    Html(rendered)
}

async fn tags_handler() -> Html<String> {
    let context = base_context();
    let rendered = TERA.render("tags.html", &context).unwrap();
    Html(rendered)
}

async fn categories_handler() -> Html<String> {
    let context = base_context();
    let rendered = TERA.render("categories.html", &context).unwrap();
    Html(rendered)
}

/*
async fn about_handler() -> Html<String> {
    let context = base_context();
    let rendered = TERA.render("about.html", &context).unwrap();
    Html(rendered)
}
*/


async fn about_handler() -> Html<String> {
    let mut context = base_context();

    let about_md = fs::read_to_string("content/about.md").unwrap_or_else(|_| {
        "⚠️ 无法加载 about 内容，请检查 content/about.md 是否存在。".to_string()
    });

    // Markdown 转 HTML
    let parser = Parser::new_ext(&about_md, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    context.insert("about_content", &html_output);

    let rendered = TERA.render("about.html", &context).unwrap();
    Html(rendered)
}

async fn not_found_handler() -> impl IntoResponse {
    let context = base_context();
    let rendered = TERA.render("404.html", &context).unwrap();
    (StatusCode::NOT_FOUND, Html(rendered))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let static_service = ServeDir::new("static");

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/posts", get(posts_handler))
        .route("/tags", get(tags_handler))
        .route("/categories", get(categories_handler))
        .route("/about", get(about_handler))
        .nest_service("/static", static_service)
        .fallback(not_found_handler);

    let addr = SocketAddr::from(([0, 0, 0, 0], 443));
    println!("Listening on https://{}", addr);

    let config = RustlsConfig::from_pem_file(
        "/etc/letsencrypt/live/lee.haoren.info/fullchain.pem",
        "/etc/letsencrypt/live/lee.haoren.info/privkey.pem",
    )
    .await?;

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

