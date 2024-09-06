use std::collections::HashMap;

use crate::{
    optional_insert,
    request::API_URL_BASE,
    structures::search_structs::{SearchOptions, SearchResponse},
    Furse, Result,
};

impl Furse {
    pub async fn search_mods(&self, opts: SearchOptions) -> Result<SearchResponse> {
        let mut query = HashMap::new();

        query.insert("gameId", opts.game_id.to_string());

        optional_insert!(query += "classId" => opts.class_id into !String);
        optional_insert!(query += "categoryId" => opts.category_id into !String);
        optional_insert!(query += "categoryIds" => opts.category_ids [vec + ","] into !String);
        optional_insert!(query += "gameVersion" => opts.game_version);
        optional_insert!(query += "gameVersions" => opts.game_versions [vec + ","]);
        optional_insert!(query += "searchFilter" => opts.search_filter);
        optional_insert!(query += "sortField" => opts.sort_field as u8 => into !String);
        optional_insert!(query += "sortOrder" => opts.sort_order as u8 => into !String);
        optional_insert!(query += "modLoaderType" => opts.mod_loader_type as u8 => into !String);
        optional_insert!(query += "modLoaderTypes" => opts.mod_loader_types [vec + ","] as u8 => into !String);
        optional_insert!(query += "gameVersionTypeId" => opts.game_version_type_id into !String);
        optional_insert!(query += "authorId" => opts.author_id into !String);
        optional_insert!(query += "primaryAuthorId" => opts.primary_author_id into !String);
        optional_insert!(query += "slug" => opts.slug);
        optional_insert!(query += "index" => opts.index into !String);
        optional_insert!(query += "pageSize" => opts.page_size into !String);

        Ok(self
            .get_with_query(API_URL_BASE.join("mods/search")?, query)
            .await?
            .data)
    }
}
