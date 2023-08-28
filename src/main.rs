pub mod async_component;
pub mod meta;
pub mod render;
pub mod ssg;

use std::path::Path;

use leptos::*;
use ssg::Ssg;
use tokio::fs;

use crate::{
    async_component::Async,
    meta::{Head, Html, Title},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("./out/articles").await?;

    // initialize the Ssg context
    let ssg = Ssg::new(Path::new("./out"));

    // generate the pages
    ssg.gen("index.html", Homepage).await?;

    let articles = list_articles().await;
    for article in articles {
        ssg.gen(&format!("articles/{}.html", article.slug), || {
            ArticlePage(ArticlePageProps { article })
        })
        .await?;
    }

    Ok(())
}

#[component]
// This is a common Layout component that will be used by all pages.
fn Layout(children: Children) -> impl IntoView {
    view! {
        // Html and Title are components from the meta module. They will be
        // rendered to the <head> of the page.
        <Html attrs=vec![("lang", "en")] />
        <Title>default title</Title>
        <Head>
            <style>{r#"
                body {
                    max-width: 600px;
                    margin: 0 auto;
                }
            "#}</style>
        </Head>

        // Async is a component from the async_component module.
        // It will wrap an async function that returns an IntoView.
        <section>
            <Async view=navigation_bar />
        </section>

        <main>
            {children()}
        </main>
    }
}

#[component]
fn Homepage() -> impl IntoView {
    view! {
        <Layout>
            <h1>Homepage</h1>
        </Layout>
    }
}

#[component]
fn ArticlePage(article: Article) -> impl IntoView {
    let title = article.title.clone();
    view! {
        <Title>{title}</Title>
        <Layout>
            <h1>{article.title}</h1>
            <p>{article.content}</p>
        </Layout>
    }
}

async fn navigation_bar() -> impl IntoView {
    // we are rendering this component using the Ssg module, so we can expect
    // the SsgContext to be available. We can use that to understand what page
    // we are currently rendering and make the link for that page bold.
    let ssg_ctx = expect_context::<ssg::SsgContext>();
    let is_index = ssg_ctx.path == "index.html";

    // this is an "async component" so we can use async/await on the top level.
    // It will be wrapped with <Async />.
    let articles = list_articles().await;

    view! {
        <nav>
            <ul>
                <li style:font-weight=move || if is_index { Some("bold") } else { None }>
                    <a href="/">Homepage</a>
                </li>

                {articles.into_iter().map(|article| {
                    let href = format!("/articles/{}.html", article.slug);
                    let active = href.ends_with(&ssg_ctx.path);
                    view! {
                        <li style:font-weight=move || if active { Some("bold") } else { None }>
                            <a href=href>{article.title}</a>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </nav>
    }
}

struct Article {
    title: String,
    slug: String,
    content: String,
}

// this is a fake database call
async fn list_articles() -> Vec<Article> {
    vec![
        Article {
            title: "title 1".into(),
            slug: "title-1".into(),
            content: "content 1".into(),
        },
        Article {
            title: "title 2".into(),
            slug: "title-2".into(),
            content: "content 2".into(),
        },
        Article {
            title: "title 3".into(),
            slug: "title-3".into(),
            content: "content 3".into(),
        },
    ]
}
