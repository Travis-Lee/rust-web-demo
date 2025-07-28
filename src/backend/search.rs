use axum::{extract::{Query, State}, response::Json};
use serde::{Deserialize, Serialize};
use tantivy::{doc, Index, ReloadPolicy};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

pub async fn search_handler(
    Query(params): Query<SearchParams>,
    State(index): State<Arc<Index>>,
) -> Json<Vec<SearchResult>> {
    let searcher = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .expect("创建 reader 失败")
        .searcher();

    let schema = index.schema();
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let url_field = schema.get_field("url").unwrap();

    let query_parser = QueryParser::for_index(&index, vec![title, body]);
    let query = match query_parser.parse_query(&params.q) {
        Ok(q) => q,
        Err(_) => return Json(vec![]),
    };

    let top_docs = match searcher.search(&query, &TopDocs::with_limit(10)) {
        Ok(docs) => docs,
        Err(_) => vec![],
    };

    let mut results = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        let title_text = retrieved_doc.get_first(title).and_then(|v| v.as_text()).unwrap_or("");
        let body_text = retrieved_doc.get_first(body).and_then(|v| v.as_text()).unwrap_or("");
        let url_text = retrieved_doc.get_first(url_field).and_then(|v| v.as_text()).unwrap_or("#");

        let snippet = if body_text.len() > 100 {
            &body_text[..100]
        } else {
            body_text
        };

        results.push(SearchResult {
            title: title_text.to_string(),
            url: url_text.to_string(),
            snippet: snippet.to_string(),
        });
    }

    Json(results)
}

/*
pub fn create_index() -> tantivy::Result<Index> {
    use tantivy::schema::*;
    use std::path::Path;
    use std::fs;

    let mut schema_builder = Schema::builder();
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let body = schema_builder.add_text_field("body", TEXT | STORED);
    let url = schema_builder.add_text_field("url", STORED);
    let schema = schema_builder.build();

    let index_path = Path::new("./tantivy_index");
    fs::create_dir_all(index_path)?; // 新增：确保目录存在

    let index = Index::create_in_dir(index_path, schema.clone())?;
    let mut writer = index.writer(50_000_000)?;

    let _ = writer.add_document(doc!(
        title => "Rust Tantivy 介绍",
        body => "Tantivy 是一个用 Rust 写的全文搜索库，非常适合自建搜索。",
        url => "/posts/tantivy-intro.html"
    ));

    let _ = writer.add_document(doc!(
        title => "Axum Web 框架",
        body => "Axum 是 Rust 生态中轻量级的 Web 框架。",
        url => "/posts/axum-guide.html"
    ));

    writer.commit()?;

    Ok(index)
}
*/

use std::fs;
use std::path::Path;
use tantivy::schema::*;

pub fn create_or_open_index() -> tantivy::Result<Index> {
    let index_path = Path::new("./tantivy_index");

    // 定义 schema（无论是新建或打开都需要）
    let mut schema_builder = Schema::builder();
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let body = schema_builder.add_text_field("body", TEXT | STORED);
    let url = schema_builder.add_text_field("url", STORED);
    let schema = schema_builder.build();

    if index_path.exists() {
        // 目录存在，尝试打开已有索引
        let index = Index::open_in_dir(index_path)?;
        Ok(index)
    } else {
        // 目录不存在，创建并写入示例文档
        fs::create_dir_all(index_path)?;
        let index = Index::create_in_dir(index_path, schema.clone())?;
        let mut writer = index.writer(50_000_000)?;

        writer.add_document(doc!(
            title => "Rust Tantivy 介绍",
            body => "Tantivy 是一个用 Rust 写的全文搜索库，非常适合自建搜索。",
            url => "/posts/tantivy-intro.html"
        ));

        writer.add_document(doc!(
            title => "Axum Web 框架",
            body => "Axum 是 Rust 生态中轻量级的 Web 框架。",
            url => "/posts/axum-guide.html"
        ));

        writer.commit()?;

        Ok(index)
    }
}

