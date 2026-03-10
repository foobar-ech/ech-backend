use std::collections::HashMap;

use serde::Serialize;

use crate::row::board::BoardRow;

#[derive(Serialize)]
pub(crate) struct BoardDto {
    id: String,
    name: String,
    info: String,
    info_outer: String,
    default_name: String,
    reactions: Vec<String>,
    category: String,
    file_types: Vec<String>,
    enable_dices: bool,
    enable_flags: bool,
    enable_icons: bool,
    enable_likes: bool,
    enable_reactions: bool,
    enable_names: bool,
    enable_oekaki: bool,
    enable_posting: bool,
    enable_sage: bool,
    enable_shield: bool,
    enable_subject: bool,
    enable_thread_tags: bool,
    enable_trips: bool,
    enable_op_mod: bool,
    bump_limit: i32,
    last_num: i32,
    max_comment: i32,
    max_files_size: i32,
    max_pages: i32,
    speed: i32,
    threads: i32,
    threads_per_page: i32,
    unique_posters: i32,
    posts_count: i32,
}

impl TryFrom<(BoardRow, &HashMap<String, i32>)> for BoardDto {
    type Error = &'static str;

    fn try_from(
        (row, posts_count): (BoardRow, &HashMap<String, i32>),
    ) -> Result<Self, Self::Error> {
        let categories = [
            "Разное",
            "Политика",
            "Тематика",
            "Творчество",
            "Техника и софт",
            "Игры",
            "Японская культура",
            "Взрослым",
            "Юзерборды",
        ];

        let reactions = row
            .reactions
            .lines()
            .filter_map(|x| x.is_empty().then_some(x.to_string()))
            .collect();
        let category = categories
            .get(row.category_id as usize)
            .ok_or("invalid category_id")?
            .to_string();
        let file_types = row.file_types.split(",").map(str::to_string).collect();
        let posts_count = posts_count.get(&row.id).cloned().unwrap_or(0);

        Ok(Self {
            id: row.id,
            name: row.name,
            info: row.info,
            info_outer: row.info_outer,
            default_name: row.default_name,
            reactions,
            category,
            file_types,
            enable_dices: row.enable_dices,
            enable_flags: row.enable_flags,
            enable_icons: row.enable_icons,
            enable_likes: row.enable_likes,
            enable_reactions: row.enable_reactions,
            enable_names: row.enable_names,
            enable_oekaki: row.enable_oekaki,
            enable_posting: row.enable_posting,
            enable_sage: row.enable_sage,
            enable_shield: row.enable_shield,
            enable_subject: row.enable_subject,
            enable_thread_tags: row.enable_thread_tags,
            enable_trips: row.enable_trips,
            enable_op_mod: row.enable_op_mod,
            bump_limit: row.bump_limit,
            last_num: row.last_num,
            max_comment: row.max_comment,
            max_files_size: row.max_files_size,
            max_pages: row.max_pages,
            speed: row.speed,
            threads: row.threads,
            threads_per_page: row.threads_per_page,
            unique_posters: row.unique_posters,
            posts_count,
        })
    }
}
