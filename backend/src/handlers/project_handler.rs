use crate::models::projects::{PostProject, ProjectDetail};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use uuid::Uuid;
pub async fn get_all_projects(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<ProjectDetail>>, axum::http::StatusCode> {
    let projects: Vec<ProjectDetail> = sqlx::query_as!(
        ProjectDetail,
        r#"
    SELECT 
        p.id,
        p.slug,
        p.title,
        p.short_description,
        p.cover_image_url,
        p.repo_url,
        p.live_url,
        p.tech_stack,
        pm.content_md
        FROM projects p
        JOIN project_markdown pm
            ON pm.project_id = p.id
        WHERE p.is_published = true
        ORDER BY p.created_at DESC;
     "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    return Ok(Json(projects));
}
pub async fn insert_project(
    State(pool): State<PgPool>,
    Json(project): Json<PostProject>,
) -> Result<Json<Uuid>, StatusCode> {
    let PostProject {
        slug,
        title,
        short_description,
        cover_image_url,
        repo_url,
        live_url,
        tech_stack,
        content_md,
    } = PostProject {
        slug: project.slug,
        title: project.title,
        short_description: project.short_description,
        cover_image_url: project.cover_image_url,
        repo_url: project.repo_url,
        live_url: project.live_url,
        tech_stack: project.tech_stack,
        content_md: project.content_md,
    };
    let project_id: Uuid = sqlx::query_scalar!(
        r#"
        INSERT INTO projects(
        id,slug,title,short_description,cover_image_url,repo_url,
        live_url,tech_stack,is_published,created_at
        ) VALUES (
         gen_random_uuid(),$1,$2,$3,$4,$5,$6,$7,true,NOW()
        )
        RETURNING id
    "#,
        slug,
        title,
        short_description,
        cover_image_url,
        repo_url,
        live_url,
        tech_stack.as_deref()
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query!(
        r#"
        INSERT INTO project_markdown (project_id, content_md)
        VALUES ($1, $2)
        "#,
        project_id,
        content_md
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(project_id))
}
