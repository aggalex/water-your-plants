use crate::business::manager::ErrorResponse;


pub struct PaginationDto {
    pub page: i64,
    pub page_size: i64,
}

impl PaginationDto {
    pub fn new_limited(page: i64, page_size: Option<i64>) -> Result<PaginationDto, ErrorResponse> {
        let page_size = page_size.unwrap_or(10);

        if page_size > 30 {
            return Err(ErrorResponse::BadRequest("page size too big".to_string()));
        }

        Ok(PaginationDto { page, page_size })
    }

    pub fn offset(&self) -> i64 {
        self.page * self.page_size
    }
}
