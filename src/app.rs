use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{Collection, Bookmark, Folder};

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: leptos::LeptosOptions,
}

#[server]
pub async fn get_collections() -> Result<Vec<Collection>, ServerFnError> {
    use sqlx::PgPool;
    use crate::models::load_env_vars;

    let (database_url, _, _) = load_env_vars().map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let pool: PgPool = sqlx::PgPool::connect(&database_url)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let collections = sqlx::query_as!(
        Collection,
        r#"SELECT 
            id, name, slug, description, is_public, 
            created_at, updated_at 
           FROM collections 
           ORDER BY created_at DESC"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(collections)
}

#[server]
pub async fn create_collection(input: (String, Option<String>)) -> Result<Collection, ServerFnError> {
    use sqlx::PgPool;
    use crate::models::load_env_vars;
    use uuid::Uuid;
    use chrono::Utc;

    let (name, description) = input;
    let (database_url, _, _) = load_env_vars().map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let pool: PgPool = sqlx::PgPool::connect(&database_url)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let id = Uuid::new_v4();
    let slug = name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    let now = Utc::now();

    let collection = sqlx::query_as!(
        Collection,
        r#"INSERT INTO collections (id, name, slug, description, is_public, created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING id, name, slug, description, is_public, created_at, updated_at"#,
        id,
        name,
        slug,
        description,
        true, // 默认为公开
        now,
        now
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(collection)
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/pintree-rust.css"/>
        <Title text="Pintree - Bookmarks Manager"/>

        // HTML内容
        <main>
            <Router>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="collections/:id" view=CollectionView/>
                </Routes>
            </Router>
        </main>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let collections = create_resource(|| (), move |_| get_collections());

    let create_collection = create_action(|input: &(String, Option<String>)| {
        let (name, description) = input;
        create_collection(((*name).clone(), *description))
    });

    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-4xl font-bold text-center mb-8">"Pintree - Bookmarks Manager"</h1>
            
            <div class="max-w-md mx-auto mb-8">
                <ActionForm action=create_collection>
                    <div class="mb-3">
                        <input
                            type="text"
                            name="name"
                            placeholder="Collection Name"
                            class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            required
                        />
                    </div>
                    <div class="mb-3">
                        <textarea
                            name="description"
                            placeholder="Description (Optional)"
                            class="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                        ></textarea>
                    </div>
                    <button
                        type="submit"
                        class="w-full bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded-lg transition duration-200"
                    >
                        "Create Collection"
                    </button>
                </ActionForm>
            </div>

            <Suspense fallback=|| view! { <p>"Loading..."</p> }>
                {move || {
                    collections()
                        .map(move |data| match data {
                            Err(e) => view! { <pre>"Server Error: " {e.to_string()}</pre> }.into_view(),
                            Ok(collections) => collections
                                .into_iter()
                                .map(|collection| {
                                    view! {
                                        <div class="border rounded-lg p-4 mb-4 hover:shadow-md transition-shadow">
                                            <h2 class="text-xl font-semibold">{collection.name}</h2>
                                            <p class="text-gray-600 mt-1">{collection.description.unwrap_or_default()}</p>
                                            <a 
                                                href=format!("/collections/{}", collection.id)
                                                class="mt-2 inline-block text-blue-500 hover:underline"
                                            >
                                                "View Collection"
                                            </a>
                                        </div>
                                    }
                                })
                                .collect_view()
                        })
                }}
            </Suspense>
        </div>
    }
}

#[derive(Params, PartialEq, Eq)]
struct CollectionParams {
    id: String,
}

#[component]
fn CollectionView() -> impl IntoView {
    let params = use_params::<CollectionParams>();
    let collection_id = move || -> Uuid {
        params.with(|p| {
            p.as_ref().ok()
                .and_then(|p| Uuid::parse_str(&p.id).ok())
                .expect("Valid UUID required")
        })
    };

    let collection_resource = create_local_resource(collection_id, move |id| async move {
        use sqlx::PgPool;
        use crate::models::load_env_vars;

        let (database_url, _, _) = load_env_vars().map_err(|e| e.to_string())?;

        let pool: PgPool = sqlx::PgPool::connect(&database_url)
            .await
            .map_err(|e| e.to_string())?;

        let collection = sqlx::query_as!(
            Collection,
            r#"SELECT 
                id, name, slug, description, is_public, 
                created_at, updated_at 
               FROM collections 
               WHERE id = $1"#,
            id
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(collection)
    });

    view! {
        <div class="container mx-auto px-4 py-8">
            <Suspense fallback=|| view! { <p>"Loading collection..."</p> }>
                {move || {
                    collection_resource.get()
                        .map(|result| match result {
                            Some(Err(e)) => view! { <pre>"Error loading collection: " {e}</pre> }.into_view(),
                            Some(Ok(None)) => view! { <p>"Collection not found"</p> }.into_view(),
                            Some(Ok(Some(collection))) => view! {
                                <div>
                                    <h1 class="text-3xl font-bold mb-4">{&collection.name}</h1>
                                    <p class="text-gray-600 mb-6">{&collection.description.unwrap_or_default()}</p>
                                    
                                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                        // 这里可以显示该集合中的书签
                                    </div>
                                </div>
                            }.into_view(),
                            None => view! { <p>"Loading..."</p> }.into_view(),
                        })
                }}
            </Suspense>
        </div>
    }
}