pub struct ExceptionResponse(pub u16);
pub struct Response(pub u16);

pub fn process_request() -> Result<Response, ExceptionResponse> {
    Ok(Response(1))
}