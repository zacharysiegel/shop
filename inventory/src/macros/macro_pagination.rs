#[macro_export]
macro_rules! has_edge_records {
    (
        $pgpool:expr,
        $asc_previous_query:expr,
        $desc_previous_query:expr,
        $current_page:expr,
        $sort_order:expr,
        $page_size:expr$(,)?) => {
        {
            let (has_previous_record, has_additional_record) = match $sort_order {
                $crate::pagination::SortOrder::Ascending => {
                    let has_lesser_records: bool = match $asc_previous_query.fetch_one($pgpool).await {
                        ::std::result::Result::Ok(_) => true,
                        ::std::result::Result::Err(_) => false,
                    };
                    let has_greater_records: bool = {
                        match $current_page.get($page_size as usize) {
                            ::std::option::Option::Some(_) => true,
                            ::std::option::Option::None => false,
                        }
                    };
                    (has_lesser_records, has_greater_records)
                }
                $crate::pagination::SortOrder::Descending => {
                    let has_lesser_records: bool = {
                        match $current_page.get($page_size as usize) {
                            ::std::option::Option::Some(_) => true,
                            ::std::option::Option::None => false,
                        }
                    };
                    let has_greater_records: bool = match $desc_previous_query.fetch_one($pgpool).await {
                        ::std::result::Result::Ok(_) => true,
                        ::std::result::Result::Err(_) => false,
                    };
                    (has_greater_records, has_lesser_records)
                }
            };
            (has_previous_record, has_additional_record)
        }
    };
}
